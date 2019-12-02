use std::env;
use std::fs;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn init_rust(year: u32, day: u32) -> Result<String> {
    let day_dir = format!("../{}/d{}", year, day);

    Command::new("cargo")
        .args(&["new", &day_dir, "--vcs", "none"])
        .output()
        .expect("Failed to run cargo new");

    let main_path = format!("{}/src/main.rs", day_dir);

    let rust_template = include_str!("../rust_template");

    fs::write(main_path, rust_template)?;

    return Ok(day_dir);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let cmd = &args[1];

    if cmd == "help" {
        println!("usage: [lang] [year] [day]");
        return Ok(());
    }

    let lang = cmd;
    let year: u32 = (&args)[2].parse()?;
    let day: u32 = (&args)[3].parse()?;

    let cookie = fs::read_to_string("./cookie")?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = reqwest::blocking::Client::new();
    let res = client
        .request(reqwest::Method::GET, url)
        .header("cookie", cookie)
        .send()?;

    if !res.status().is_success() {
        panic!("There was an error, please check the cookie string");
    }

    let dir = match lang.as_str() {
        "rust" => init_rust(year, day)?,
        _ => {
            panic!("{} not implemented", lang)
        }
    };

    let path = format!("{}/input", dir);
    println!("writing input to {}", path);

    fs::write(&path, res.text()?)?;
    fs::write(format!("{}.test", &path), "")?;

    Ok(())
}
