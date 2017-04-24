extern crate clap;

use clap::{Arg, App};
use std::collections::HashMap;

mod sender;
mod parser;
mod errors;

// HTTP Request structure
#[derive(Debug)]
pub struct Request {
    body: String,
    headers: HashMap<String, String>,
    method: String,
    url: String,
}

fn main() {
    // Validate and set args using clap crate for building CLI
    let matches = App::new("Getsb")
        .version("0.1.0")
        .author("Nazarii Sheremet. <nazarii.sheremet@gmail.com>")
        .about("Getsb is a command line tool for sending HTTP request.")
        .arg(Arg::with_name("request-file")
                 .short("r")
                 .long("request-file")
                 .value_name("FILE")
                 .help("Sets a custom request file")
                 .takes_value(true))
        .arg(Arg::with_name("METHOD")
                 .required_unless("request-file")
                 .possible_values(&["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
                 .help("Sets the METHOD of request")
                 .index(1))
        .arg(Arg::with_name("URL")
                 .required_unless("request-file")
                 .help("Sets the URL of request")
                 .index(2))
        .arg(Arg::with_name("body")
                 .short("b")
                 .long("body")
                 .value_name("BODY")
                 .help("Sets the body of request")
                 .takes_value(true))
        .arg(Arg::with_name("headers")
                 .short("h")
                 .long("headers")
                 .multiple(true)
                 .value_name("HEADERS")
                 .help("Sets the headers of request")
                 .takes_value(true))
        .arg(Arg::with_name("to-file")
                 .short("f")
                 .long("to-file")
                 .value_name("FILE")
                 .help("Save all response to file")
                 .takes_value(true))
        .get_matches();

    let mut to_file = String::new();

    // Checking first request-file
    if let Some(file) = matches.value_of("request-file") {
        if let Some(val) = matches.value_of("to-file") {
            to_file = val.to_string();
        }

        // Parsing file for creating request
        proccess_file(file, to_file);
    } else {
        let mut method = String::new();
        let mut url = String::new();
        let mut body = String::new();
        let mut header = HashMap::new();

        if let Some(headers) = matches.values_of("headers") {
            for h in headers {
                let line: Vec<&str> = h.split(":").collect();
                header.insert(line[0].trim().to_string(), line[1].trim().to_string());
            }
        }

        if let Some(val) = matches.value_of("body") {
            body = val.to_string();
        }

        if let Some(val) = matches.value_of("URL") {
            url = val.to_string();
        }

        if let Some(val) = matches.value_of("METHOD") {
            method = val.to_string();
        }

        if let Some(val) = matches.value_of("to-file") {
            to_file = val.to_string();
        }

        let request = Request {
            method: method,
            url: url,
            body: body,
            headers: header,
        };

        sender::send(request, to_file);
    }
}

// Working with request info from file
fn proccess_file(filename: &str, to_filename: String) {
    let request = parser::parse_file(filename);
    sender::send(request, to_filename);
}
