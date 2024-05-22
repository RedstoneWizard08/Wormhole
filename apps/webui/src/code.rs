use std::{env, fs, path::PathBuf, process::Command};

use anyhow::Result;
use axum::extract::Query;

pub fn is_openvscode_server() -> Result<bool> {
    if let Ok(browser) = env::var("BROWSER") {
        if !browser.ends_with(".sh") {
            return Ok(false);
        }

        let data = fs::read_to_string(browser)?;

        return Ok(data.contains("openvscode-server"));
    } else {
        Ok(false)
    }
}

pub fn get_helper_path() -> Result<PathBuf> {
    let browser = env::var("BROWSER")?;

    Ok(PathBuf::from(browser)
        .parent()
        .unwrap()
        .join("..")
        .join("remote-cli")
        .join("openvscode-server"))
}

pub fn open_file(data: String) -> Result<()> {
    Command::new(get_helper_path()?)
        .arg("--goto")
        .arg(data)
        .spawn()?
        .wait()?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    pub file: String,
}

#[axum::debug_handler]
pub async fn handler(Query(QueryParams { file }): Query<QueryParams>) -> Result<String> {
    open_file(file)?;

    Ok(String::new())
}
