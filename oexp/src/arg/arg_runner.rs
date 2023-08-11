use crate::arg::config_json;
use crate::error::error;
use std::path::Path;
use std::env;

const JSON_FILE_NAME: &str = "data/oexp_exps.json";

pub fn explain(ecode: &str) -> bool {
    let json_path: std::path::PathBuf;

    let binary_path = env::current_exe();
    match binary_path {
        Ok(bin_path) => {
            let bin_dir = bin_path.parent();
            match bin_dir {
                None => {
                    error::error("Ee005", "error while adding paths");
                },
                _ => {

                }
            }
            json_path = bin_dir.unwrap().join(JSON_FILE_NAME);
        }
        Err(e) => {
            error::error("Ee006", format!("could not get current path: {}", e).as_str());
            return false;
        }
    }

    if Path::new(&json_path).exists() == false {
        error::error("Ee003", "could not find error file");
        return false;
    }

    else {
        let json: serde_json::Value = config_json::read_config_json(
                                                                        json_path.to_str().expect("Failed to convert to &str")
                                                                    );

        if let Some(error) = json.get(ecode) {
            if let Some(exp) = error.as_str() {
                println!("{}", exp);
            } else {
                println!("explination of ecode {} is not a string", ecode);
            }
        } else {
            error::error("Ee004", "error does not exits");
        }

        return true;
    }
}