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
