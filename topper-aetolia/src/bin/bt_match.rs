//! bt_match — compare a behavior tree against actions taken in a combat log.
//!
//! Usage:
//!   cargo run --bin bt_match -- \
//!     --log topper-explainer/my_logs/Foo_vs_Bar.json \
//!     --player Sheryni \
//!     --tree sentinel/base
//!
//! Exits 0 if all observed actions match the BT, or 1 at the first divergence.

#![allow(warnings)]
#[macro_use]
extern crate lazy_static;

use std::{collections::HashSet, env, fs};

use topper_aetolia::{
    bt_match::{set_bt_dir, BtMatchConfig, MatchRunner},
    explainer::{observations::OBSERVER, parse_me_and_you, ExplainerPage},
};

fn main() {
    let (log_path, player_name, tree_name, config_path) = parse_args();
    let ignored = load_config(&config_path);
    let (page, opponent_name) = load_log(&log_path, &player_name, &tree_name);

    let bt_dir = {
        let mut dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        dir.push("behavior_trees");
        dir.to_string_lossy().to_string()
    };
    set_bt_dir(&bt_dir);

    let time_slices = page.build_time_slices(&|slice| OBSERVER.observe(slice));
    println!("Processing {} time slices...\n", time_slices.len());

    let mut runner = MatchRunner::new(player_name, opponent_name, tree_name, ignored);
    for slice in &time_slices {
        if let Err(div) = runner.process_slice(slice) {
            print!("{}", div);
            std::process::exit(1);
        }
    }
    runner.finish();
}

fn parse_args() -> (String, String, String, String) {
    let args: Vec<String> = env::args().collect();
    let mut log_path: Option<String> = None;
    let mut player_name: Option<String> = None;
    let mut tree_name: Option<String> = None;
    let mut config_path = "bt_match.json".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--log" => {
                log_path = args.get(i + 1).cloned();
                i += 2;
            }
            "--player" => {
                player_name = args.get(i + 1).cloned();
                i += 2;
            }
            "--tree" => {
                tree_name = args.get(i + 1).cloned();
                i += 2;
            }
            "--config" => {
                config_path = args.get(i + 1).cloned().unwrap_or(config_path);
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    const USAGE: &str = "Usage: bt_match --log <path> --player <name> --tree <tree>";
    let log_path = log_path.unwrap_or_else(|| {
        eprintln!("{}", USAGE);
        std::process::exit(2);
    });
    let player_name = player_name.unwrap_or_else(|| {
        eprintln!("{}", USAGE);
        std::process::exit(2);
    });
    let tree_name = tree_name.unwrap_or_else(|| {
        eprintln!("{}", USAGE);
        std::process::exit(2);
    });
    (log_path, player_name, tree_name, config_path)
}

fn load_config(path: &str) -> HashSet<String> {
    let config: BtMatchConfig = fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    let ignored: HashSet<String> = config
        .ignore
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect();
    if !ignored.is_empty() {
        println!("Ignoring skills: {:?}", ignored);
    }
    ignored
}

fn load_log(log_path: &str, player_name: &str, tree_name: &str) -> (ExplainerPage, String) {
    let json = fs::read_to_string(log_path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", log_path, e);
        std::process::exit(2);
    });
    let page: ExplainerPage = serde_json::from_str(&json).unwrap_or_else(|e| {
        eprintln!("Failed to parse log JSON: {}", e);
        std::process::exit(2);
    });
    let (log_me, log_you) = parse_me_and_you(&page);
    let opponent_name = if player_name == log_me { log_you } else { log_me };
    println!(
        "Analyzing {} vs {} using tree '{}'",
        player_name, opponent_name, tree_name
    );
    (page, opponent_name)
}
