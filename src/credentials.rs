use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
    pub team_domain: String,
    #[serde(default)]
    pub user_id: String,
}

pub fn save(token: &str, team_id: &str, team_domain: &str, user_id: &str) -> Result<()> {
    let dir = dirs::home_dir().context("no home dir")?;
    let slack_dir = dir.join(".slack");
    std::fs::create_dir_all(&slack_dir).context("create .slack dir")?;
    let path = slack_dir.join("credentials.json");
    let mut data: HashMap<String, Auth> = std::fs::read(&path)
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default();
    let key = if team_id.is_empty() { "default" } else { team_id };
    data.insert(
        key.to_string(),
        Auth {
            token: token.to_string(),
            team_id: team_id.to_string(),
            team_domain: team_domain.to_string(),
            user_id: user_id.to_string(),
        },
    );
    std::fs::write(&path, serde_json::to_string_pretty(&data)?).context("write credentials")?;
    Ok(())
}

pub fn load(team_id: Option<&str>) -> Result<String> {
    if let Ok(t) = std::env::var("SLACK_TOKEN") {
        if !t.is_empty() {
            return Ok(t);
        }
    }
    let dir = dirs::home_dir().context("no home dir")?;
    let path: PathBuf = [dir, ".slack".into(), "credentials.json".into()].iter().collect();
    let data: HashMap<String, Auth> =
        serde_json::from_slice(&std::fs::read(&path).context("read credentials")?)
            .context("parse credentials")?;
    if let Some(tid) = team_id {
        if let Some(a) = data.get(tid) {
            if !a.token.is_empty() {
                return Ok(a.token.clone());
            }
        }
    }
    for a in data.values() {
        if !a.token.is_empty() {
            return Ok(a.token.clone());
        }
    }
    anyhow::bail!("no token found")
}
