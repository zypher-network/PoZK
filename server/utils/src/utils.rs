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

pub async fn write_task_input(tid: &str, inputs: Vec<u8>, publics: Vec<u8>) -> Result<()> {
    let mut path = BASE_PATH.get().expect("Missing BASE PATH").clone();
    path.push(tid);

    let mut bytes = (inputs.len() as u32).to_be_bytes().to_vec();
    bytes.extend(inputs);
    bytes.extend(publics);

    fs::write(path, bytes).await?;
    Ok(())
}

pub async fn read_task_input(tid: &str) -> Result<Vec<u8>> {
    let mut path = BASE_PATH.get().expect("Missing BASE PATH").clone();
    path.push(tid);

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

pub async fn remove_task_input(tid: &str) -> Result<()> {
    let mut path = BASE_PATH.get().expect("Missing BASE PATH").clone();
    path.push(tid);

    fs::remove_file(path).await?;
    Ok(())
}

pub async fn write_task_proof(tid: &str, proof: Vec<u8>) -> Result<()> {
    let mut path = BASE_PATH.get().expect("Missing BASE PATH").clone();
    path.push(format!("proof-{}", tid));

    fs::write(path, proof).await?;
    Ok(())
}

pub async fn read_task_proof(tid: &str) -> Result<Vec<u8>> {
    let mut path = BASE_PATH.get().expect("Missing BASE PATH").clone();
    path.push(format!("proof-{}", tid));

    let bytes = fs::read(&path).await?;
    fs::remove_file(path).await?;

    Ok(bytes)
}

pub fn get_task_api(tid: &str) -> String {
    let server = API_SERVER.get().expect("Missing API SERVER");
    format!("{}/inner/tasks/{}", server, tid)
}

pub fn convert_task_to_connect_api(url: &str) -> (String, &str) {
    let mut v: Vec<&str> = url.split('/').collect();
    let id = v.pop().unwrap_or("");
    let _ = v.pop(); // pop tasks
    v.push("connect"); // push connect
    (v.join("/"), id)
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

pub fn is_valid_url(url: &str, https: bool) -> bool {
    let rule = if https {
        r"^https://([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,6}(/[a-zA-Z0-9-._~:/?#@!$&'()*+,;=]*)?$"
    } else {
        r"^(http://|https://)([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,6}(/[a-zA-Z0-9-._~:/?#@!$&'()*+,;=]*)?$"
    };

    let url_regex = regex::Regex::new(rule).unwrap(); // safe
    url_regex.is_match(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        assert_eq!(is_valid_url("", true), false);
        assert_eq!(is_valid_url("http://localhost", true), false);
        assert_eq!(is_valid_url("https://", true), false);
        assert_eq!(is_valid_url("example.com", true), false);
        assert_eq!(is_valid_url("http://example.com", true), false);
        assert_eq!(is_valid_url("http://example.com", false), true);
        assert_eq!(is_valid_url("https://example.com", true), true);
        assert_eq!(
            is_valid_url("https://example.org/path?query=string#fragment", true),
            true
        );
        assert_eq!(is_valid_url("https://sub.domain.co.uk", true), true);
        assert_eq!(is_valid_url("ftp://example.com", true), false);
    }

    #[tokio::test]
    async fn test_download() {
        let res = download_input_with_uri("http://localhost:9098/inner/tasks/1")
            .await
            .unwrap();
        println!("{} {:?}", res.len(), res);
    }

    #[tokio::test]
    async fn test_upload() {
        upload_proof_with_uri(
            "http://localhost:9098/inner/tasks/1",
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
        )
        .await
        .unwrap();
    }
}
