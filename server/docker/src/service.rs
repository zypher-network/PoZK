use std::collections::HashMap;
use anyhow::{anyhow, Result};
use bollard::container::{Config, CreateContainerOptions, InspectContainerOptions, RemoveContainerOptions, StartContainerOptions, StopContainerOptions};
use bollard::Docker;
use bollard::image::{CreateImageOptions, ListImagesOptions};
use bollard::models::{ContainerInspectResponse, ContainerState, HostConfig, PortBinding, PortMap};
use chrono::{TimeZone, Utc};
use futures_util::StreamExt;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct DockerManager {
    docker: Docker,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ContainerNewOption {
    pub cpu_shares: Option<i64>,
    pub cpus: Option<i64>,
    pub env: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
    pub expose: Option<Vec<Expose>>,
    pub memory: Option<i64>,
    pub volumes: Option<Vec<Volumes>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct ImageInfo {
    pub repository: String,
    pub created: String,
    pub id: String,
    pub tag: String,
    pub container_list: Vec<ContainerInfo>,
    pub total: usize,
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

impl DockerManager {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_socket_defaults()?;
        Ok(Self { docker })
    }

    pub async fn pull_image(&self, repository: &str, tag: &str) -> Result<()> {
        let repo_tag = format!("{repository}:{tag}");
        let pull_options = CreateImageOptions{
            from_image: repo_tag.clone(),
            ..Default::default()
        };
        let mut pull_stream = self.docker.create_image(Some(pull_options), None, None);
        while let Some(pull_result) = pull_stream.next().await {
            match pull_result {
                Ok(v) => {
                    log::info!("pull image: {v:?}");
                }
                Err(e) => {
                    log::error!("pull image: {e:?}");
                    return Err(anyhow!("pull image repo_tag: {repo_tag}, err: {e:?}"));
                }
            }
        }

        Ok(())
    }

    pub async fn remove_image(&self, image_id: &str) -> Result<()> {
        let remove_list = self.docker.remove_image(image_id, None, None).await?;

        log::debug!("remove image: {remove_list:?}");

        Ok(())
    }

    pub async fn new_container(
        &self,
        repo: &str,
        tag: &str,
        task_id: u64,
        option: &ContainerNewOption,
    ) -> Result<ContainerInspectResponse> {
        if !self.image_exist(repo, tag).await? {
            self.pull_image(repo, tag).await?;
        }

        let repo_tag = format!("{repo}:{tag}");
        let name = format!(
            "minner-{}-{tag}-{}",
            repo.replace("/", "-"),
            task_id
        );

        let create_op = CreateContainerOptions{ name, platform: None };
        let mut op = Config::default();
        let mut host_op = HostConfig::default();

        op.image = Some(repo_tag);

        if let Some(memory) = option.memory {
            host_op.memory = Some(memory);
        };

        if let Some(cpu) = option.cpus {
            host_op.cpu_count = Some(cpu);
        }

        if let Some(list) = &option.expose {
            let mut src_port_map: HashMap<String, HashMap<(), ()>> = HashMap::new();
            let mut host_port_map = PortMap::new();

            for e in list {
                src_port_map.insert(e.src_port.to_string(), HashMap::new());
                host_port_map.insert(e.src_port.to_string(), Some(vec![
                    PortBinding{
                        host_ip: Some("127.0.0.1".to_string()),
                        host_port: Some(e.host_port.to_string())
                    },
                ]));
            }

            op.exposed_ports = Some(src_port_map);
            host_op.port_bindings = Some(host_port_map);
        }

        if let Some(cpu_shares) = option.cpu_shares {
            // container_options_builder_mut = container_options_builder_mut.cpu_shares(cpu_shares);
            host_op.cpu_shares = Some(cpu_shares);
        }

        if let Some(env) = &option.env {
            // container_options_builder_mut = container_options_builder_mut.env(env);
            op.env = Some(env.clone());
        }

        if let Some(volumes) = &option.volumes {
            let volumes = volumes
                .into_iter()
                .map(|v| format!("{}:{}", v.host_volumes, v.src_volumes))
                .collect::<Vec<_>>();
            host_op.binds = Some(volumes);
        }

        if let Some(cmd) = &option.cmd {
            op.cmd = Some(cmd.clone());
        }

        op.host_config = Some(host_op);

        let container_create_resp = self.docker.create_container(Some(create_op), op).await?;

        let op = Some(InspectContainerOptions{
            size: false,
        });

        let container_info = self.docker.inspect_container(&container_create_resp.id, op).await?;

        Ok(container_info)
    }

    pub async fn query_container_status(&self, id: &str) -> Result<Option<ContainerState>> {
        let op = Some(InspectContainerOptions{
            size: false,
        });

        let container = self.docker.inspect_container(id, op).await?;
        Ok(container.state)
    }

    pub async fn remove_container(&self, id: &str) -> Result<()> {
        let op = Some(RemoveContainerOptions{
            force: true,
            ..Default::default()
        });

        self.docker.remove_container(id, op).await?;
        Ok(())
    }

    pub async fn start_container(&self, id: &str) -> Result<()> {
        self.docker.start_container(id, None::<StartContainerOptions<String>>).await?;
        Ok(())
    }

    pub async fn stop_container(&self, id: &str) -> Result<()> {

        let op = Some(StopContainerOptions{
            t: 30,
        });

        self.docker.stop_container(id, op).await?;
        Ok(())
    }

    pub async fn image_exist(&self, repo: &str, tag: &str) -> Result<bool> {
        let repo_tag = format!("{repo}:{tag}");

        let mut filters = HashMap::new();
        filters.insert("reference", vec![repo_tag.as_str()]);


        let op = ListImagesOptions {
            all: true,
            filters,
            digests: false,
        };

        let images = self.docker.list_images(Some(op)).await?;

        let op = images.iter().find(|image| {

            let op = image.repo_tags.iter().find(|v| v == &&repo_tag);

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
        repo: &str,
        tag: &str,
    ) -> Result<Option<ImageInfo>> {
        let repo_tag = format!("{repo}:{tag}");

        let mut filters = HashMap::new();
        filters.insert("reference", vec![repo_tag.as_str()]);

        let op = ListImagesOptions {
            all: true,
            filters,
            digests: false,
        };

        let images = self.docker.list_images(Some(op)).await?;

        for image in images {

            let Some(repo_tag) = image.repo_tags.get(0) else {
                continue;
            };

            let repo_tag_split = repo_tag.split(":").collect::<Vec<_>>();

            let Some(repo_tmp) = repo_tag_split.get(0) else {
                continue;
            };

            let Some(tag_tmp) = repo_tag_split.get(1) else {
                continue;
            };

            if repo_tmp.eq(&repo) && tag_tmp.eq(&tag) {
                let split = image.id.split(":").collect::<Vec<_>>();
                let id = split.get(1).unwrap().to_string();
                let created = Utc.timestamp(image.created,0);
                return Ok(Some(ImageInfo {
                    repository: repo.to_string(),
                    created: created.to_rfc3339(),
                    id,
                    tag: tag.to_string(),
                    container_list: vec![],
                    total: 0,
                }));
            }
        }

        Ok(None)
    }

    pub async fn prover_image_list(
        &self,
        image_id: &str,
        container_ids: Vec<String>,
    ) -> Result<ImageInfo> {

        let image = self.docker.inspect_image(image_id).await?;

        let Some(repo_tags) = image.repo_tags else {
            return Err(anyhow!("image: {image_id}, not exist repo tags"));
        };

        let Some(repo_tag) = repo_tags.get(0) else {
            return Err(anyhow!("image: {image_id}, repo tag index 0 not exist"));
        };
        let split = repo_tag.split(":").collect::<Vec<_>>();

        let repo = split.get(0).unwrap_or(&"").to_string();
        let tag = split.get(1).unwrap_or(&"").to_string();

        let Some(created) = image.created else {
            return Err(anyhow!("image: {image_id}, not exist created"));
        };

        let mut image_info = ImageInfo {
            repository: repo,
            created: created.to_rfc3339(),
            id: image_id.to_string(),
            tag,
            container_list: vec![],
            total: 0,
        };

        for container_id in container_ids {
            let container = self.docker.inspect_container(&container_id, None).await?;

            let Some(state) = container.state else {
                return Err(anyhow!("image: {image_id}, not exist state"));
            };

            let Some(status) = state.status else {
                return Err(anyhow!("image: {image_id}, not exist status"));
            };

            let container_info = ContainerInfo {
                id: container.id.unwrap_or_default(),
                created: container.created.unwrap_or_default(),
                status: status.to_string(),
                image: container.image.unwrap_or_default(),
                running: state.running.unwrap_or_default(),
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

    // #[test]
    // fn test_image_list() {
    //     let rt = tokio::runtime::Builder::new_current_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap();
    //     rt.block_on(async {
    //         let dm = DockerManager::new().unwrap();
    //         let list = dm.image_list(0, 10).await.unwrap();
    //         let json = serde_json::to_string_pretty(&list).unwrap();
    //         println!("image list: {json}");
    //     });
    // }

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
            dm.pull_image(image, tag).await.unwrap();

            // 3. new postgres
            let cc_info = dm.new_container(image, tag, 0,&op).await.unwrap();
            println!("container create info: {cc_info:?}");

            // 4. start
            // dm.start_container(&cc_info.id).await.unwrap();
            // let container_info = dm.container_info(&cc_info.id).await.unwrap();
            // println!("start container_info: {:?}", container_info);

            // 5. stop
            // dm.stop_container(&cc_info.id).await.unwrap();
            // let container_info = dm.container_info(&cc_info.id).await.unwrap();
            // println!("stop container_info: {:?}", container_info);

            // 6. remove
            // dm.remove_container(&cc_info.id).await.unwrap();
            // let container_list = dm.container_list(0, 10).await.unwrap();
            // println!("container list: {:?}", container_list);
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
            dm.pull_image(image, tag).await.unwrap();

            // 3. new postgres
            let cc_info = dm.new_container(image, tag,0, &op).await.unwrap();
            println!("container create info: {cc_info:?}");

            // 4. start
            // dm.start_container(&cc_info.id).await.unwrap();
            // let container_info = dm.container_info(&cc_info.id).await.unwrap();
            // println!("start container_info: {:?}", container_info);

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
