use crate::{
    ControllerKey, ControllerValue, DockerImageMeta, DockerValue, CONTROLLER_SET, CONTROLLER_TABLE,
    DOCKER_TABLE,
};
use anyhow::{anyhow, Result};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::types::Address;
use redb::{Database, ReadableTable};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

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
    pub meta: DockerImageMeta,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DockerImage {
    pub image_id: String,
    pub prover: String,
    pub name: String,
    pub created: String,
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

    pub fn controller_add(
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

    pub fn controller_set(&self, miner: &ControllerKey, controller: &ControllerKey) -> Result<()> {
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

    pub fn query_controller_set(&self, miner: &ControllerKey) -> Result<ControllerKey> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_SET)?;
        let Some(controller) = table.get(miner)? else {
            return Err(anyhow!("miner:{miner:?} not exist controllers"));
        };

        Ok(controller.value())
    }

    pub fn controller_list(
        &self,
        miner: &ControllerKey,
        from: usize,
        size: usize,
    ) -> Result<Option<ControllerList>> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONTROLLER_TABLE)?;
        let Some(controllers) = table.get(miner)? else {
            log::warn!("miner: {miner:?} not exits controllers");
            return Ok(None);
        };

        let total = controllers.value().0.len();

        let map = controllers.value().0;
        let mut iter = map.iter().skip(from).take(size);
        let mut list = vec![];

        while let Some((key, _val)) = iter.next() {
            list.push(key.clone());
        }

        Ok(Some(ControllerList { data: list, total }))
    }

    pub fn controller_set_entry(
        &self,
        miner: &ControllerKey,
    ) -> Result<(ControllerKey, SigningKey)> {
        let controller = self.query_controller_set(miner)?;
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

    pub fn controller_export(
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
    pub fn prover_add(
        &self,
        miner: &ControllerKey,
        image_id: &str,
        image_name: &str,
        repository: &str,
        prover: &Address,
        image_created: &str,
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

                if let Some(containers) = dv.containers.get_mut(prover) {
                    containers.push(id.to_string());
                    exist = true;
                }

                if !exist {
                    let mut exist = false;

                    if let Some(containers) = dv.containers.get_mut(prover) {
                        containers.push(id.to_string());
                        exist = true;
                    }

                    if !exist {
                        dv.containers.insert(prover.clone(), vec![id.to_string()]);
                    }
                }
            }

            dv.ids.insert(
                prover.clone(),
                DockerImageMeta {
                    repository: repository.to_string(),
                    image_id: image_id.to_string(),
                    tag: tag.to_string(),
                    name: image_name.to_string(),
                    created: image_created.to_string(),
                },
            );

            table.insert(miner, dv)?;
        }
        txn.commit()?;

        Ok(())
    }

    pub fn prover_container_add(
        &self,
        miner: &ControllerKey,
        prover: &Address,
        container_id: &str,
    ) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(DOCKER_TABLE)?;
            let mut dv = if let Some(dv) = table.get(miner)? {
                dv.value()
            } else {
                return Err(anyhow!("miner: {miner:?} not exist repository map"));
            };

            let mut exist = false;

            if let Some(containers) = dv.containers.get_mut(&prover) {
                containers.push(container_id.to_string());
                exist = true;
            }

            if !exist {
                dv.containers
                    .insert(prover.clone(), vec![container_id.to_string()]);
            }

            table.insert(miner, dv)?;
        }
        txn.commit()?;
        Ok(())
    }

    pub fn prover_container_remove(
        &self,
        miner: &ControllerKey,
        prover: &Address,
        container_id: &str,
    ) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(DOCKER_TABLE)?;
            let mut dv = if let Some(dv) = table.get(miner)? {
                dv.value()
            } else {
                return Err(anyhow!("miner: {miner:?} not exist repository map"));
            };

            let mut index = None;
            if let Some(containers) = dv.containers.get(prover) {
                for (idx, id) in containers.iter().enumerate() {
                    if id.eq(container_id) {
                        index.replace(idx);
                        break;
                    }
                }
            }

            if let Some(idx) = index {
                let containers = dv.containers.get_mut(prover).unwrap(); // safe
                containers.remove(idx);
            }

            table.insert(miner, dv)?;
        }
        txn.commit()?;
        Ok(())
    }

    pub fn prover_meta(
        &self,
        miner: &ControllerKey,
        prover: &Address,
    ) -> Result<Option<DockerImageMeta>> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(DOCKER_TABLE)?;
        let dv = if let Some(dv) = table.get(miner)? {
            dv.value()
        } else {
            log::warn!("miner: {miner:?} not exist repository map");
            return Ok(None);
        };

        return if let Some(meta) = dv.ids.get(prover) {
            Ok(Some(meta.clone()))
        } else {
            log::warn!("prover: {prover:?} not exist on db");
            Ok(None)
        };
    }

    pub fn prover_image_remove(&self, miner: &ControllerKey, prover: &Address) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(DOCKER_TABLE)?;

            let mut dv = if let Some(dv) = table.get(miner)? {
                dv.value()
            } else {
                return Err(anyhow!("miner: {miner:?} not exist repository map"));
            };

            dv.ids.remove(prover);
            dv.containers.remove(prover);

            table.insert(miner, dv)?;
        }
        txn.commit()?;

        Ok(())
    }

    pub fn docker_container_list(
        &self,
        miner: &ControllerKey,
        prover: &Address,
        from: usize,
        size: usize,
    ) -> Result<Option<DockerContainerList>> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(DOCKER_TABLE)?;
        let dv = if let Some(dv) = table.get(miner)? {
            dv.value()
        } else {
            log::warn!("miner: {miner:?} not exist repository map");
            return Ok(None);
        };

        let Some(list) = dv.containers.get(prover) else {
            log::warn!("prover: {prover:?} not exist container");
            return Ok(None);
        };

        let Some(meta) = dv.ids.get(prover) else {
            log::warn!("prover: {prover:?} not exist meta");
            return Ok(None);
        };

        let total = list.len();

        let data = list
            .iter()
            .skip(from)
            .take(size)
            .map(|v| v.clone())
            .collect::<Vec<_>>();

        Ok(Some(DockerContainerList {
            data,
            total,
            meta: meta.clone(),
        }))
    }

    pub fn docker_image_list(
        &self,
        miner: &ControllerKey,
        from: usize,
        size: usize,
    ) -> Result<Option<DockerImageList>> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(DOCKER_TABLE)?;
        let dv = if let Some(dv) = table.get(miner)? {
            dv.value()
        } else {
            log::warn!("miner: {miner:?} not exist repository map");
            return Ok(None);
        };

        let total = dv.ids.len();

        let data = dv
            .ids
            .iter()
            .skip(from)
            .take(size)
            .map(|(prover, meta)| DockerImage {
                image_id: meta.image_id.to_string(),
                prover: format!("{prover:?}"),
                name: meta.name.clone(),
                created: meta.created.clone(),
            })
            .collect::<Vec<_>>();

        Ok(Some(DockerImageList { data, total }))
    }
}
