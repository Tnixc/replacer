use std::env;
use std::fs;
use std::path::PathBuf;
#[derive(Debug)]
struct Replacement {
    pub from: String,
    pub to: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path/to/target>", args[0]);
        return;
    }
    let query = &args[1];
    let mut replacements: Vec<Replacement> = Vec::new();
    if fs::read_to_string("./config.toml").is_err() {
        println!("config.toml not found");
        return;
    }
    let reps = fs::read_to_string("./config.toml").unwrap();
    for z in reps.split("\n") {
        if !z.starts_with("#") && z != "" {
            let x = z.split("#").collect::<Vec<&str>>()[0];
            let y: Vec<&str> = x.split("=").collect();
            replacements.push(Replacement {
                from: y[0].trim().to_string(),
                to: y[1].replace("\"", "").trim().to_string(),
            });
        }
    }
    if fs::read_dir(query).is_err() {
        println!("{} not found", query);
        return;
    }
    recursive_file(&PathBuf::from(query), &replacements)
}

fn recursive_file(path: &PathBuf, replacements: &Vec<Replacement>) {
    let paths = fs::read_dir(path);
    for path in paths.unwrap() {
        let this = path.unwrap().path();
        if this.is_dir() {
            recursive_file(&this, replacements)
        }
        if this.is_file() {
            op(&this, replacements);
        }
    }
}

fn op(file: &PathBuf, reqs: &Vec<Replacement>) {
    if !file.is_file() {
        panic!("something has gone terribly wrong");
    }
    let mut text = fs::read_to_string(file).unwrap_or_default();
    if text == "" {
        return;
    }
    let mut counta = 0;
    for req in reqs {
        if text.contains(req.from.as_str()) {
            counta += 1;
        }
        text = text.replace(req.from.as_str(), req.to.as_str());
    }
    let _ = fs::write(file, text);
    if counta > 0{
        println!("{:?}, replaced {:?} terms", file, counta)
    }
}
