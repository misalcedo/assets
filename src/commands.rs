use std::path::PathBuf;
use anyhow::anyhow;
use tokio::io::AsyncReadExt;
use crate::import;
use crate::options::ImportOptions;

pub async fn import_assets(import_options: &ImportOptions) -> anyhow::Result<()> {
    // We could also chunk the assets into smaller batches if needed.
    let contents = read_assets(import_options.path.as_ref()).await?;
    let assets: Vec<import::Asset> = serde_json::from_str(&contents)?;

    let client = reqwest::Client::new();
    let response = client
        .post(import_options.uri.as_str())
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&assets)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if status.is_success() {
        tracing::debug!(%body, %status, "Finished importing assets");
        Ok(())
    } else {
        tracing::error!(%body, %status, "Failed to import assets");
        Err(anyhow!("Failed to import assets"))
    }
}

async fn read_assets(path: Option<&PathBuf>) -> anyhow::Result<String> {
    let mut buffer = String::new();

    match path {
        Some(path) => {
            let mut file = tokio::fs::File::open(path).await?;
            file.read_to_string(&mut buffer).await?;
        }
        None => {
            let mut stdin = tokio::io::stdin();
            stdin.read_to_string(&mut buffer).await?;
        }
    };

    Ok(buffer)
}