use std::env;

use git2::Repository;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

fn main() {
    let repo = Repository::open_from_env().unwrap_or_else(|_| {
        eprintln!("Failed to open repository");
        std::process::exit(1);
    });

    let head = repo.head().unwrap_or_else(|_| {
        eprintln!("Failed to get HEAD");
        std::process::exit(1);
    });

    let commit = head.peel_to_commit().unwrap_or_else(|_| {
        eprintln!("Failed to peel to commit");
        std::process::exit(1);
    });

    let commit_id = commit.id().to_string();

    let commit_prefix = &commit_id[..8];

    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let profile = env::var("BUILD_INFO_PROFILE_PIPELINE").unwrap_or_else(|_| "dev".to_string());

    let time = OffsetDateTime::now_local().unwrap_or_else(|_| {
        eprintln!("Failed to get local time");
        OffsetDateTime::now_utc()
    });
    let time = time.format(&Rfc3339).unwrap_or_else(|_| "unknown".to_string());


    println!("cargo::rustc-env=BUILD_INFO_VERSION_LONG= {version} {profile} (Build: {commit_prefix}, {time})");
}
