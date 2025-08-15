use std::io::Read;
use std::path::PathBuf;
use crate::import;
use crate::options::ImportOptions;

pub fn import_assets(import_options: &ImportOptions) -> anyhow::Result<()> {
    // We could also chunk the assets into smaller batches if needed.
    let contents = read_assets(import_options.path.as_ref())?;

    let assets: Vec<import::Asset> = serde_json::from_str(&contents)?;
    println!("{assets:?}");

    Ok(())
}

fn read_assets(path: Option<&PathBuf>) -> anyhow::Result<String> {
    let mut buffer = String::new();

    match path {
        Some(path) => {
            let mut file = std::fs::File::open(path)?;
            file.read_to_string(&mut buffer)?;
        }
        None => {
            let mut stdin = std::io::stdin();
            stdin.read_to_string(&mut buffer)?;
        }
    };

    Ok(buffer)
}