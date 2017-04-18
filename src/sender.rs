extern crate knock;

use std::process;
use self::knock::*;
use self::knock::response::Response;
use Request;
use errors;

pub fn send(request: self::Request) {
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
        Ok(response) => print_response(response),
        Err(_) => {
            errors::invalid_response();
            process::exit(0);
        }
    }
}

fn print_response(response: Response) {
    println!("Status: {}\r\n", response.status);

    for (key, val) in response.header.iter() {
        println!("{}: {}", key, val);
    }

    println!("\r\n\r\n{}", response.body);
}
