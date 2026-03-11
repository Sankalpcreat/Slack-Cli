use crate::client;
use crate::credentials;
use anyhow::{Context, Result};
use clap::Command;
use std::collections::HashMap;
use std::env;

pub fn command() -> Command {
    Command::new("apps")
        .about("App manifest and scopes (requires config token)")
        .arg(clap::Arg::new("config_token").long("config-token").value_name("T").help("App config token"))
        .subcommand(
            Command::new("manifest")
                .subcommand(Command::new("export").about("Export manifest").arg(clap::arg!(<app_id>)))
                .subcommand(
                    Command::new("update")
                        .about("Update from manifest file")
                        .arg(clap::arg!(<app_id>))
                        .arg(clap::arg!(<manifest_file>)),
                ),
        )
        .subcommand(
            Command::new("scopes")
                .subcommand(
                    Command::new("add")
                        .about("Add scopes")
                        .arg(clap::arg!(<app_id>))
                        .arg(clap::arg!(--user <S> "Comma-separated user scopes"))
                        .arg(clap::arg!(--bot <S> "Comma-separated bot scopes")),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove scopes")
                        .arg(clap::arg!(<app_id>))
                        .arg(clap::arg!(--user <S>))
                        .arg(clap::arg!(--bot <S>)),
                ),
        )
        .subcommand(
            Command::new("permissions")
                .about("App permissions (uses user token)")
                .subcommand(Command::new("info").about("Get permissions info"))
                .subcommand(
                    Command::new("request")
                        .about("Request scopes")
                        .arg(clap::arg!(<scopes>))
                        .arg(clap::arg!([trigger_id])),
                )
                .subcommand(
                    Command::new("resources")
                        .about("List permission resources")
                        .arg(clap::arg!([resource_types]))
                        .arg(clap::arg!([cursor])),
                ),
        )
}

pub fn run(m: &clap::ArgMatches, team_id: Option<&str>) -> Result<()> {
    let config_token = m.get_one::<String>("config_token").map(|s| s.as_str());
    let (name, sub) = m.subcommand().unwrap();
    match name {
        "manifest" => {
            let tok = config_token.map(String::from).or_else(|| env::var("SLACK_CONFIG_TOKEN").ok()).filter(|s| !s.is_empty()).ok_or_else(|| anyhow::anyhow!("config token required"))?;
            let (mname, msub) = sub.subcommand().unwrap();
            match mname {
                "export" => {
                    let mut params = HashMap::new();
                    params.insert("app_id".into(), msub.get_one::<String>("app_id").unwrap().clone());
                    let out = client::call(&tok, "apps.manifest.export", Some(&params))?;
                    println!("{}", String::from_utf8_lossy(&out));
                }
                "update" => {
                    let app_id = msub.get_one::<String>("app_id").unwrap();
                    let path = msub.get_one::<String>("manifest_file").unwrap();
                    let manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
                    let mut params = HashMap::new();
                    params.insert("app_id".into(), app_id.clone());
                    params.insert("manifest".into(), manifest.to_string());
                    let out = client::call(&tok, "apps.manifest.update", Some(&params))?;
                    println!("{}", String::from_utf8_lossy(&out));
                }
                _ => {}
            }
        }
        "scopes" => {
            let tok = config_token.map(String::from).or_else(|| env::var("SLACK_CONFIG_TOKEN").ok()).filter(|s| !s.is_empty()).ok_or_else(|| anyhow::anyhow!("config token required"))?;
            let (sname, ssub) = sub.subcommand().unwrap();
            let app_id = ssub.get_one::<String>("app_id").unwrap();
            let mut params = HashMap::new();
            params.insert("app_id".into(), app_id.clone());
            let resp = client::call(&tok, "apps.manifest.export", Some(&params))?;
            let mut data: serde_json::Value = serde_json::from_slice(&resp)?;
            let manifest = data.get_mut("manifest").context("no manifest in response")?;
            let oauth = manifest.get_mut("oauth_config").and_then(|o| o.as_object_mut()).context("no oauth_config")?;
            let scopes = oauth.entry("scopes").or_insert(serde_json::json!({}));
            let scopes_obj = scopes.as_object_mut().context("scopes not object")?;
            let user_list: Vec<String> = serde_json::from_value(scopes_obj.get("user").cloned().unwrap_or(serde_json::json!([]))).unwrap_or_default();
            let bot_list: Vec<String> = serde_json::from_value(scopes_obj.get("bot").cloned().unwrap_or(serde_json::json!([]))).unwrap_or_default();
            let flag_user = ssub.get_one::<String>("user").map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect::<Vec<_>>()).unwrap_or_default();
            let flag_bot = ssub.get_one::<String>("bot").map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect::<Vec<_>>()).unwrap_or_default();
            let (new_user, new_bot) = if sname == "add" {
                let mut u = user_list;
                let mut b = bot_list;
                for s in flag_user {
                    if !u.contains(&s) {
                        u.push(s);
                    }
                }
                for s in flag_bot {
                    if !b.contains(&s) {
                        b.push(s);
                    }
                }
                (u, b)
            } else {
                let rm_user: std::collections::HashSet<_> = flag_user.into_iter().collect();
                let rm_bot: std::collections::HashSet<_> = flag_bot.into_iter().collect();
                (user_list.into_iter().filter(|x| !rm_user.contains(x)).collect(), bot_list.into_iter().filter(|x| !rm_bot.contains(x)).collect())
            };
            scopes_obj.insert("user".to_string(), serde_json::to_value(new_user)?);
            scopes_obj.insert("bot".to_string(), serde_json::to_value(new_bot)?);
            let mut upd = HashMap::new();
            upd.insert("app_id".to_string(), app_id.clone());
            upd.insert("manifest".to_string(), manifest.to_string());
            let out = client::call(&tok, "apps.manifest.update", Some(&upd))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        "permissions" => {
            let token = credentials::load(team_id)?;
            let (pname, psub) = sub.subcommand().unwrap();
            let mut params = HashMap::new();
            let method = match pname {
                "info" => {
                    let out = client::call(&token, "apps.permissions.info", None)?;
                    println!("{}", String::from_utf8_lossy(&out));
                    return Ok(());
                }
                "request" => {
                    params.insert("scopes".into(), psub.get_one::<String>("scopes").unwrap().clone());
                    if let Some(t) = psub.get_one::<String>("trigger_id") {
                        params.insert("trigger_id".into(), t.clone());
                    }
                    "apps.permissions.request"
                }
                "resources" => {
                    if let Some(r) = psub.get_one::<String>("resource_types") {
                        params.insert("resource_types".into(), r.clone());
                    }
                    if let Some(c) = psub.get_one::<String>("cursor") {
                        params.insert("cursor".into(), c.clone());
                    }
                    "apps.permissions.resources.list"
                }
                _ => return Ok(()),
            };
            let out = client::call(&token, method, Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
