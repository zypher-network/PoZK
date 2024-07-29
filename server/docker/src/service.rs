use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use poem_openapi::Object;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shiplift::builder::{ContainerListOptionsBuilder, ImageListOptionsBuilder};
use shiplift::rep::{Container, ContainerCreateInfo, ContainerDetails, Image, ImageDetails};
use shiplift::{
    ContainerOptions, Docker, Error, ImageListOptions, PullOptions, RmContainerOptions,
};
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct DockerManager {
    docker: Docker,
    req_client: Client,
}


#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ContainerNewOption {
    pub cpu_shares: Option<u32>,
    pub cpus: Option<u64>,
    pub env: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
    pub expose: Option<Vec<Expose>>,
    pub memory: Option<u64>,
    pub volumes: Option<Vec<Volumes>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ImageInfo {
    pub repository: String,
    pub created: String,
    pub id: String,
    pub tag: String,
    pub container_list: Vec<ContainerInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ContainerInfo {
    pub id: String,
    pub created: String,
    pub status: String,
    pub image: String,
    pub running: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct Expose {
    /// container
    pub src_port: u32,
    pub protocol: String,
    /// host
    pub host_port: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct Volumes {
    pub src_volumes: String,
    pub host_volumes: String,
}

fn convert_to_vec_of_strs<'a>(vec: &'a Vec<String>) -> Vec<&'a str> {
    vec.iter().map(|s| s.as_str()).collect()
}

impl DockerManager {
    pub fn new() -> Result<Self> {
        let docker = Docker::new();
        let client = Client::new();
        Ok(Self {
            docker,
            req_client: client,
        })
    }

    pub async fn pull_image(&self, repository: &str, tag: &str) -> Result<()> {
        let repo_tag = format!("{repository}:{tag}");
        let pull_options = PullOptions::builder().image(&repo_tag).build();
        let mut pull_stream = self.docker.images().pull(&pull_options);
        while let Some(pull_result) = pull_stream.next().await {
            match pull_result {
                Ok(v) => {
                    log::info!("pull image: {v:?}");
                }
                Err(e) => {
                    log::error!("pull image: {e:?}");
                    return Err(anyhow!("pull image repo_tag:{repo_tag}, err: {e:?}"));
                }
            }
        }


        Ok(())
    }

    pub async fn new_container(
        &self,
        image: &str,
        tag: &str,
        option: &ContainerNewOption,
    ) -> Result<ContainerCreateInfo> {
        if !self.image_exist(image, tag).await? {
            self.pull_image(image, tag).await?;
        }

        let repo_tag = format!("{image}:{tag}");
        let name = format!("minner-{}-{tag}", image.replace("/", "-"));

        let mut container_options_builder = ContainerOptions::builder(&repo_tag);
        let mut container_options_builder_mut = container_options_builder.name(&name);

        if let Some(memory) = option.memory {
            container_options_builder_mut = container_options_builder_mut.memory(memory);
        };

        if let Some(cpu) = option.cpus {
            container_options_builder_mut = container_options_builder_mut.cpus(cpu as f64);
        }

        if let Some(list) = &option.expose {
            for e in list {
                container_options_builder_mut =
                    container_options_builder_mut.expose(e.src_port, &e.protocol, e.host_port);
            }
        }

        if let Some(cpu_shares) = option.cpu_shares {
            container_options_builder_mut = container_options_builder_mut.cpu_shares(cpu_shares);
        }

        if let Some(env) = &option.env {
            container_options_builder_mut = container_options_builder_mut.env(env);
        }

        if let Some(volumes) = &option.volumes {
            println!("volumes: {volumes:?}");

            let volumes = volumes
                .into_iter()
                .map(|v| format!("{}:{}", v.host_volumes, v.src_volumes))
                .collect::<Vec<_>>();
            let volumes = convert_to_vec_of_strs(&volumes);
            container_options_builder_mut = container_options_builder_mut.volumes(volumes);
        }

        if let Some(cmd) = &option.cmd {
            let cmd = convert_to_vec_of_strs(&cmd);
            container_options_builder_mut = container_options_builder_mut.cmd(cmd);
        }

        let container_options = container_options_builder_mut.build();

        log::debug!("container_options: {container_options:?}");

        let container = self
            .docker
            .containers()
            .create(&container_options)
            .await
            .map_err(|e| {
                match &e {
                    Error::Fault { code, message } => {
                        log::error!("create container: {code}, {message}");
                    }
                    _ => {}
                }
                e
            })?;

        Ok(container)
    }

    pub async fn remove_container(&self, id: &str) -> Result<()> {
        let container = self.docker.containers().get(id);
        container
            .remove(RmContainerOptions::default())
            .await
            .map_err(|e| anyhow!("remove container : {e:?}"))
    }

    pub async fn start_container(&self, id: &str) -> Result<()> {
        let container = self.docker.containers().get(id);
        container
            .start()
            .await
            .map_err(|e| anyhow!("start container : {e:?}"))
    }

    pub async fn stop_container(&self, id: &str) -> Result<()> {
        let container = self.docker.containers().get(id);
        container
            .stop(None)
            .await
            .map_err(|e| anyhow!("stop container : {e:?}"))
    }

    pub async fn image_exist(&self, image: &str, tag: &str) -> Result<bool> {
        let repo_tag = format!("{image}:{tag}");
        let images = self
            .docker
            .images()
            .list(&ImageListOptions::default())
            .await?;

        let op = images.iter().find(|image| {
            let op = if let Some(list) = &image.repo_tags {
                list.iter().find(|v| v == &&repo_tag)
            } else {
                None
            };

            if op.is_some() {
                true
            } else {
                false
            }
        });

        if op.is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_image_by_repository(
        &self,
        repository: &str,
    ) -> Result<Option<String>> {
        let op = {
            let mut op = ImageListOptionsBuilder::default();
            op.all();
            op
        };

        let images = self.docker.images().list(&op.build()).await?;

        for image in images {
            let Some(repo_tags) = image.repo_tags else {
                continue;
            };

            let Some(repo_tag) = repo_tags.get(0) else {
                continue;
            };

            let split = repo_tag.split(":").collect::<Vec<_>>();
            let Some(repo) = split.get(0) else {
                continue;
            };

            if repo.eq(&repository) {
                let split = image.id.split(":").collect::<Vec<_>>();;
                let id = split.get(1).unwrap().to_string();
                return Ok(Some(id))
            }
        }

        Ok(None)
    }

    pub async fn prover_image_list(
        &self,
        image_id: &str,
        container_ids: Vec<String>,
    ) -> Result<ImageInfo> {
        let image = self.docker.images().get(image_id);
        let image_detail = image.inspect().await?;

        let s = image_detail.repo_tags
            .unwrap_or_default().get(0)
            .unwrap_or(&"".to_string()).clone();

        let split = s.split(":").collect::<Vec<_>>();
        let repository = split.get(0).unwrap_or(&"").to_string();
        let tag = split.get(1).unwrap_or(&"").to_string();

        let mut image_info = ImageInfo{
            repository,
            created: image_detail.created.to_rfc3339(),
            id: image_detail.id,
            tag,
            container_list: vec![],
        };

        for container_id in container_ids {
            let container = self.docker.containers().get(container_id);
            let container_detail = container.inspect().await?;

            let container_info = ContainerInfo{
                id: container_detail.id,
                created: container_detail.created.to_rfc3339(),
                status: container_detail.state.status,
                image: container_detail.image,
                running: container_detail.state.running,
            };

            image_info.container_list.push(container_info);
        }

        Ok(image_info)
    }
}

#[cfg(test)]
mod test {
    use crate::service::Volumes;
    use crate::{ContainerNewOption, DockerManager, Expose};
    use std::time::Duration;

    #[test]
    fn test_image_list() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let dm = DockerManager::new().unwrap();
            let list = dm.image_list(0, 10).await.unwrap();
            let json = serde_json::to_string_pretty(&list).unwrap();
            println!("image list: {json}");
        });
    }

    #[test]
    fn test_container_list() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let dm = DockerManager::new().unwrap();
            let list = dm.container_list(0, 10).await.unwrap();
            let json = serde_json::to_string_pretty(&list).unwrap();
            println!("container list: {json}");
        });
    }

    #[test]
    fn test_run_postgres() {
        env_logger::init();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {

            let mut dm = DockerManager::new().unwrap();


            // image, tag, op
            let (image, tag, op) = {
                let repo = "postgres";
                let tag = "latest";
                let env = {
                    let mut list = vec![];
                    list.push("POSTGRES_PASSWORD=1".to_string());
                    list
                };
                let op = ContainerNewOption {
                    cpu_shares: None,
                    cpus: None,
                    env: Some(env),
                    cmd: None,
                    expose: Some(vec![Expose {
                        src_port: 5432,
                        protocol: "tcp".to_string(),
                        host_port: 5432,
                    }]),
                    memory: None,
                    volumes: None,
                };

                (repo, tag, op)
            };

            // 2. pull image
            dm.pull_images(image, tag).await.unwrap();

            // 3. new postgres
            let cc_info = dm.new_container(image, tag, &op).await.unwrap();
            println!("container create info: {cc_info:?}");

            // 4. start
            dm.start_container(&cc_info.id).await.unwrap();
            let container_info = dm.container_info(&cc_info.id).await.unwrap();
            println!("start container_info: {:?}", container_info);

            // 5. stop
            dm.stop_container(&cc_info.id).await.unwrap();
            let container_info = dm.container_info(&cc_info.id).await.unwrap();
            println!("stop container_info: {:?}", container_info);

            // 6. remove
            dm.remove_container(&cc_info.id).await.unwrap();
            let container_list = dm.container_list(0, 10).await.unwrap();
            println!("container list: {:?}", container_list);
        })
    }

    /*
    挂载会有个问题, 宿主机上文件或者文件夹最好给权限777
    ```
        sudo chmod -R 777 $HOME/.celestia-light-mocha-4/
    ```
    否则创建出来的容器会报错权限问题,
    这个shiplift控制docker创建出来的目录是root的,但是celestia没有权限操作root目录
    */
    #[test]
    fn test_run_celestia() {
        env_logger::init();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {

            tokio::time::sleep(Duration::from_secs(5)).await;

            let mut dm = DockerManager::new().unwrap();


            // image, tag, op
            let (image, tag, op) = {
                let repo = "ghcr.io/rollkit/celestia-da";
                let tag = "v0.12.10";

                let cmd = vec![
                    "celestia-da".to_string(),
                    "light".to_string(),
                    "start".to_string(),
                    "--p2p.network=mocha".to_string(),
                    "--da.grpc.namespace=000000506f6c61726973".to_string(),
                    "--da.grpc.listen=0.0.0.0:26650".to_string(),
                    "--core.ip=rpc-mocha.pops.one".to_string(),
                    "--gateway".to_string(),
                ];

                let env = vec![
                    "NODE_TYPE=light".to_string(),
                    "P2P_NETWORK=mocha".to_string(),
                ];

                let op = ContainerNewOption {
                    cpu_shares: None,
                    cpus: None,
                    env: Some(env),
                    cmd: Some(cmd),
                    expose: Some(vec![
                        Expose {
                            src_port: 26650,
                            protocol: "tcp".to_string(),
                            host_port: 26650,
                        },
                        Expose {
                            src_port: 26658,
                            protocol: "tcp".to_string(),
                            host_port: 26658,
                        },
                        Expose {
                            src_port: 26659,
                            protocol: "tcp".to_string(),
                            host_port: 26659,
                        },
                    ]),
                    memory: None,
                    // volumes: None,
                    volumes: Some(vec![Volumes {
                        src_volumes: "/home/celestia/.celestia-light-mocha-4".to_string(),
                        host_volumes: format!(
                            "{}/.celestia-light-mocha-4",
                            std::env::var("HOME").unwrap()
                        ),
                    }]),
                };

                (repo, tag, op)
            };

            // 2. pull image
            dm.pull_images(image, tag).await.unwrap();

            // 3. new postgres
            let cc_info = dm.new_container(image, tag, &op).await.unwrap();
            println!("container create info: {cc_info:?}");

            // 4. start
            dm.start_container(&cc_info.id).await.unwrap();
            let container_info = dm.container_info(&cc_info.id).await.unwrap();
            println!("start container_info: {:?}", container_info);

            // 5. stop
            // dm.stop_container(&cc_info.id).await.unwrap();
            // let container_info = dm.container_info(&cc_info.id).await.unwrap();
            // println!("stop container_info: {:?}", container_info);
            //
            // // 6. remove
            // dm.remove_container(&cc_info.id).await.unwrap();
            // let container_list = dm.container_list(0,10).await.unwrap();
            // println!("container list: {:?}", container_list);
        });
    }

}
