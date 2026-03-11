use crate::client;
use crate::credentials;
use anyhow::{Context, Result};
use clap::Command;
use std::collections::HashMap;
use std::env;

const DEFAULT_USER_SCOPES: &str = "channels:read,channels:write,chat:write,users:read,files:read,files:write,groups:read,groups:write,links:write";

fn rotate_token(refresh_token: &str) -> Result<String> {
    let mut params = HashMap::new();
    params.insert("refresh_token".into(), refresh_token.to_string());
    let out = client::call_opt(None, "tooling.tokens.rotate", Some(&params))?;
    let dec: serde_json::Value = serde_json::from_slice(&out)?;
    dec.get("token")
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("no token in rotate response"))
}

pub fn command() -> Command {
    Command::new("apps")
        .about("App manifest and scopes (requires config token)")
        .arg(clap::Arg::new("config_token").long("config-token").value_name("T").help("App config token"))
        .subcommand(
            Command::new("token")
                .subcommand(
                    Command::new("rotate")
                        .about("Rotate refresh token to get new config token")
                        .arg(clap::arg!(--"refresh-token" <T> "Refresh token (xoxe-...)").required(true)),
                ),
        )
        .subcommand(
            Command::new("create")
                .about("Create Slack app from manifest")
                .arg(clap::arg!(--name <N> "App name").required(true))
                .arg(clap::arg!(--"config-token" <T> "Config token (xoxe-...)").required(false))
                .arg(clap::arg!(--"refresh-token" <T> "Refresh token (use instead of config-token)").required(false))
                .arg(clap::arg!(--scopes <S> "Comma-separated user scopes").default_value(DEFAULT_USER_SCOPES)),
        )
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
        "token" => {
            let (tname, tsub) = sub.subcommand().unwrap();
            if tname == "rotate" {
                let refresh = tsub.get_one::<String>("refresh-token").unwrap();
                let mut params = HashMap::new();
                params.insert("refresh_token".into(), refresh.clone());
                let out = client::call_opt(None, "tooling.tokens.rotate", Some(&params))?;
                println!("{}", String::from_utf8_lossy(&out));
                return Ok(());
            }
        }
        "create" => {
            let app_name = sub.get_one::<String>("name").unwrap();
            let config_tok = sub.get_one::<String>("config-token");
            let refresh_tok = sub.get_one::<String>("refresh-token");
            let scopes_str = sub.get_one::<String>("scopes").map(|s| s.as_str()).unwrap_or(DEFAULT_USER_SCOPES);
            let tok = if let Some(t) = config_tok {
                t.clone()
            } else if let Some(r) = refresh_tok {
                rotate_token(r)?
            } else if let Ok(t) = env::var("SLACK_CONFIG_TOKEN") {
                t
            } else if let Ok(r) = env::var("SLACK_REFRESH_TOKEN") {
                rotate_token(&r)?
            } else {
                anyhow::bail!("provide --config-token, --refresh-token, or set SLACK_CONFIG_TOKEN/SLACK_REFRESH_TOKEN")
            };
            let user_scopes: Vec<&str> = scopes_str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            let long_desc = "A command-line application that uses the Slack Web API. Created programmatically for use with slack-cli and similar tools. Supports channels, chat, files, users, search, reactions, and more. Use Slack from the terminal, scripts, or AI agents without opening the Slack app.";
            let manifest = serde_json::json!({
                "_metadata": {"major_version": 2, "minor_version": 1},
                "display_information": {
                    "name": app_name,
                    "description": "CLI app for Slack API",
                    "long_description": long_desc
                },
                "oauth_config": {
                    "scopes": {"user": user_scopes},
                    "redirect_urls": ["https://localhost"]
                },
                "settings": {
                    "socket_mode_enabled": false,
                    "org_deploy_enabled": false
                }
            });
            let mut params = HashMap::new();
            params.insert("manifest".into(), manifest.to_string());
            let out = client::call(&tok, "apps.manifest.create", Some(&params))?;
            let dec: serde_json::Value = serde_json::from_slice(&out)?;
            let app_id = dec.get("app_id").and_then(|v| v.as_str()).unwrap_or("?");
            let install_url = format!("https://api.slack.com/apps/{}/oauth", app_id);
            println!("{}", String::from_utf8_lossy(&out));
            println!("\nNext: 1. Open {} and add redirect URL (e.g. https://localhost)", install_url);
            println!("      2. Click Install to Workspace, approve");
            println!("      3. Copy User OAuth Token (xoxp-...) and run: slack-cli auth login --token xoxp-...");
            if cfg!(target_os = "macos") {
                let _ = std::process::Command::new("open").arg(&install_url).spawn();
            } else if cfg!(target_os = "linux") {
                let _ = std::process::Command::new("xdg-open").arg(&install_url).spawn();
            } else if cfg!(target_os = "windows") {
                let _ = std::process::Command::new("cmd").args(["/c", "start", &install_url]).spawn();
            }
            return Ok(());
        }
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
