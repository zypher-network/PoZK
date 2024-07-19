use crate::{ControllerKey, ControllerValue, CONTROLLER_SET, CONTROLLER_SET_KEY, CONTROLLER_TABLE};
use anyhow::{anyhow, Result};
use redb::{Database, ReadableTable, ReadableTableMetadata};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ReDB {
    db: Arc<Database>,
}


pub struct ControllerList {
    pub data: Vec<ControllerKey>,
    pub total: usize
}

impl ReDB {
    pub fn new(db_path: &PathBuf, remove: bool) -> Result<Self> {
        let path = db_path.as_path();

        if remove {
            if path.exists() {
                fs::remove_dir_all(path)?;
            }
        }

        fs::create_dir_all(path)?;

        let db_path = path.join("db.redb");
        let db = Database::create(db_path)?;

        Ok(Self { db: Arc::new(db) })
    }

    pub async fn controller_add(&self, key: &ControllerKey, value: &ControllerValue) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(CONTROLLER_TABLE)?;
            table.insert(key, value)?;
        }
        txn.commit()?;
        Ok(())
    }

    pub async fn controller_set(&self, key: &ControllerKey) -> Result<()> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_TABLE)?;

        let Some(val) = table.get(key)? else {
            return Err(anyhow!("key not exist: {key:?}"));
        };

        let txn = self.db.begin_write()?;

        {
            let mut table = txn.open_table(CONTROLLER_SET)?;
            table.insert(CONTROLLER_SET_KEY, key)?;
        }

        txn.commit()?;

        Ok(())
    }

    pub async fn query_controller_set(&self) -> Result<ControllerKey> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_SET)?;
        let Some(val) = table.get(CONTROLLER_SET_KEY)? else {
            return Err(anyhow!("not set controller"));
        };

        Ok(val.value())
    }

    pub async fn controller_list(&self, from: usize, size: usize) -> Result<ControllerList> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_TABLE)?;
        let total = table.len()? as usize;

        let mut iter = table.iter()?.skip(from).take(size);
        let mut list = vec![];

        while let Some(res) = iter.next() {
            let (key, _val) = res?;
            list.push(key.value());
        }

        Ok(ControllerList{ data: list, total })
    }

    pub async fn controller_set_entry(&self) -> Result<(ControllerKey, ControllerValue)> {
        let key = self.query_controller_set().await?;
        let txn = self.db.begin_read()?;

        let table = txn.open_table(CONTROLLER_TABLE)?;

        let Some(val) = table.get(&key)? else {
            return Err(anyhow!("set key: {:?} not match val", key.0));
        };

        Ok((key, val.value()))
    }
}
