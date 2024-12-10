mod controller;
mod prover;
mod scan;
mod task;
pub use controller::{Controller, MainController};
pub use prover::Prover;
pub use scan::ScanBlock;
pub use task::Task;

use anyhow::{anyhow, Result};
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use std::{fs, path::PathBuf};

pub type BaseTableDefinition<'a> = TableDefinition<'a, &'a [u8], Vec<u8>>;

pub trait KvTable: Sized {
    fn table<'a>() -> BaseTableDefinition<'a>;

    fn key(&self) -> Vec<u8>;

    fn to_value(&self) -> Vec<u8>;

    fn from_value(key: &[u8], value: &[u8]) -> Option<Self>;
}

pub struct ReDB {
    db: Database,
}

impl ReDB {
    pub fn new(db_path: &PathBuf, remove: bool) -> Result<Self> {
        let path = db_path.as_path();

        if remove {
            if path.exists() {
                fs::remove_dir_all(path)?;
            }
        }
        fs::create_dir_all(path).map_err(|e| anyhow!("Path {:?}: {}", path, e))?;

        let db_path = path.join("db.redb");
        let db = Database::create(db_path).map_err(|e| anyhow!("Path {:?}: {}", path, e))?;

        // init all tables
        let txn = db.begin_write()?;
        {
            let _ = txn.open_table(Controller::table());
            let _ = txn.open_table(MainController::table());
            let _ = txn.open_table(Prover::table());
            let _ = txn.open_table(Task::table());
        }
        txn.commit()?;

        Ok(Self { db })
    }

    pub fn add<T: KvTable>(&self, t: &T) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(T::table())?;
            table.insert(t.key().as_slice(), t.to_value())?;
        }
        txn.commit()?;

        Ok(())
    }

    pub fn get<T: KvTable>(&self, key: &[u8]) -> Result<Option<T>> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(T::table())?;
        let res = table.get(key)?.and_then(|v| T::from_value(key, &v.value()));

        Ok(res)
    }

    pub fn contains<T: KvTable>(&self, key: &[u8]) -> Result<bool> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(T::table())?;
        let res = table.get(key)?.is_some();

        Ok(res)
    }

    pub fn remove<T: KvTable>(&self, key: &[u8]) -> Result<Option<T>> {
        let txn = self.db.begin_write()?;
        let res = {
            let mut table = txn.open_table(T::table())?;
            let res = table
                .remove(key)?
                .and_then(|v| T::from_value(key, &v.value()));
            res
        };
        txn.commit()?;
        Ok(res)
    }

    pub fn list<T: KvTable>(&self, from: usize, size: usize) -> Result<(Vec<T>, usize)> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(T::table())?;

        let mut items = vec![];
        let total = table.len()? as usize;
        for item in table.iter()?.skip(from).take(size) {
            if let Ok((k, v)) = item {
                if let Some(t) = T::from_value(k.value(), &v.value()) {
                    items.push(t)
                }
            }
        }

        Ok((items, total))
    }

    pub fn count<T: KvTable>(&self) -> Result<usize> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(T::table())?;
        Ok(table.len()? as usize)
    }
}
