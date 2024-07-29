use crate::{ControllerKey, ControllerValue, CONTROLLER_SET, CONTROLLER_SET_KEY, CONTROLLER_TABLE, DOCKER_TABLE, DockerValue, DockerImageMeta};
use anyhow::{anyhow, Result};
use ethers::core::k256::ecdsa::SigningKey;
use redb::{Database, ReadableTable, ReadableTableMetadata};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub struct ReDB {
    db: Arc<Database>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ControllerList {
    pub data: Vec<ControllerKey>,
    pub total: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DockerContainerList {
    pub data: Vec<String>,
    pub total: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DockerImage {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DockerImageList {
    pub data: Vec<DockerImage>,
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

impl ReDB {
    pub async fn docker_add(
        &self,
        miner: &ControllerKey,
        image_id: &str,
        image_name: &str,
        repository: &str,
        tag: &str,
        container_id: Option<&str>,
    ) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(DOCKER_TABLE)?;
            let mut dv = if let Some(map) = table.get(miner)? {
                map.value()
            } else {
                DockerValue::default()
            };


            if let Some(id) = container_id {
                let mut exist = false;

                if let Some(containers) = dv.containers.get_mut(image_id) {
                    containers.push(id.to_string());
                    exist = true;
                }

                if !exist {
                    dv.containers.insert(image_id.to_string(), vec![id.to_string()]);
                }
            }

            dv.ids.insert(image_id.to_string(), DockerImageMeta{
                repository: repository.to_string(),
                tag: tag.to_string(),
                name: image_name.to_string(),
            });

            table.insert(miner, dv)?;
        }
        txn.commit()?;

        Ok(())

    }

    pub async fn docker_delete(
        &self,
        miner: &ControllerKey,
        image_id: &str,
    ) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(DOCKER_TABLE)?;

            let mut dv = if let Some(dv) = table.get(miner)? {
                dv.value()
            } else {
                return Err(anyhow!("miner: {miner:?} not exist repository map"));
            };

            dv.ids.remove(image_id);
            dv.containers.remove(image_id);

            table.insert(miner, dv)?;
        }
        txn.commit()?;

        Ok(())
    }

    pub async fn docker_container_list(
        &self,
        miner: &ControllerKey,
        image_id: &str,
        from: usize,
        size: usize,
    ) -> Result<DockerContainerList> {
        let txn = self.db.begin_read()?;
        let mut table = txn.open_table(DOCKER_TABLE)?;
        let mut dv = if let Some(dv) = table.get(miner)? {
            dv.value()
        } else {
            return Err(anyhow!("miner: {miner:?} not exist repository map"));
        };
        let Some(list) = dv.containers.get(image_id) else {
            return Err(anyhow!("repository: {image_id} not exist container"));
        };

        let total = list.len();

        let data = list.iter().skip(from).take(size).map(|v|v.clone()).collect::<Vec<_>>();

        Ok(DockerContainerList{ data, total })
    }

    pub async fn docker_image_list(
        &self,
        miner: &ControllerKey,
        from: usize,
        size: usize,
    ) -> Result<DockerImageList> {
        let txn = self.db.begin_read()?;
        let mut table = txn.open_table(DOCKER_TABLE)?;
        let mut dv = if let Some(dv) = table.get(miner)? {
            dv.value()
        } else {
            return Err(anyhow!("miner: {miner:?} not exist repository map"));
        };

        let total = dv.ids.len();

        let data = dv.ids.iter().skip(from).take(size).map(|(id,meta)|DockerImage{
            id: id.clone(),
            name: meta.name.clone(),
        }).collect::<Vec<_>>();

        Ok(DockerImageList{ data, total })

    }
}
