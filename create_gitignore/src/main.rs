extern crate reqwest;

static base_url: &str = "https://www.toptal.com/developers/gitignore/api/";

fn main() {
    run();
}

fn run() {
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
    fetch_gitignore(values);
}

fn fetch_gitignore(mut values: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

    let joined = values.join(",");
    let url = base_url.to_owned() + &joined.to_string();


    let response = reqwest::get(&url)?.text()?;

    println!("{}", response);
    Ok(())
}
