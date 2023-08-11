
use serde_json::Value;

use std::fs::File;
use std::io::Read;

pub fn read_config_json(path: &str) -> Value{
    let mut file = File::open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    //Deserializes json data struct
    let data = serde_json::from_str(content.as_str());
    match data {
        Ok(v) => {

        }
        Err(e) => {
            
        }
    }
}