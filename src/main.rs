use clap::{App, SubCommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;

fn main() {
    let matches = App::new("mytool")
        .subcommand(SubCommand::with_name("sort")
            .about("Sort files in a directory"))
        .subcommand(SubCommand::with_name("setup")
            .about("Open predefined browser tabs"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("sort") {
        let mut dir = env::home_dir().unwrap();
        dir.push("Downloads");
        let dir = dir.to_str().unwrap();

        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() {
                        let file_type = path.extension().unwrap().to_str().unwrap();

                        let new_dir = format!("{}/{}", dir, file_type);
                        match fs::create_dir(&new_dir) {
                            Ok(_) => {}
                            Err(e) => println!("Error creating directory: {:?}", e),
                        }

                        let new_path = format!("{}/{}", new_dir, path.file_name().unwrap().to_str().unwrap());
                        match fs::rename(&path, &new_path) {
                            Ok(_) => {}
                            Err(e) => println!("Error moving file: {:?}", e),
                        }
                    }
                }
            }
            Err(e) => println!("Error reading directory: {:?}", e),
        }
    } else if let Some(_matches) = matches.subcommand_matches("setup") {

        Command::new("firefox")
            .arg("--new-tab")
            .arg("https://gmail.com")
            .spawn()
            .expect("Failed to open firefox and navigate to gmail");
        Command::new("firefox")
            .arg("--new-tab")
            .arg("https://google.com")
            .spawn()
            .expect("Failed to open firefox and navigate to google");
        Command::new("firefox")
            .arg("--new-tab")
            .arg("https://youtube.com")
            .spawn()
            .expect("Failed to open firefox and navigate to youtube");

    }
}



                        // use clap::{App, SubCommand};
// use std::fs;
// use std::path::{Path, PathBuf};
// use std::env;

// fn main() {
//     let matches = App::new("mytool")
//         .subcommand(SubCommand::with_name("sort")
//             .about("Sort files in a directory"))
//         .get_matches();

//     if let Some(_matches) = matches.subcommand_matches("sort") {

//         let mut dir = env::home_dir().unwrap();
//         dir.push("Downloads");
//         let dir = dir.to_str().unwrap();

//         match fs::read_dir(dir) {
//             Ok(entries) => {
//                 for entry in entries {
//                     let entry = entry.unwrap();
//                     let path = entry.path();
//                     let file_type = path.extension().unwrap().to_str().unwrap();

//                     let new_dir = format!("{}/{}", dir, file_type);

//                     match fs::create_dir(&new_dir) {
//                         Ok(_) => {}
//                         Err(e) => println!("Error creating directory: {:?}", e),
//                     }

//                     let new_path = format!("{}/{}", new_dir, path.file_name().unwrap().to_str().unwrap());
//                     match fs::rename(&path, &new_path) {
//                         Ok(_) => {}
//                         Err(e) => println!("Error moving file: {:?}", e),
//                     }
//                 }
//             }
//             Err(e) => println!("Error reading directory: {:?}", e),
//         }
//     }
// }






// use clap::{App, SubCommand};
// use std::fs;
// use std::path::{Path, PathBuf};
// use std::env;
// // use std::process::Command;

// fn main() {
//     let matches = App::new("mytool")
//         .subcommand(SubCommand::with_name("sort")
//             .about("Sort files in a directory"))
//         .get_matches();

//     if let Some(_matches) = matches.subcommand_matches("sort") {

//         let mut dir = env::home_dir().unwrap();
//         dir.push("Downloads");
//         let dir = dir.to_str().unwrap();

//         match fs::read_dir(dir) {
//             Ok(entries) => {
//                 for entry in entries {
//                     let entry = entry.unwrap();
//                     let path = entry.path();
//                     let file_type = path.extension().unwrap().to_str().unwrap();

//                     let new_dir = format!("{}/{}", dir, file_type);

//                     match fs::create_dir(&new_dir) {
//                         Ok(_) => {}
//                         Err(e) => println!("Error creating directory: {:?}", e),
//                     }

//                     let new_path = format!("{}/{}", new_dir, path.file_name().unwrap().to_str().unwrap());
//                     match fs::rename(&path, &new_path) {
//                         Ok(_) => {}
//                         Err(e) => println!("Error moving file: {:?}", e),
//                     }
//                 }
//             }
//             Err(e) => println!("Error reading directory: {:?}", e),
//         }
//     // } else if let Some(_matches) = matches.subcommand_matches("setup") {

//     //     Command::new("brave")
//     //         .arg("--new-tab")
//     //         .arg("https://gmail.com")
//     //         .spawn()
//     //         .expect("Failed to open Brave and navigate to gmail");
//     //     Command::new("brave")
//     //         .arg("--new-tab")
//     //         .arg("https://google.com")
//     //         .spawn()
//     //         .expect("Failed to open Brave and navigate to google");
//     //     Command::new("brave")
//     //         .arg("--new-tab")
//     //         .arg("https://youtube.com")
//     //         .spawn()
//     //         .expect("Failed to open Brave and navigate to youtube");
        
//     }
// }
