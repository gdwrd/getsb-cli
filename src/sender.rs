extern crate knock;

use std::process;
use Request;
use errors;
use self::knock::*;
use std::fs::File;
use std::io::Write;
use self::knock::response::Response;

pub fn send(request: self::Request, path: &str) {
    let mut http = match HTTP::new(&request.url) {
        Ok(http) => http,
        Err(_) => {
            errors::invalid_url_err();
            process::exit(0);
        }
    };

    let result = http.request(&request.method)
        .body_as_str(&request.body)
        .header(request.headers)
        .send();

    match result {
        Ok(response) => {
            if path.is_empty() {
                print_response(&response);
            } else {
                save_to_file(&response, path);
            }
        }
        Err(_) => {
            errors::invalid_response();
            process::exit(0);
        }
    }
}

fn print_response(response: &Response) {
    println!("Status: {}\r\n", response.status);

    for (key, val) in response.header.iter() {
        println!("{}: {}", key, val);
    }

    println!("\r\n\r\n{}", response.body);
}

fn save_to_file(response: &Response, path: &str) {
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => {
            errors::invalid_save_path();
            process::exit(0);
        }
    };

    let content = response.as_str();

    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Response was saved to file: {}", path),
        Err(_) => {
            errors::cant_save_response();
            process::exit(0);
        }
    }
}
