use crate::{ControllerKey, ControllerValue, CONTROLLER_SET, CONTROLLER_SET_KEY, CONTROLLER_TABLE};
use anyhow::{anyhow, Result};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::types::Address;
use redb::{Database, ReadableTable, ReadableTableMetadata};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ReDB {
    db: Arc<Database>,
}

pub struct ControllerList {
    pub data: Vec<ControllerKey>,
    pub total: usize,
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

    pub async fn controller_add(
        &self,
        miner: &ControllerKey,
        controller: &ControllerKey,
        value: &SigningKey,
    ) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(CONTROLLER_TABLE)?;
            let mut map = if let Some(map) = table.get(miner)? {
                map.value()
            } else {
                ControllerValue(BTreeMap::new())
            };

            map.0.insert(controller.clone(), value.to_bytes().to_vec());

            table.insert(miner, map)?;
        }
        txn.commit()?;
        Ok(())
    }

    pub async fn controller_set(
        &self,
        miner: &ControllerKey,
        controller: &ControllerKey,
    ) -> Result<()> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_TABLE)?;

        let Some(controller_map) = table.get(miner)? else {
            return Err(anyhow!("miner not exist controllers: {miner:?}"));
        };

        if controller_map.value().0.get(controller).is_none() {
            return Err(anyhow!(
                "controller: {controller:?} not exist miner controllers"
            ));
        }

        let txn = self.db.begin_write()?;

        {
            let mut table = txn.open_table(CONTROLLER_SET)?;

            table.insert(miner, controller)?;
        }

        txn.commit()?;

        Ok(())
    }

    pub async fn query_controller_set(&self, miner: &ControllerKey) -> Result<ControllerKey> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_SET)?;
        let Some(controller) = table.get(miner)? else {
            return Err(anyhow!("miner:{miner:?} not exist controllers"));
        };

        Ok(controller.value())
    }

    pub async fn controller_list(
        &self,
        miner: &ControllerKey,
        from: usize,
        size: usize,
    ) -> Result<ControllerList> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_TABLE)?;
        let Some(controllers) = table.get(miner)? else {
            return Err(anyhow!("miner: {miner:?} not exits controllers"));
        };

        let total = controllers.value().0.len();

        let map = controllers.value().0;
        let mut iter = map.iter().skip(from).take(size);
        let mut list = vec![];

        while let Some((key, _val)) = iter.next() {
            list.push(key.clone());
        }

        Ok(ControllerList { data: list, total })
    }

    pub async fn controller_set_entry(
        &self,
        miner: &ControllerKey,
    ) -> Result<(ControllerKey, SigningKey)> {
        let controller = self.query_controller_set(miner).await?;
        let txn = self.db.begin_read()?;

        let table = txn.open_table(CONTROLLER_TABLE)?;
        let Some(controllers) = table.get(miner)? else {
            return Err(anyhow!("miner: {miner:?} not exist controllers"));
        };

        let signing_key = if let Some(val) = controllers.value().0.get(&controller) {
            SigningKey::from_slice(val)?
        } else {
            return Err(anyhow!("set key: {:?} not match val", controller));
        };

        Ok((controller, signing_key))
    }

    pub async fn controller_export(
        &self,
        miner: &ControllerKey,
        controller: &ControllerKey,
    ) -> Result<SigningKey> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_TABLE)?;
        let Some(controllers) = table.get(miner)? else {
            return Err(anyhow!("miner: {miner:?} not exist controllers"));
        };

        let signing_key = if let Some(val) = controllers.value().0.get(&controller) {
            SigningKey::from_slice(val)?
        } else {
            return Err(anyhow!("set key: {:?} not match val", controller));
        };

        Ok(signing_key)
    }
}
