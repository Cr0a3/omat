use std::process::Command;
use crate::error::error;
use crate::arg::config_json;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

const COMPILER_NAME: &str = "omatc.exe";
const RELEATIV_CONFIG_PATH: &str = "Opack.json";

const MAINOM_FILE_CONTENT: &str = r#"
use std;

fn main() -> i32 {
    std::out.println("Hello World");
    return 0;
}"#;

fn get_path_to_build_dir(v: &str) -> String {
    return config_json::read_config_json(RELEATIV_CONFIG_PATH)[format!("{}-target", v)].to_string().replace("\"", "");
}

fn get_package_name() -> String {
    return config_json::read_config_json(RELEATIV_CONFIG_PATH)["package-name"].to_string().replace("\"", "");
}

pub fn build(v: &str) -> bool {
    if Path::new("opack.json").exists() == false {
        error::error("Et006", "could not find opack.json file");
        return false;
    }

    let version: &str;

    if v == "debug" {
        version = "--debug";
    }

    else if v == "release" {
        version = "--release";
    }

    else {
        error::error("Et004", format!("unaviable target: {}", v).as_str());
        return false;
    }

    let compiler_path: std::path::PathBuf;

    let binary_path = env::current_exe();
    match binary_path {
        Ok(bin_path) => {
            let bin_dir = bin_path.parent();
            match bin_dir {
                None => {
                    error::error("Et009", "error while adding paths");
                },
                _ => {

                }
            }
            compiler_path = bin_dir.unwrap().join(COMPILER_NAME);
        }
        Err(e) => {
            error::error("Et008", format!("could not get current path: {}", e).as_str());
            return false;
        }
    }

    let status = Command::new(compiler_path)
                                            .arg(version)
                                            .arg("-o")
                                            .arg(format!("{}/{}", get_path_to_build_dir(v), get_package_name()))
                                            .arg("--input")
                                            .arg(format!("{}", config_json::read_config_json(RELEATIV_CONFIG_PATH)["main_path"].to_string().replace("\"", "")))
                                            .status();
    match status {
        Ok(stat) => {
            if stat.success() {
                return true;
            }
            else {
                return false;
            }
        },
        Err(e) => {
            error::error("Et008", format!("could not start the compiler: {}", e).as_str());
            return false;
        },
    }
}

pub fn clean(v: &str) -> bool {
    if Path::new("opack.json").exists() == false {
        error::error("Et006", "could not find opack.json file");
        return false;
    }

    let dir_path: String;

    if v == "debug" {
        dir_path = get_path_to_build_dir(v);
    }

    else if v == "release" {
        dir_path = get_path_to_build_dir(v);
    }

    else {
        error::error("Et004", format!("unaviable target: {}", v).as_str());
        return false;
    }

    //remove dir
    if let Err(err) = fs::remove_dir(dir_path) {
        // Check the specific error to handle different cases
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                return false;
            }
            std::io::ErrorKind::PermissionDenied => {
                error::error("Et005", format!("Permission denied: {:?}", err).as_str());
            }
            _ => {
                return false;
            }
        }
    }

    return true;

}

pub fn new(package_name: &str) {
    let directory_name = format!("{}", package_name);
    let opack_content = format!(
        "{{ \"package-name\": \"{}\", \"debug-target\": \"target/debug\", \"release-target\": \"target/release\", \"main_path\": \"src/main.om\" }}",
        package_name
    );
    
    // Create the root directory with the specified name
    if let Err(err) = fs::create_dir(&directory_name) {
        error::error("Et002", format!("error while creating new package: {}", err).as_str());
        return;
    }

    // Create and write to the opack.json file
    let opack_file_path = format!("{}/opack.json", directory_name);
    let mut opack_file = match File::create(&opack_file_path) {
        Ok(file) => file,
        Err(err) => {
            error::error("Et002", format!("error while creating new package: {}", err).as_str());
            return;
        }
    };
    if let Err(err) = opack_file.write_all(opack_content.as_bytes()) {
        error::error("Et002", format!("error while creating new package: {}", err).as_str());
        return;
    }

    // Create the 'target' and 'src' directories
    let target_dir_path = format!("{}/target", directory_name);
    let src_dir_path = format!("{}/src", directory_name);
    if let Err(err) = fs::create_dir(&target_dir_path) {
        error::error("Et002", format!("error while creating new package: {}", err).as_str());
        return;
    }
    if let Err(err) = fs::create_dir(&src_dir_path) {
        error::error("Et002", format!("error while creating new package: {}", err).as_str());
        return;
    }

    // Create the 'debug' and 'release' directories inside target
    let debug_dir_path = format!("{}/target/release", directory_name);
    let release_dir_path = format!("{}/target/debug", directory_name);
    if let Err(err) = fs::create_dir(&debug_dir_path) {
        error::error("Et002", format!("error while creating new package: {}", err).as_str());
        return;
    }
    if let Err(err) = fs::create_dir(&release_dir_path) {
        error::error("Et002", format!("error while creating new package: {}", err).as_str());
        return;
    }

    // Create and write to the main.om file inside 'src'
    let main_om_file_path = format!("{}/main.om", src_dir_path);
    let mut main_om_file = match File::create(&main_om_file_path) {
        Ok(file) => file,
        Err(err) => {
            error::error("Et002", format!("error while creating new package: {}", err).as_str());
            return;
        }
    };

    if let Err(err) = main_om_file.write_all(MAINOM_FILE_CONTENT.as_bytes()) {
        error::error("Et002", format!("error while creating new package: {}", err).as_str());
        return;
    }
}

pub fn run(v: &str) -> bool {
    if Path::new("opack.json").exists() == false {
        error::error("Et006", "could not find opack.json file");
        return false;
    }

    clean(v);

    let sucess: bool = build(v);

    if sucess == true {
        //launch programm
        let path: String = format!("{}/{}", get_path_to_build_dir(v), get_package_name());
        let output = Command::new(path)
                              .output();

        match output {
            Ok(o) =>  println!("Programm exits with value {}", o.status),
            Err(_) => error::error("Et007", "could not start the application"),
        }
                
        return true;
    }

    else {
        return false;
    }
}