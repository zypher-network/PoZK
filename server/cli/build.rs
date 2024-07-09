use std::env;

use git2::Repository;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

fn main() {
    let repo = Repository::open_from_env().unwrap();
    let head = repo.head().unwrap();

    let commit = head.peel_to_commit().unwrap();

    let commit_id = commit.id().to_string();

    let commit_prefix = &commit_id[..8];

    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let profile = env::var("BUILD_INFO_PROFILE_PIPELINE").unwrap_or(String::from("dev"));

    let time = OffsetDateTime::now_local().unwrap();
    let time = time.format(&Rfc3339).unwrap();

    println!("cargo::rustc-env=BUILD_INFO_VERSION_LONG= {version} {profile} (Build: {commit_prefix}, {time})");
}
