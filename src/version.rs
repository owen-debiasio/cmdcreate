use crate::{
    logger::log,
    utils::{
        io::error,
        net::{http_client, is_offline},
    },
};
use serde_json::Value;
use std::{cmp::Ordering, error::Error};

pub const VERSION: &str = "v1.1.1";

pub fn is_development_version() -> bool {
    let parse_version = |v: &str| -> (u32, u32, u32) {
        let nums: Vec<u32> = v
            .trim_start_matches('v')
            .split('.')
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        (
            *nums.first().unwrap_or(&0),
            *nums.get(1).unwrap_or(&0),
            *nums.get(2).unwrap_or(&0),
        )
    };

    match parse_version(VERSION).cmp(&parse_version(&get_latest_tag(
        "owen-debiasio",
        "cmdcreate",
    ))) {
        Ordering::Less | Ordering::Equal => false,
        Ordering::Greater => true,
    }
}

pub fn get_latest_tag(owner: &str, repo: &str) -> String {
    if is_offline() {
        log(
            "version::get_latest_tag(): No internet... Unable to retrieve latest tag...",
            1,
        );
        return "unknown".to_string();
    }

    let result: Result<String, Box<dyn Error>> = (|| {
        let response = http_client()
            .get(format!(
                "https://api.github.com/repos/{owner}/{repo}/releases/latest"
            ))
            .header("User-Agent", "rust-app")
            .send()?;

        let json: Value = response.json()?;

        let tag = json["tag_name"]
            .as_str()
            .ok_or("Missing tag_name")?
            .to_owned();

        Ok(tag)
    })();

    match result {
        Ok(tag) => {
            log(&format!("version::get_latest_tag(): Latest tag: {tag}"), 0);
            tag
        }
        Err(e) => error("Unable to retrieve latest tag:", &e.to_string()),
    }
}

pub fn get_latest_commit(owner: &str, repo: &str, branch: &str) -> String {
    let commit: Value = http_client()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/commits/{branch}"
        ))
        .send()
        .expect("request failed")
        .json()
        .expect("invalid json");

    let commit = commit["sha"]
        .as_str()
        .expect("missing sha")
        .chars()
        .take(7)
        .collect::<String>();

    log(
        &format!("version::get_latest_commit(): Retrieved latest commit: \"{commit}\""),
        0,
    );

    commit
}
