
use serde_json::Value;

use std::fs::File;
use std::io::Read;

pub fn read_config_json(path: &str) -> Value{
    let mut file = File::open(path).expect("could not open config file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("could not read config file");

    //Deserializes json data struct
    let data: Value  = serde_json::from_str(content.as_str()) .expect("error in json deserializeation of config file");
    return data;
}