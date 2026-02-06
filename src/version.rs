use crate::{logger::log, utils::misc::http_client};
use serde_json::Value;
use std::{cmp::Ordering, error::Error};

pub const VERSION: &str = "v1.0.9";

pub fn is_development_version(_version: &str) -> bool {
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

    match parse_version(VERSION).cmp(&parse_version(&get_latest_release().unwrap())) {
        Ordering::Less | Ordering::Equal => false,
        Ordering::Greater => true,
    }
}

pub fn get_latest_tag(owner: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    log("version::get_latest_tag(): Retrieving latest tag...", 0);

    let json: Value = http_client()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest"
        ))
        .send()?
        .json()?;

    let tag = json["tag_name"]
        .as_str()
        .ok_or("Missing tag_name in response")?
        .to_owned();

    log(
        &format!("version::get_latest_tag(): Latest tag: {tag}..."),
        0,
    );

    Ok(tag)
}

pub fn get_latest_release() -> Option<String> {
    log(
        "version::get_latest_release(): Retrieving latest release...",
        0,
    );

    get_latest_tag("owen-debiasio", "cmdcreate").ok()
}

pub fn get_latest_commit(owner: &str, repo: &str, branch: &str) -> String {
    log(
        "version::get_latest_commit(): Retrieving latest commit...",
        0,
    );

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
