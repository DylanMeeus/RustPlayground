extern crate reqwest;
use std::fs;

static base_url: &str = "https://www.toptal.com/developers/gitignore/api/";

fn main() {
    run();
}

fn run() {
    let args = parse_args();
    let result = fetch_gitignore(args);
    match result {
        Ok(s) => {
            write_gitignore(s);
        },
        Err(e) => ()
    }
}

/// parse_args fetches from the CLI the tools/languages for which we wnat to generate a .gitignore
/// file
fn parse_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    let tail = match args.split_first() {
        Some(r) => Some(r.1),
        None => None
    };

    let values = tail.unwrap().to_vec();
    if values.len() == 0 {
        println!("expected at least one arg");
        std::process::exit(1);
    }
    values
}

/// fetch_gitignore performs a HTTP GET request to toptal to retrieve our git file
fn fetch_gitignore(mut values: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let joined = values.join(",");
    let url = base_url.to_owned() + &joined.to_string();
    let response = reqwest::get(&url)?.text()?;

    Ok(response)
}

/// write_gitignore writes out a .gitignore file to disk. 
/// This overrides any existing .gitignore file in the same directory
fn write_gitignore(gitignore_content: String) -> std::io::Result<()> {
    // todo: check if there's a gitignore file present -> if so, allow forced overwrites
    fs::write(".gitignore", gitignore_content)?;
    Ok(())
}
