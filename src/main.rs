#[macro_use]
extern crate clap;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod instruction;

use std::process::exit;
use std::io::{stdin, Read};
use clap::App;

fn main() {
    let yaml = load_yaml!("wurl-tools.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let ping = value_t_or_exit!(matches, "print", String);
    let pong = value_t_or_exit!(matches, "pong", String);
    let message = value_t_or_exit!(matches, "message", String);
    let close = value_t_or_exit!(matches, "close", String);
    //let headers = values_t!(matches, "headers", String).unwrap_or(Vec::new());
    //let method = value_t!(matches, "method", Method).unwrap_or(Method::Get);
    //let print_headers = matches.is_present("head");
    /*
    let mut data = value_t!(matches, "data", String).ok();

    // Read stdin when given --data -
    if let Some(read_data) = data.clone() {
        if read_data == "-" {
            let mut buffer = String::new();
            let stdin = stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buffer).ok();
            data = Some(buffer);
        }
    }

    let request = build_request(url, method, headers, data);

    match fetch(request, print_headers) {
        Ok(mut response) => {
            if print_headers {
                eprintln!("Authentication response");
                eprintln!("---");
                eprintln!("{} {}", response.version(), response.status());
                eprintln!("{}", response.headers());
            }

            let cookies = response
                .headers_mut()
                .get::<SetCookie>()
                .unwrap_or_else(|| {
                    eprintln!("No Set-Cookie header present");
                    exit(1);
                });

            let mut cookie_values = Vec::new();
            for cookie in cookies.iter() {
                // Get only key=value part of cookie, not the metadata
                let split = cookie.split(';').collect::<Vec<&str>>();
                if let Some(header) = split.first() {
                    cookie_values.push(header.clone());
                }
            }

            print!("Cookie: {}", cookie_values.join("; "));
        }
        Err(error) => eprintln!("An error occured while fetching: {}", error),
    }
    */
}

