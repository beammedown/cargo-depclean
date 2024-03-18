use clap::Parser;
use std::{fs, path::Path};

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

    println!("{}", str_dir);

    if dbg!(contains_cargo_toml(str_dir)) {
        let res = check(str_dir);
        if res.is_err() {
            println!("Error checking dependencies");
            println!("{:?}", res);
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
                let res = fs::write(format!("{}/Cargo.toml", str_dir), new_content);
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
    Path::new(path).join("Cargo.toml").exists()
}

fn check(dir: &str) -> Result<Vec<String>, ()>{
    let mut dependencies = Vec::new();
    let cargo_content = fs::read_to_string(format!("{}/Cargo.toml", dir));
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

    let folder = fs::read_dir(format!("{}/src", dir));
    
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

            if dependencies.iter().any(|x| line.contains(x)) {
                for item in dependencies.iter() {
                    if line.contains(item) && !dependencies_in_files.contains(item) {
                        dependencies_in_files.push(item.clone());
                    }
                }
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
