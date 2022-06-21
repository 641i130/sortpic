use std::{fs,env,io};
use image::image_dimensions;

fn main() {
    // Stupid confirm var
    let mut yes = String::new();
    // Handle arguments:
    let args: Vec<String> = env::args().collect();
    if &args[1] == "-h" {
        println!("Usage: sortpic [directory of images]");
        std::process::exit(1);
    }
    match fs::read_dir(&args[1]) {
        Ok(_) => println!("Reading directory {}",&args[1]),
        Err(err) => {
            println!("{}",err);
            println!("Not a directory!");
            println!("Usage: sortpic [directory of images]");
            std::process::exit(1);
        },
    }

    let files = fs::read_dir(&args[1]).unwrap();
    // Make directory for portrait, landscape and square images
    for f in ["portrait","landscape","squarelike"] {
        let slash = "/";
        let dir = [&args[1],slash,f].join("");
        println!("Created directory : \n{}",dir);
        fs::create_dir_all(dir).unwrap();
    }
    let mut count: i64 = 0;
    for f in files {
    let name: String = f
            .as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if name.contains("png") || name.contains("jpg") {
            count+=1;
        }
    }
    println!("Moving {} images based off of dimensions.",count);
    println!("Continue?");
    io::stdin()
        .read_line(&mut yes)
        .expect("Failed.");
    // Read dir one more time
    let files = fs::read_dir(&args[1]).unwrap();
    for f in files {
        let name: String = f
            .as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if name.contains("png") || name.contains("jpg") {
            let lol = image_dimensions(f.as_ref().unwrap().path()).unwrap();
            let width: i32 = lol.0 as i32;
            let height: i32 = lol.1 as i32;
            let ratio: i32 = width-height;
            if ratio > 300  {
                // Height is smaller than the width (probably)!
                let out = [&args[1],"/landscape/",&name].join("");
                println!("{}",out);
                fs::copy(f.unwrap().path(), out).unwrap();
            } else if ratio == 0 || (ratio < 300 && ratio > 300) {
                // Square-ish images
                let out = [&args[1],"/squarelike/",&name].join("");
                println!("{}",out);
                fs::copy(f.unwrap().path(), out).unwrap();
            } else {
                // NEGATIVE IMAGE! PORTAIT
                let out = [&args[1],"/portrait/",&name].join("");
                println!("{}",out);
                fs::copy(f.unwrap().path(), out).unwrap();
            }
            println!("{}",name);
            println!("{} - {} = {}",width,height,ratio);
        } else {
            println!("FILE TYPE NOT ALLOWED");
        }
    }
    println!("DONE!");
}
