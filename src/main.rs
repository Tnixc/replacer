use std::fs;
#[derive(Debug)]
struct Replacement {
    pub from: String,
    pub to: String,
}
#[tokio::main]
async fn main() {
    let mut replacements: Vec<Replacement> = Vec::new();
    let reps = fs::read_to_string("./config.toml").unwrap();
    for z in reps.split("\n"){
        if !z.starts_with("#") && z != "" {
            let x = z.split("#").collect::<Vec<&str>>()[0];
            let y: Vec<&str> = x.split("=").collect();
            replacements.push(
                Replacement {
                    from: y[0].trim().to_string(),
                    to: y[1].replace("\"", "").trim().to_string(),
                }
            );
        }
    }
    // for thing in replacements {
    //     print!("{:?}", thing.from);
    //     println!(" -> {:?}", thing.to);
    // }
    // for thing in fs::read_dir("./").await.unwrap() {

    let paths = std::fs::read_dir("./").unwrap();
    for path in paths {
        println!("{}", path.unwrap())
    }
}
