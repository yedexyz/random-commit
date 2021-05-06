use std::fs;
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize)]
struct Commits {
    messages: Vec<String>,
}

fn main() {
    let data = fs::read_to_string("commits.json")
        .expect("Failed to read commits.json");

    let commits: Commits = serde_json::from_str(&data)
        .expect("Failed to parse commits.json");

    let random_commit = commits.messages.choose(&mut rand::thread_rng())
        .expect("Failed to pick random commit message");

    println!("{}", random_commit);
}
