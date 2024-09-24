#[macro_use]
extern crate tracing;

use anyhow::{anyhow, Result};
use bollard::{
    container::{
        Config, CreateContainerOptions, InspectContainerOptions, StartContainerOptions,
        StopContainerOptions,
    },
    image::{CreateImageOptions, ListImagesOptions},
    models::{ContainerState, HostConfig},
    Docker,
};
use futures_util::StreamExt;
use pozk_utils::get_task_api;
use std::collections::HashMap;

const DOCKER_ORG: &str = "zyphernetwork";

#[derive(Default)]
pub struct RunOption {
    cpu: Option<i64>,
    memory: Option<i64>,
}

#[derive(Clone)]
pub struct DockerManager {
    docker: Docker,
}

impl DockerManager {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_socket_defaults()?;
        Ok(Self { docker })
    }

    /// pull new prover image
    pub async fn pull(&self, prover: &str, tag: &str) -> Result<String> {
        let repo_tag = format!("{}/{}:{}", DOCKER_ORG, prover, tag);

        // TODO proxy

        let pull_options = CreateImageOptions {
            from_image: repo_tag.clone(),
            ..Default::default()
        };
        let mut pull_stream = self.docker.create_image(Some(pull_options), None, None);
        while let Some(pull_result) = pull_stream.next().await {
            match pull_result {
                Ok(v) => {
                    info!("[Docker] pull image: {v:?}");
                }
                Err(e) => {
                    error!("[Docker] pull image: {e:?}");
                    return Err(anyhow!("pull image repo_tag: {repo_tag}, err: {e:?}"));
                }
            }
        }

        // get image id by repo
        let mut filters = HashMap::new();
        filters.insert("reference", vec![repo_tag.as_str()]);

        let op = ListImagesOptions {
            all: true,
            filters,
            digests: false,
        };

        let images = self.docker.list_images(Some(op)).await?;

        for image in images {
            let Some(i_repo_tag) = image.repo_tags.get(0) else {
                continue;
            };

            if i_repo_tag == &repo_tag {
                let split = image.id.split(":").collect::<Vec<_>>();
                let id = split.get(1).unwrap().to_string();
                return Ok(id);
            }
        }

        Err(anyhow!("Missing docker image"))
    }

    /// start a container to run the zkp
    pub async fn run(&self, image: &str, tid: u64, roption: RunOption) -> Result<String> {
        let name = format!("{}-{}", image, tid);
        let input_env = format!("INPUT={}", get_task_api(tid));

        // create op
        let create_op = CreateContainerOptions {
            name,
            platform: None,
        };

        let config = Config {
            image: Some(image),
            env: Some(vec![&input_env]),
            host_config: Some(HostConfig {
                auto_remove: Some(true),
                memory: roption.memory,
                cpu_count: roption.cpu,
                ..Default::default()
            }),
            ..Default::default()
        };

        let container_create_resp = self
            .docker
            .create_container(Some(create_op), config)
            .await?;

        // check
        let op = Some(InspectContainerOptions { size: false });
        let container_info = self
            .docker
            .inspect_container(&container_create_resp.id, op)
            .await?;

        let container_id = container_info.id.ok_or(anyhow!("Missing container"))?;

        // start container
        self.docker
            .start_container(&container_id, None::<StartContainerOptions<String>>)
            .await?;

        Ok(container_id)
    }

    /// remove image
    pub async fn remove(&self, image: &str) -> Result<()> {
        let remove_list = self.docker.remove_image(image, None, None).await?;

        info!("[Docker] remove image: {remove_list:?}");

        Ok(())
    }

    /// container status
    pub async fn status(&self, container: &str) -> Result<Option<ContainerState>> {
        let op = Some(InspectContainerOptions { size: false });

        let container = self.docker.inspect_container(container, op).await?;
        Ok(container.state)
    }

    /// stop container
    pub async fn stop(&self, container: &str) -> Result<()> {
        let op = Some(StopContainerOptions { t: 30 });

        self.docker.stop_container(container, op).await?;
        Ok(())
    }
}
