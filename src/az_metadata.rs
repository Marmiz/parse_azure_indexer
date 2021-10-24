use serde::Deserialize;
use std::{error::Error, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct AzureMeta {
    service_name: String,
    index_name: String,
    api_version: String,
}

pub fn read_toml_file(file: PathBuf) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(file)?;

    let def: AzureMeta = toml::from_str(&content)?;
    dbg!(def);

    Ok(())
}
