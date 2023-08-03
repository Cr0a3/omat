use std::process::Command;
use crate::error::error;
use crate::arg::configJson;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

const COMPILER_NAME: &str = "omatc.exe";
const RELEATIV_CONFIG_PATH: &str = "Opack.json";

fn get_path_to_build_dir(v: &str) -> String {
    return configJson::read_config_json(RELEATIV_CONFIG_PATH)[format!("{}-target", v)].to_string();
}

fn get_package_name() -> String {
    return configJson::read_config_json(RELEATIV_CONFIG_PATH)["package-name"].to_string();
}

pub fn build(v: &str) -> bool {
    if Path::new("opack.json").exists() == false {
        error::error("Et006", "could not find opack.json file");
        return false;
    }

    let mut arg = String::new();

    if v == "debug" {
        arg = String::from("-debug");
    }

    else if v == "release" {
        arg = String::from("-release");
    }

    else {
        error::error("Et004", format!("unaviable target: {}", v).as_str());
        return false;
    }

    let binary_path = env::current_exe().expect("Failed to get the current executable path");
    let binary_directory = binary_path.parent().expect("Failed to get the parent directory");
    let compiler_path = binary_directory.join(COMPILER_NAME);

    let status = Command::new(compiler_path)
                                            .arg(arg)
                                            .arg(format!("-o {}/{}", get_path_to_build_dir(v), get_package_name()))
                                            .arg(configJson::read_config_json(RELEATIV_CONFIG_PATH)["main_path"].to_string())
                                            .status()
                                            .expect(
                                                "Failed to start the compiler"
                                            );
    if status.success() {
        return true;
    }
    else {
        return false;
    }
}

pub fn clean(v: &str) -> bool {
    if Path::new("opack.json").exists() == false {
        error::error("Et006", "could not find opack.json file");
        return false;
    }

    let mut dirPath: String = String::new();
    if v == "debug" {
        dirPath = get_path_to_build_dir(v);
    }

    else if v == "release" {
        dirPath = get_path_to_build_dir(v);
    }

    else {
        error::error("Et004", format!("unaviable target: {}", v).as_str());
        return false;
    }

    //remove dir
    if let Err(err) = fs::remove_dir(dirPath) {
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
    let main_om_content = "Hello World!";
    
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
    if let Err(err) = main_om_file.write_all(main_om_content.as_bytes()) {
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
        let output: std::process::Output = Command::new(path)
                              .output().expect(
                                "Failed starting the programm"
                              );

        println!("Programm exits with value {}", output.status);                      
        return true;
    }

    else {
        return false;
    }
}