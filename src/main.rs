mod utils;

extern crate ring;
extern crate base64;

use std::str;
use base64::{Engine as _, engine::general_purpose};
use clap::Parser;

use std::process;
use std::thread;
use pbr::ProgressBar;
use ring::hmac;

// Define global variables
// const HMAC_ALG: &'static hmac::Algorithm = &hmac::HMAC_SHA256;

// Define a struct for storing session data
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(default_value = "decode", help = "The mode you want to use")]
    mode: String,

    #[arg(short, long, default_value = "default.txt", help = "The wordlist you wanna use while brute-forcing the session")]
    wordlist: String,

    #[arg(short, long, default_value = "jwt", help = "The framework used to create the session")]
    framework: String,

    #[arg(short, long, help = "The session you want to brute-force")]
    session: String,

    #[arg(long, short, help = "Enable progress bar ( that might affect the performance )")]
    verbose: bool,

    #[arg(long, help = "Disable printing anything rather than the results")]
    silent: bool,
}

fn main() {
    let args = Cli::parse();

    if args.silent == false {
        utils::banner::print_banner();
    }

    if args.mode.to_lowercase() == "decode" {
        if args.framework.to_lowercase() == "flask" {
            if args.session.contains(".") {
                let session_string: Vec<String> = args.session.split('.').map(|s| s.to_string()).collect();
                let decoded_session_string = general_purpose::STANDARD_NO_PAD.decode(session_string[0].clone()).unwrap();
                
                let s = match str::from_utf8(&decoded_session_string) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            
                let session_data = utils::read::read_json_session(&s);
            
                match session_data {
                    Ok(data) => println!("Session data: {:?}", data),
                    Err(e) => println!("Error: {:?}", e),
                }
            } else {
                // let session_string = "eyJkYXRhIjp7InVzZXJuYW1lIjoiRGVtb24xQSIsImVtYWlsIjoiZGVtb25hQGV4YW1wbGUuY29tIn19";
                let decoded_session_string = general_purpose::STANDARD.decode(args.session).unwrap();
                
                let s = match str::from_utf8(&decoded_session_string) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            
                let session_data = utils::read::read_json_session(&s);
            
                match session_data {
                    Ok(data) => println!("Session data: {:?}", data),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
    } else if args.mode.to_lowercase() == "unsign"  {
        if args.framework.to_lowercase() == "flask" {
            let lines = utils::read::read_file_lines(&args.wordlist).unwrap();
            let count = lines.iter().count() as u64;
            let count = count.clone();

            if args.verbose {
                let mut pb = ProgressBar::new(count);
                let mut counter = 0 as u64;
                let session = args.session.clone();

                for line in lines {
                    pb.inc();
                    counter += 1;
                    // Use default salt for flask sessions
                    let _t = match utils::decode::decode_flask_cookie(&session, &line, "cookie-session", hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY) {
                        Ok(secret) => {
                            println!("\nYou can unsign your flask session using the key: {}", secret);
                            process::exit(0);
                        }

                        Err(_) => {
                            if counter == count {
                                println!("\nSessionizer couldn't find the secret key for your session\nConsider trying other wordlists, or double check the framework");
                                process::exit(0);
                            }
                        }
                    };
                }
            } else {
                let mut counter = 0 as u64;

                for line in lines {
                    counter += 1;
                    // Use default salt for flask sessions
                    let session = args.session.clone();
                    let _t = thread::spawn(move || {
                        let _t = match utils::decode::decode_flask_cookie(&session, &line, "cookie-session", hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY) {
                            Ok(secret) => {
                                println!("You can unsign your flask session using the key: {}", secret);
                                process::exit(0);
                            }

                            Err(_) => {
                                if counter == count {
                                    println!("Sessionizer couldn't find the secret key for your session\nConsider trying other wordlists, or double check the framework");
                                    process::exit(0);
                                }
                            }
                        };
                    });
                }
            }
        } else if args.framework.to_lowercase() == "jwt" {
            let lines = utils::read::read_file_lines(&args.wordlist).unwrap();

            if args.verbose {
                let mut pb = ProgressBar::new(lines.iter().count() as u64);

                for line in lines {
                    pb.inc();
    
                    let _t = match utils::decode::decode_jwt(&args.session, &line) {
                        Ok(_) => {
                            println!("\nSessionizer found the secret key for your JWT session");
                            println!("You can unsign your JWT session using the key: {}", line);
                            process::exit(0);
                        },
    
                        Err(_) => {},
                    };

                println!("\nSessionizer couldn't find the secret key for your session");
                println!("Consider trying other wordlists, or double check the framework");
                }
            } else {
                let count = lines.iter().count() as u64;
                let count = count.clone();
                let mut counter = 0 as u64;

                for line in lines {
                    counter += 1;
                    let session = args.session.clone();
                    let _thread = thread::spawn(move || {
                        let _t = match utils::decode::decode_jwt(&session, &line) {
                            Ok(secret) => {
                                println!("You can unsign your JWT session using the key: {}", secret);
                                process::exit(0);
                            },
        
                            Err(_) => {
                                if counter == count {
                                    println!("Sessionizer couldn't find the secret key for your session\nConsider trying other wordlists, or double check the framework");
                                    process::exit(0);
                                }
                            },
                        };
                    });
                }
            }
        }
    } else {
        println!("Unsupported mode: {}\nOnly supported modes are decode and unsign", args.mode);
    }
}

// cargo run -- unsign --session eyJ0ZXN0IjoidGVzdGluZyJ9.Y-sl7Q.DZ0kEaeyyJh3goHuZDjynPCEYbg --wordlist hello.txt