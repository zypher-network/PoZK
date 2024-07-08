#[cfg(test)]
mod tests {
    use futures_util::StreamExt;
    use shiplift::{ContainerOptions, Docker, PullOptions};

    #[test]
    fn test_local_start_postgres() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let docker = Docker::new();

            let pull_options = PullOptions::builder().image("postgres:latest").build();
            let mut pull_stream = docker.images().pull(&pull_options);
            while let Some(pull_result) = pull_stream.next().await {
                match pull_result {
                    Ok(output) => println!("{:?}", output),
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }

            let container_options = ContainerOptions::builder("postgres:latest")
                .memory(512 * 1024 * 1024)
                .cpu_shares(100_000)
                .cpus(1.0)
                .expose(5432, "tcp", 5432)
                .env(vec!["POSTGRES_PASSWORD=1"])
                .name("minner-postgres")
                .build();

            let container = docker
                .containers()
                .create(&container_options)
                .await
                .unwrap();
            docker
                .containers()
                .get(&container.id)
                .start()
                .await
                .unwrap();

            println!("Container started with ID: {}", container.id);
        });
    }
}
