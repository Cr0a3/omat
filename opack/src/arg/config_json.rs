
use serde_json::Value;

use std::process;
use std::fs::File;
use std::io::Read;
use crate::error::error;

pub fn read_config_json(path: &str) -> Value{
    let file = File::open(path);
    let mut content = String::new();

    match file {
        Ok(mut f) => {
            f.read_to_string(&mut content).expect("err");
        }
        Err(e) => {
            error::error("Et011", format!("could not open json config file: {}", e).as_str());
        },
    }


    //Deserializes json data struct
    let data = serde_json::from_str(content.as_str());
    match data {
        Ok(v) => {
            v
        }
        Err(e) => {
            error::error("Et010", format!("error while deseralizing json data struct: {}", e).as_str());
            process::exit(-1);
        }
    }
}