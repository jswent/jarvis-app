#![allow(unused)]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
mod mods;

fn print_jarvis_text() {
    let file = std::fs::read_to_string("/etc/jarvis/server_ver").expect("could not read file");
    let mut file_temp = file.to_string();
    file_temp.truncate(file_temp.len() - 1);
    println!("JARVIS release {} on Ubuntu 20.04.3 LTS", file_temp); 
}

fn main() {
    //import all arguments given on command line into args vector
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let mut s = &args[1];

        if args.len() >= 2 && (s.eq("version") || s.eq("--version") || s.eq("-v")) {
            if args.len() > 2 {
                s = &args[2];
            }

            if args.len() == 2 {
                let file = std::fs::read_to_string("/etc/jarvis/server_ver").expect("could not read file");
                let mut file_temp = file.to_string();
                file_temp.truncate(file_temp.len() - 1);        
                println!("{}", file_temp);
            }
            else if args.len() >= 3 && (s.eq("change") || s.eq("--change") || s.eq("-c")) {
                if mods::auth::check_root() == true {
                    if args.len() == 3 {
                        panic!("Usage: jarvis version change [NEW_VER]");
                    }
                    else if args.len() == 4 {
                        let mut owned_string: String = args[3].to_owned();
                        let borrowed_string: &str = "\n";
                        owned_string.push_str(borrowed_string);
                        fs::write("/etc/jarvis/server_ver", owned_string).expect("Unable to write to file");
                    }
                }
                else {
                    panic!("Insufficient permissions");
                }
            }
        }
        else if args.len() >= 2 && (s.eq("help") || s.eq("--help") || s.eq("-h")) {
            if args.len() > 2 {
                panic!("specified parameters exceed command. Usage: jarvis --help")
            }

            print_jarvis_text();
            let file = std::fs::read_to_string("/etc/jarvis/help.txt").expect("unable to read file /etc/jarvis/help.txt");
            let mut file_temp = file.to_string();
            file_temp.truncate(file_temp.len() - 1);
            println!("\n{}", file_temp);
            
        }
        else if args.len() >= 2 && (s.eq("pwman") || s.eq("--pwman") || s.eq("-p")) {
            if args.len() == 2 {
                println!("Welcome to JARVIS password manager!");
            }
        }
        else if args.len() >= 2 && (s.eq("auth") || s.eq("--auth") || s.eq("-a")) {
            if args.len() > 2 {
                s = &args[2];
            }
            
            if args.len() == 2 {
                println!("Usage: jarvis --auth [--change, --output, --help]");
            }
            else if args.len() >= 3 && (s.eq("change") || s.eq("--change") || s.eq("-c")) {
                if args.len() > 3 {
                    s = &args[3];
                }
                
                if args.len() == 3 {
                    println!("Usage: jarvis --auth --change [--internal, --external]");
                }
                else if args.len() >= 4 && (s.eq("internal") || s.eq("--internal") || s.eq("-i")) {
                    if args.len() > 4 {
                        s = &args[4];
                    }

                    if args.len() == 4 {
                        println!("Usage jarvis --auth --change --internal [+/-pubkey, +/-pass, +/-root_login]");
                    }
                    else if args.len() == 5 {
                        if mods::auth::check_root() == true {
                            mods::auth::change_internal(s);
                        }
                        else {
                            println!("ERROR: Invalid permissions");
                        }
                    }
                }
                else if args.len() >= 4 && (s.eq("external") || s.eq("--external") || s.eq("-e")) {
                    if args.len() > 4 {
                        s = &args[4];
                    }
                    
                    if args.len() == 4 {
                        println!("Usage: jarvis --auth --change --external [+/-pubkey, +/-pass, +/-root_login]");
                    }
                    else if args.len() == 5 {
                        if mods::auth::check_root() == true {
                            mods::auth::change_external(s);
                        }
                        else {
                            println!("ERROR: Invalid permissions");
                        }
                    }
                }
                else {
                    mods::jarvis_messages::unknown_command();
                }
            }
            else if args.len() >= 2 && (s.eq("output") || s.eq("--output") || s.eq("-o")) {
                mods::auth::status();
            }
            else if args.len() >= 2 && (s.eq("help") || s.eq("--help") || s.eq("-h")) {
                mods::auth::help();
            }
            else {
                mods::jarvis_messages::unknown_command();
            }
        }
        else {
            mods::jarvis_messages::unknown_command();
        }
    }
    else {
        let file = std::fs::read_to_string("/etc/jarvis/server_ver").expect("could not read file");
        let mut file_temp = file.to_string();
        file_temp.truncate(file_temp.len() - 1);
        println!("JARVIS release {} on Ubuntu 20.04.3 LTS", file_temp); 
        println!("\nPlease run jarvis --help for a list of commands");
    }
}
