use chrono::prelude::*;
use std::process::Command;

fn main() {
    // Set the APP_BUILD_DATE
    let local: DateTime<Local> = Local::now();
    println!("cargo:rustc-env=APP_BUILD_DATE={}.{:0>2}.{:0>2}", local.year(), local.month(), local.day());

    // Set the APP_GIT_COMMIT
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=APP_GIT_COMMIT={}", git_hash);
}
