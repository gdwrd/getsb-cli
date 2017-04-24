pub fn invalid_url_err() {
    println!("error: The following parameter is invalid:");
    println!("\t<URL>");
    println!("\r\nUSAGE:");
    println!("\tgetsb [OPTIONS] <METHOD> <URL> [SUBCOMMAND]");
    println!("For more information try --help");
}

pub fn invalid_response() {
    println!("error: Something went wrong:");
    println!("\r\nUSAGE:");
    println!("\tgetsb [OPTIONS] <METHOD> <URL> [SUBCOMMAND]");
    println!("For more information try --help");
}

pub fn invalid_file_url() {
    println!("error: The request-file is not found:");
    println!("\r\nUSAGE:");
    println!("\tgetsb [OPTIONS] <METHOD> <URL> [SUBCOMMAND]");
    println!("For more information try --help");
}

pub fn invalid_file_read() {
    println!("error: Can't read from file:");
    println!("\r\nUSAGE:");
    println!("\tgetsb [OPTIONS] <METHOD> <URL> [SUBCOMMAND]");
    println!("For more information try --help");
}

pub fn invalid_json() {
    println!("error: The file structure is invalid:");
    println!("\r\nUSAGE:");
    println!("\tgetsb [OPTIONS] <METHOD> <URL> [SUBCOMMAND]");
    println!("For more information try --help");
}

pub fn invalid_save_path() {
    println!("error: The to-file path is invalid:");
    println!("\r\nUSAGE:");
    println!("\tgetsb [OPTIONS] <METHOD> <URL> [SUBCOMMAND]");
    println!("For more information try --help");
}

pub fn cant_save_response() {
    println!("error: The Response to file can't be saved");
    println!("\r\nUSAGE:");
    println!("\tgetsb [OPTIONS] <METHOD> <URL> [SUBCOMMAND]");
    println!("For more information try --help");
}
