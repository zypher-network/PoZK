use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tokio::fs;

static BASE_PATH: OnceCell<PathBuf> = OnceCell::new();
static API_SERVER: OnceCell<String> = OnceCell::new();

pub fn init_path_and_server(path: &str, server: &str) {
    BASE_PATH
        .set(PathBuf::from(path))
        .expect("Unable set BASE_PATH");
    API_SERVER
        .set(server.to_owned())
        .expect("Unable set API_SERVER");
}

pub async fn write_task_input(tid: u64, inputs: Vec<u8>, publics: Vec<u8>) -> Result<()> {
    let mut path = BASE_PATH.get().expect("Missing BASE PATH").clone();
    path.push(tid.to_string());

    let mut bytes = (inputs.len() as u32).to_be_bytes().to_vec();
    bytes.extend(inputs);
    bytes.extend(publics);

    fs::write(path, bytes).await?;
    Ok(())
}

pub async fn read_task_input(tid: u64) -> Result<Vec<u8>> {
    let mut path = BASE_PATH.get().expect("Missing BASE PATH").clone();
    path.push(tid.to_string());

    let bytes = fs::read(path).await?;
    Ok(bytes)
}

pub async fn parse_task_input(data: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut inputs_len_bytes = [0u8; 4];
    inputs_len_bytes.copy_from_slice(&data[0..4]);
    let inputs_len = u32::from_be_bytes(inputs_len_bytes) as usize;
    if data.len() < inputs_len + 4 {
        return Err(anyhow!("Invalid proof length"));
    }

    let raw_data = &data[4..];
    let inputs = raw_data[..inputs_len].to_vec();
    let publics = raw_data[inputs_len..].to_vec();

    Ok((inputs, publics))
}

pub async fn remove_task_input(tid: u64) -> Result<()> {
    let mut path = BASE_PATH.get().expect("Missing BASE PATH").clone();
    path.push(tid.to_string());

    fs::remove_file(path).await?;
    Ok(())
}

pub fn get_task_api(tid: u64) -> String {
    let server = API_SERVER.get().expect("Missing API SERVER");
    format!("{}/tasks/{}", server, tid)
}

pub async fn download_input() -> Result<Vec<u8>> {
    let uri = std::env::var("INPUT").unwrap();
    download_input_with_uri(&uri).await
}

pub async fn download_input_with_uri(uri: &str) -> Result<Vec<u8>> {
    let body = reqwest::get(uri).await?.bytes().await?;
    Ok(body.to_vec())
}

pub async fn upload_proof(output: Vec<u8>, proof: Vec<u8>) -> Result<()> {
    let uri = std::env::var("INPUT").unwrap();
    upload_proof_with_uri(&uri, output, proof).await
}

pub async fn upload_proof_with_uri(uri: &str, output: Vec<u8>, proof: Vec<u8>) -> Result<()> {
    let mut bytes = vec![];
    bytes.extend((output.len() as u32).to_be_bytes());
    bytes.extend(output);
    bytes.extend(proof);

    let client = reqwest::Client::new();
    let _ = client.post(uri).body(bytes).send().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download() {
        let res = download_input_with_uri("http://localhost:9098/tasks/1")
            .await
            .unwrap();
        println!("{} {:?}", res.len(), res);
    }

    #[tokio::test]
    async fn test_upload() {
        upload_proof_with_uri(
            "http://localhost:9098/tasks/1",
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
        )
        .await
        .unwrap();
    }
}
