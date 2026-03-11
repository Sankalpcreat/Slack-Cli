use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("users")
        .about("Users")
        .subcommand(
            Command::new("list")
                .about("List users")
                .arg(clap::arg!(--limit <N>).default_value("100")),
        )
        .subcommand(Command::new("info").about("Get user info").arg(clap::arg!(<user_id>)))
        .subcommand(Command::new("lookup").about("Lookup by email").arg(clap::arg!(<email>)))
        .subcommand(Command::new("presence").about("Get presence").arg(clap::arg!(<user_id>)))
        .subcommand(Command::new("set-presence").about("Set presence (auto|away)").arg(clap::arg!(<presence>)))
        .subcommand(Command::new("profile").about("Get profile").arg(clap::arg!(<user_id>)))
        .subcommand(
            Command::new("profile-set")
                .about("Set profile")
                .arg(clap::arg!(<user_id>))
                .arg(clap::arg!(--profile <JSON>).required(true)),
        )
        .subcommand(Command::new("set-photo").about("Set profile photo").arg(clap::arg!(<image_path>)))
        .subcommand(Command::new("delete-photo").about("Remove profile photo"))
        .subcommand(
            Command::new("conversations")
                .about("List user's conversations")
                .arg(clap::arg!([user_id]))
                .arg(clap::arg!(--limit <N>).default_value("100"))
                .arg(clap::arg!(--types <T>).default_value("public_channel,private_channel,mpim,im")),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    let method = match name {
        "list" => {
            params.insert("limit".into(), sub.get_one::<String>("limit").cloned().unwrap_or_else(|| "100".into()));
            "users.list"
        }
        "info" => {
            params.insert("user".into(), sub.get_one::<String>("user_id").unwrap().clone());
            "users.info"
        }
        "lookup" => {
            params.insert("email".into(), sub.get_one::<String>("email").unwrap().clone());
            "users.lookupByEmail"
        }
        "presence" => {
            params.insert("user".into(), sub.get_one::<String>("user_id").unwrap().clone());
            "users.getPresence"
        }
        "set-presence" => {
            params.insert("presence".into(), sub.get_one::<String>("presence").unwrap().clone());
            "users.setPresence"
        }
        "profile" => {
            params.insert("user".into(), sub.get_one::<String>("user_id").unwrap().clone());
            "users.profile.get"
        }
        "profile-set" => {
            params.insert("user".into(), sub.get_one::<String>("user_id").unwrap().clone());
            params.insert("profile".into(), sub.get_one::<String>("profile").unwrap().clone());
            "users.profile.set"
        }
        "set-photo" => {
            let path = sub.get_one::<String>("image_path").unwrap();
            let out = client::set_user_photo(token, path)?;
            println!("{}", String::from_utf8_lossy(&out));
            return Ok(());
        }
        "delete-photo" => {
            let out = client::call(token, "users.deletePhoto", None)?;
            println!("{}", String::from_utf8_lossy(&out));
            return Ok(());
        }
        "conversations" => {
            params.insert("limit".into(), sub.get_one::<String>("limit").cloned().unwrap_or_else(|| "100".into()));
            params.insert("types".into(), sub.get_one::<String>("types").cloned().unwrap_or_else(|| "public_channel,private_channel,mpim,im".into()));
            if let Some(u) = sub.get_one::<String>("user_id") {
                params.insert("user".into(), u.clone());
            }
            "users.conversations"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
