#![allow(unused)]

use users::{get_user_by_uid, get_current_uid};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::fs;

fn print_jarvis_text() {
    let file = std::fs::read_to_string("/etc/jarvis/server_ver").expect("could not read file");
    let mut file_temp = file.to_string();
    file_temp.truncate(file_temp.len() - 1);
    println!("JARVIS release {} on Ubuntu 20.04.3 LTS", file_temp); 
}

fn main() {
    //let args = Cli::from_args();
    //println!("{}", args.len());
    let args: Vec<String> = env::args().collect();
    //println!("{}", args.len());

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
                let user = get_user_by_uid(get_current_uid()).unwrap();
                if user.name() == "root" {
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
    }
    else {
        let file = std::fs::read_to_string("/etc/jarvis/server_ver").expect("could not read file");
        let mut file_temp = file.to_string();
        file_temp.truncate(file_temp.len() - 1);
        println!("JARVIS release {} on Ubuntu 20.04.3 LTS", file_temp); 
        println!("\nPlease run jarvis --help for a list of commands");
    }
}
