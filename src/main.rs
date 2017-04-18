extern crate clap;

use clap::{Arg, App, SubCommand};
use std::collections::HashMap;

mod sender;
mod parser;
mod errors;

pub struct Request {
    body: String,
    headers: HashMap<String, String>,
    method: String,
    url: String,
}

fn main() {
    let matches = App::new("Getsb")
        .version("0.1.0")
        .author("Nazarii Sheremet. <nazarii.sheremet@gmail.com>")
        .about("Does awesome things")
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
        .subcommand(SubCommand::with_name("test")
                        .about("controls testing features")
                        .version("1.3")
                        .author("Someone E. <someone_else@other.com>")
                        .arg(Arg::with_name("debug")
                                 .short("d")
                                 .help("print debug information verbosely")))
        .get_matches();



    if let Some(file) = matches.value_of("request-file") {
        proccess_file(file.to_string())
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

        let request = Request {
            method: method,
            url: url,
            body: body,
            headers: header,
        };

        sender::send(request);
    }
}

fn proccess_file(filename: String) {
    println!("{:?}", filename);
}
