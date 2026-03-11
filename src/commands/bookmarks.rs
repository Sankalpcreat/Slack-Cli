use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("bookmarks")
        .about("Channel bookmarks")
        .subcommand(Command::new("list").about("List bookmarks").arg(clap::arg!(<channel_id>)))
        .subcommand(
            Command::new("add")
                .about("Add bookmark")
                .arg(clap::arg!(<channel_id>))
                .arg(clap::arg!(<title>))
                .arg(clap::arg!(<link>))
                .arg(clap::arg!(--type <T> "link").default_value("link")),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit bookmark")
                .arg(clap::arg!(<channel_id>))
                .arg(clap::arg!(<bookmark_id>))
                .arg(clap::arg!(--title <T>))
                .arg(clap::arg!(--link <L>)),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove bookmark")
                .arg(clap::arg!(<channel_id>))
                .arg(clap::arg!(<bookmark_id>)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    params.insert("channel_id".into(), sub.get_one::<String>("channel_id").unwrap().clone());
    let method = match name {
        "list" => "bookmarks.list",
        "add" => {
            params.insert("title".into(), sub.get_one::<String>("title").unwrap().clone());
            params.insert("link".into(), sub.get_one::<String>("link").unwrap().clone());
            params.insert("type".into(), sub.get_one::<String>("type").cloned().unwrap_or_else(|| "link".into()));
            "bookmarks.add"
        }
        "edit" => {
            params.insert("bookmark_id".into(), sub.get_one::<String>("bookmark_id").unwrap().clone());
            if let Some(t) = sub.get_one::<String>("title") {
                params.insert("title".into(), t.clone());
            }
            if let Some(l) = sub.get_one::<String>("link") {
                params.insert("link".into(), l.clone());
            }
            "bookmarks.edit"
        }
        "remove" => {
            params.insert("bookmark_id".into(), sub.get_one::<String>("bookmark_id").unwrap().clone());
            "bookmarks.remove"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
