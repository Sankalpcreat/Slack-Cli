use crate::client;
use anyhow::Result;
use clap::Command;
use std::collections::HashMap;

pub fn command() -> Command {
    Command::new("workflows")
        .about("Workflow Builder steps")
        .subcommand(
            Command::new("step-completed")
                .about("Complete workflow step")
                .arg(clap::arg!(<workflow_step_execute_id>))
                .arg(clap::arg!([outputs_json])),
        )
        .subcommand(
            Command::new("step-failed")
                .about("Fail workflow step")
                .arg(clap::arg!(<workflow_step_execute_id>))
                .arg(clap::arg!([error_message] ...)),
        )
}

pub fn run(m: &clap::ArgMatches, token: &str) -> Result<()> {
    let (name, sub) = m.subcommand().unwrap();
    let mut params = HashMap::new();
    params.insert("workflow_step_execute_id".into(), sub.get_one::<String>("workflow_step_execute_id").unwrap().clone());
    let method = match name {
        "step-completed" => {
            if let Some(o) = sub.get_one::<String>("outputs_json") {
                params.insert("outputs".into(), o.clone());
            }
            "workflows.stepCompleted"
        }
        "step-failed" => {
            if let Some(parts) = sub.get_many::<String>("error_message") {
                params.insert("error".into(), parts.cloned().collect::<Vec<_>>().join(" "));
            }
            "workflows.stepFailed"
        }
        _ => return Ok(()),
    };
    let out = client::call(token, method, Some(&params))?;
    println!("{}", String::from_utf8_lossy(&out));
    Ok(())
}
