use crate::client;
use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("usergroups")
        .about("User groups")
        .subcommand(Command::new("list").about("List user groups").arg(Arg::new("include_count").long("include_count").action(ArgAction::SetTrue)))
        .subcommand(
            Command::new("create")
                .about("Create user group")
                .arg(clap::arg!(<name>))
                .arg(clap::arg!(--description <D>))
                .arg(clap::arg!(--handle <H>)),
        )
        .subcommand(
            Command::new("update")
                .about("Update user group")
                .arg(clap::arg!(<usergroup_id>))
                .arg(clap::arg!(--name <N>))
                .arg(clap::arg!(--description <D>))
                .arg(clap::arg!(--handle <H>)),
        )
        .subcommand(Command::new("disable").about("Disable user group").arg(clap::arg!(<usergroup_id>)))
        .subcommand(Command::new("enable").about("Enable user group").arg(clap::arg!(<usergroup_id>)))
        .subcommand(Command::new("users").about("List users in group").arg(clap::arg!(<usergroup_id>)))
        .subcommand(
            Command::new("users-update")
                .about("Update users in group")
                .arg(clap::arg!(<usergroup_id>))
                .arg(clap::arg!(<user_ids> ...)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    let method = match name {
        "list" => {
            if sub.get_flag("include_count") {
                params.insert("include_count".into(), "true".into());
            }
            "usergroups.list"
        }
        "create" => {
            params.insert("name".into(), sub.get_one::<String>("name").unwrap().clone());
            if let Some(d) = sub.get_one::<String>("description") {
                params.insert("description".into(), d.clone());
            }
            if let Some(h) = sub.get_one::<String>("handle") {
                params.insert("handle".into(), h.clone());
            }
            "usergroups.create"
        }
        "update" => {
            params.insert("usergroup".into(), sub.get_one::<String>("usergroup_id").unwrap().clone());
            if let Some(n) = sub.get_one::<String>("name") {
                params.insert("name".into(), n.clone());
            }
            if let Some(d) = sub.get_one::<String>("description") {
                params.insert("description".into(), d.clone());
            }
            if let Some(h) = sub.get_one::<String>("handle") {
                params.insert("handle".into(), h.clone());
            }
            "usergroups.update"
        }
        "disable" => {
            params.insert("usergroup".into(), sub.get_one::<String>("usergroup_id").unwrap().clone());
            "usergroups.disable"
        }
        "enable" => {
            params.insert("usergroup".into(), sub.get_one::<String>("usergroup_id").unwrap().clone());
            "usergroups.enable"
        }
        "users" => {
            params.insert("usergroup".into(), sub.get_one::<String>("usergroup_id").unwrap().clone());
            "usergroups.users.list"
        }
        "users-update" => {
            params.insert("usergroup".into(), sub.get_one::<String>("usergroup_id").unwrap().clone());
            params.insert("users".into(), sub.get_many::<String>("user_ids").unwrap().cloned().collect::<Vec<_>>().join(","));
            "usergroups.users.update"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
