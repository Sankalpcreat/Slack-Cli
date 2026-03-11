use crate::client;
use anyhow::Result;
use clap::{Arg, Command};
use serde_json::json;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("calls")
        .about("Slack Calls")
        .subcommand(
            Command::new("add")
                .about("Register a call")
                .arg(clap::arg!(<external_unique_id>))
                .arg(clap::arg!(<join_url>))
                .arg(clap::arg!(--title <T>))
                .arg(Arg::new("desktop-join-url").long("desktop-join-url").value_name("U")),
        )
        .subcommand(Command::new("end").about("End a call").arg(clap::arg!(<id>)))
        .subcommand(Command::new("info").about("Get call info").arg(clap::arg!(<id>)))
        .subcommand(
            Command::new("update")
                .about("Update call")
                .arg(clap::arg!(<id>))
                .arg(clap::arg!(--title <T>))
                .arg(Arg::new("join-url").long("join-url").value_name("U")),
        )
        .subcommand(
            Command::new("participants-add")
                .about("Add participants")
                .arg(clap::arg!(<id>))
                .arg(clap::arg!(<user_ids> ...)),
        )
        .subcommand(
            Command::new("participants-remove")
                .about("Remove participants")
                .arg(clap::arg!(<id>))
                .arg(clap::arg!(<user_ids> ...)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    let method = match name {
        "add" => {
            params.insert("external_unique_id".into(), sub.get_one::<String>("external_unique_id").unwrap().clone());
            params.insert("join_url".into(), sub.get_one::<String>("join_url").unwrap().clone());
            if let Some(t) = sub.get_one::<String>("title") {
                params.insert("title".into(), t.clone());
            }
            if let Some(u) = sub.get_one::<String>("desktop-join-url") {
                params.insert("desktop_app_join_url".into(), u.clone());
            }
            "calls.add"
        }
        "end" => {
            params.insert("id".into(), sub.get_one::<String>("id").unwrap().clone());
            "calls.end"
        }
        "info" => {
            params.insert("id".into(), sub.get_one::<String>("id").unwrap().clone());
            "calls.info"
        }
        "update" => {
            params.insert("id".into(), sub.get_one::<String>("id").unwrap().clone());
            if let Some(t) = sub.get_one::<String>("title") {
                params.insert("title".into(), t.clone());
            }
            if let Some(u) = sub.get_one::<String>("join-url") {
                params.insert("join_url".into(), u.clone());
            }
            "calls.update"
        }
        "participants-add" => {
            params.insert("id".into(), sub.get_one::<String>("id").unwrap().clone());
            let users: Vec<serde_json::Value> = sub
                .get_many::<String>("user_ids")
                .unwrap()
                .map(|u| json!({"slack_id": u}))
                .collect();
            params.insert("users".into(), serde_json::to_string(&users)?);
            "calls.participants.add"
        }
        "participants-remove" => {
            params.insert("id".into(), sub.get_one::<String>("id").unwrap().clone());
            let users: Vec<serde_json::Value> = sub
                .get_many::<String>("user_ids")
                .unwrap()
                .map(|u| json!({"slack_id": u}))
                .collect();
            params.insert("users".into(), serde_json::to_string(&users)?);
            "calls.participants.remove"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
