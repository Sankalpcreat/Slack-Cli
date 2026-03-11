use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("files")
        .about("Files")
        .subcommand(
            Command::new("list")
                .about("List files")
                .arg(clap::arg!(--limit <N>).default_value("100"))
                .arg(clap::arg!(--channel <ID>))
                .arg(clap::arg!(--user <ID>)),
        )
        .subcommand(Command::new("info").about("Get file info").arg(clap::arg!(<file_id>)))
        .subcommand(Command::new("delete").about("Delete a file").arg(clap::arg!(<file_id>)))
        .subcommand(
            Command::new("upload")
                .about("Upload file to channel")
                .arg(clap::arg!(<channel>))
                .arg(clap::arg!(<file_path>))
                .arg(clap::arg!(--comment <TEXT>)),
        )
        .subcommand(
            Command::new("download")
                .about("Download file by ID")
                .arg(clap::arg!(<file_id>))
                .arg(clap::arg!([output_path])),
        )
        .subcommand(Command::new("revoke-public").about("Revoke public URL").arg(clap::arg!(<file_id>)))
        .subcommand(Command::new("share-public").about("Share file publicly").arg(clap::arg!(<file_id>)))
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    match name {
        "list" => {
            let mut params = HashMap::new();
            params.insert("limit".into(), sub.get_one::<String>("limit").cloned().unwrap_or_else(|| "100".into()));
            if let Some(c) = sub.get_one::<String>("channel") {
                params.insert("channel".into(), c.clone());
            }
            if let Some(u) = sub.get_one::<String>("user") {
                params.insert("user".into(), u.clone());
            }
            let out = client::call(token, "files.list", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        "info" => {
            let mut params = HashMap::new();
            params.insert("file".into(), sub.get_one::<String>("file_id").unwrap().clone());
            let out = client::call(token, "files.info", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        "delete" => {
            let mut params = HashMap::new();
            params.insert("file".into(), sub.get_one::<String>("file_id").unwrap().clone());
            let out = client::call(token, "files.delete", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        "upload" => {
            let channel = sub.get_one::<String>("channel").unwrap();
            let path = sub.get_one::<String>("file_path").unwrap();
            let comment = sub.get_one::<String>("comment").cloned();
            let out = client::upload_file(token, channel, path, comment.as_deref())?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        "download" => {
            let file_id = sub.get_one::<String>("file_id").unwrap();
            let out_path = sub.get_one::<String>("output_path").map(|s| s.as_str());
            let saved = client::download_file(token, file_id, out_path)?;
            println!("Downloaded to {}", saved);
        }
        "revoke-public" => {
            let mut params = HashMap::new();
            params.insert("file".into(), sub.get_one::<String>("file_id").unwrap().clone());
            let out = client::call(token, "files.revokePublicURL", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        "share-public" => {
            let mut params = HashMap::new();
            params.insert("file".into(), sub.get_one::<String>("file_id").unwrap().clone());
            let out = client::call(token, "files.sharedPublicURL", Some(&params))?;
            println!("{}", String::from_utf8_lossy(&out));
        }
        _ => {}
    }
    Ok(())
}
