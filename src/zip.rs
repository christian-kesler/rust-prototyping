use clap::{App, SubCommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;
use zip::read::ZipFile;
use zip::ZipArchive;
use std::io::{self, copy};

fn main() {
    let matches = App::new("mytool")
        .subcommand(SubCommand::with_name("sort")
            .about("Sort files in a directory"))
        .subcommand(SubCommand::with_name("setup")
            .about("Open predefined browser tabs"))
        .subcommand(SubCommand::with_name("rocket")
            .about("Simulate rocket physics"))
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

                        if file_type == "zip" {
                            // extract .zip file
                            let file = fs::File::open(&path).unwrap();
                            let mut archive = ZipArchive::new(file).unwrap();
                            for i in 0..archive.len() {
                                let mut file = archive.by_index(i).unwrap();
                                let outpath = format!("{}/{}", dir, file.name());
                                if &*file.name() == "__MACOSX/" {
                                    continue;
                                }
                                if file.name().ends_with('/') {
                                    fs::create_dir_all(&outpath).unwrap();
                                } else {
                                    if let Some(p) = outpath.parent() {
                                        if !p.exists() {
                                            fs::create_dir_all(&p).unwrap();
                                        }
                                    }
                                    let mut outfile = fs::File::create(&outpath).unwrap();
                                    copy(&mut file, &mut outfile).unwrap();
                                }
                                // Get and Set permissions
                                #[cfg(unix)]
                                {
                                    use std::os::unix::fs::PermissionsExt;
                                    if let Some(mode) = file.unix_mode() {
                                        fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                                    }
                                }
                            }
                        } else {
                            let new_dir = format!("{}/{}", dir, file_type);
                            match fs::create_dir(&new_dir) {
                                Ok(_) => {}
                                Err(e) => println!("Error creating directory: {:?}", e)
                            }

                            let new_path = format!("{}/{}", new_dir, path.file_name().unwrap().to_str().unwrap());
                            match fs::rename(&path, &new_path) {
                                Ok(_) => {}
                                Err(e) => println!("Error moving file: {:?}", e),
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Error reading directory: {:?}", e),
        }
    }
}














    // } else if let Some(_matches) = matches.subcommand_matches("setup") {

    //     Command::new("firefox")
    //         .arg("--new-tab")
    //         .arg("https://gmail.com")
    //         .spawn()
    //         .expect("Failed to open firefox and navigate to gmail");
    //     Command::new("firefox")
    //         .arg("--new-tab")
    //         .arg("https://google.com")
    //         .spawn()
    //         .expect("Failed to open firefox and navigate to google");
    //     Command::new("firefox")
    //         .arg("--new-tab")
    //         .arg("https://youtube.com")
    //         .spawn()
    //         .expect("Failed to open firefox and navigate to youtube");

    // } else if let Some(_matches) = matches.subcommand_matches("rocket") {

    //     // Initial state
    //     let mut position = vec3(0.0, 0.0, 0.0);
    //     let mut velocity = vec3(0.0, 0.0, 0.0);
    //     let mut mass = 100.0; // kg
    //     let mut thrust = 100.0; // N
    //     let mut time = 0.0; // sec
    //     let mut dt = 0.1; // sec
    
    //     // Simulation loop
    //     while !is_orbital(position.z, velocity.z) {
    //         let acceleration = calculate_acceleration(thrust, mass, position);
    //         velocity += acceleration * dt;
    //         position += velocity * dt;
    //         mass -= dt * calculate_mass_flow(thrust);
    //         time += dt;
    //     }
    
    //     // Output final state
    //     println!("Orbit achieved in {} seconds!", time);
    //     println!("Final altitude: {} m", position.z);
    //     println!("Final velocity: {:.2} m/s", velocity.magnitude());
    //     println!("Final orbital parameters: {:?}", calculate_orbital_parameters(position, velocity));
        
    //     fn is_orbital(altitude: f64, velocity: f64) -> bool {
    //         altitude > 200_000.0 && velocity > 7_800.0
    //     }
        
    //     fn calculate_acceleration(thrust: f64, mass: f64, position: vec3) -> vec3 {
    //         // Add code to calculate the acceleration due to thrust and gravity
    //     }
        
    //     fn calculate_mass_flow(thrust: f64) -> f64 {
    //         // Add code to calculate the mass flow rate
    //     }
        
    //     fn calculate_orbital_parameters(position: vec3, velocity: vec3) -> OrbitalParameters {
    //         // Add code to calculate the orbital parameters from position and velocity
    //     }

//     }
// }
