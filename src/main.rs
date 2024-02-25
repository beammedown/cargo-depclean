use clap::Parser;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The folder of the rust project. Defaults to ./
    #[arg(short, long, default_value_t = String::from("./"))]
    user_dir: String,
}

fn main() {
    let args = Args::parse();
    let dir = args.user_dir;



    let str_dir = dir.as_str();


    if contains_cargo_toml(str_dir) {
        let res = check();
        if res.is_err() {
            println!("Error checking dependencies");
            return;
        }

        let res = res.unwrap();

        if res.len() > 0 {
            println!("Dependencies not found in files: {:?}", res);
            println!("Do you want to remove them from Cargo.toml? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "y" {
                let cargo_content = fs::read_to_string("Cargo.toml");
                if cargo_content.is_err() {
                    println!("Error reading Cargo.toml");
                    return;
                }
                let cargo_content = cargo_content.unwrap();

                let mut new_content = String::new();
                let mut is_dependecy = false;

                for line in cargo_content.lines() {
                    if line.starts_with("[dependencies]") {
                        is_dependecy = true;
                        new_content.push_str(line);
                        new_content.push_str("\n");
                        continue;
                    }
                    if is_dependecy {
                        if line.starts_with("[") {
                            is_dependecy = false;
                            new_content.push_str(line);
                            new_content.push_str("\n");
                            continue;
                        }
                        if line == "" {
                            continue;
                        }
                        if res.iter().any(|x| line.starts_with(x)) {
                            println!("Removing dependency: {}", line);
                        } else {
                            new_content.push_str(line);
                            new_content.push_str("\n");
                        }
                    } else {
                        new_content.push_str(line);
                        new_content.push_str("\n");
                    }
                }
                let res = fs::write("Cargo.toml", new_content);
                if res.is_err() {
                    println!("Error writing to Cargo.toml");
                    return;
                }
                println!("Dependencies removed from Cargo.toml");
            } else {
                println!("Dependencies not removed from Cargo.toml");
            }
        } else {
            println!("All dependencies found in files\nYou don't need to remove any dependency from Cargo.toml");
        }
    } else {
        println!("Cargo.toml not found");
    }
}


fn contains_cargo_toml(path: &str) -> bool {
    let path = format!("{}/Cargo.toml", path);
    fs::metadata(path).is_ok()
}

fn check() -> Result<Vec<String>, ()>{
    let mut dependencies = Vec::new();
    let cargo_content = fs::read_to_string("Cargo.toml");
    if cargo_content.is_err() {
        println!("Error reading Cargo.toml");
        return Err(());
    }
    let cargo_content = cargo_content.unwrap();

    let mut  is_dependecy = false;

    for line in cargo_content.lines() {
        if line.starts_with("[dependencies]") {
            is_dependecy = true;
            continue;
        }
        if is_dependecy {
            if line.starts_with("[") {
                break;
            }
            if line == "" {
                continue;
            }
            let line = line.trim().split(" ").collect::<Vec<&str>>();
            dependencies.push(line[0].to_string().replace("-", "_"));
        }
    }

    println!("Dependencies: {:?}", dependencies);



    let mut dependencies_in_files: Vec<String> = Vec::new();

    let folder = fs::read_dir("src");
    
    if folder.is_err() {
        println!("Error reading folder");
        return Err(());
    }
    let folder = folder.unwrap();

    let mut removable = Vec::new();
    for file in folder {        
        if file.is_err() {
            println!("Error reading folder");
            return Err(());
        }
        let file = file.unwrap().path();
        
        println!("Checking file: {:?}", &file);

        let file_content = fs::read_to_string(&file);

        if file_content.is_err() {
            println!("Error reading file");
            return Err(());
            
        }
        let file_content = file_content.unwrap();

        for line in file_content.lines() {
            if line.starts_with("use") {
                let line = line.trim().split(" ").collect::<Vec<&str>>();
                if line.len() < 1 {
                    continue;
                }
                let true_dep: Vec<&str> = line[1].split("::").collect();
                dependencies_in_files.push(true_dep[0].to_string().replace(";", ""));
            }
        }
        println!("Dependencies in file: {:?}", dependencies_in_files);


    }
    for dep in &dependencies {
        if !dependencies_in_files.contains(dep) {
            println!("Dependency {} not found in file", dep);
            removable.push(dep.clone());
        }
    }
    Ok(removable.clone())

}
