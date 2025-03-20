use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, config::Credentials};
use aws_sdk_s3::primitives::ByteStream;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::path::Path;
use tokio;

const BUCKET_NAME: &str = "develop";
const MINIO_ENDPOINT: &str = "http://localhost:9000";

async fn get_s3_client() -> Result<Client, Box<dyn Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");

    let config = aws_config::from_env()
        .region(region_provider)
        .endpoint_url(MINIO_ENDPOINT)
        .credentials_provider(Credentials::new("minio", "minio123", None, None, "static"))
        .load()
        .await;
    Ok(Client::new(&config))
}

async fn upload_file(client: &Client, file_path: &str, key: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file_path);
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let byte_stream = ByteStream::from(buffer);

    client
        .put_object()
        .bucket(BUCKET_NAME)
        .key(key)
        .body(byte_stream)
        .send()
        .await?;
    
    println!("filename:{} is uploaded", file_path);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = get_s3_client().await?;
    let file_path = "minio-test-data/wink.png";
    let key = "img/wink.png";

    upload_file(&client, file_path, key).await?;
    Ok(())
}
