extern crate serde_json;

use std::collections::HashMap;
use std::process;
use std::fs::File;
use std::io::Read;
use errors;
use Request;
use self::serde_json::Value;

// Parse json file
pub fn parse_file(filename: &str) -> Request {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            errors::invalid_file_url();
            process::exit(0);
        }
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(_) => {
            errors::invalid_file_read();
            process::exit(0);
        }
    }

    let json: Value = match serde_json::from_str(&buffer) {
        Ok(json) => json,
        Err(_) => {
            errors::invalid_json();
            process::exit(0);
        }
    };

    validate_json(json.clone());

    let mut data: HashMap<String, String> = HashMap::new();
    let mut i = 0;

    loop {
        if json["headers"][i] != Value::Null {
            let mut tmp = json["headers"][i].to_string();
            tmp = remove_quotes(tmp);
            let item: Vec<&str> = tmp.split(":").collect();
            data.insert(item[0].to_string(), item[1].to_string());
        } else {
            break;
        }

        i += 1;
    }

    Request {
        method: remove_quotes(json["method"].to_string()),
        url: remove_quotes(json["url"].to_string()),
        body: json["body"].to_string(),
        headers: data,
    }
}

// Checking the required fields in json
fn validate_json(json: serde_json::Value) {
    if json["method"] == Value::Null || json["url"] == Value::Null ||
       json["body"] == Value::Null || json["headers"] == Value::Null {
        errors::invalid_json();
        process::exit(0);
    }
}

// Removing quotes from the beginning and end of the string
fn remove_quotes(mut str: String) -> String {
    str.pop();
    str.remove(0);
    str.to_string()
}
