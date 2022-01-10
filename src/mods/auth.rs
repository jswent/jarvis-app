use users::{get_user_by_uid, get_current_uid};
use std::fs;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use local_ip_address::local_ip;
use crate::mods::user_input::get_input_short;
use crate::mods::ssh;

pub fn check_root() -> bool {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    return user.name() == "root";
}

pub fn change_internal(action: &String) {
    let lines = lines_from_file("/etc/ssh/sshd_config");
    let mut new_lines = lines_from_file("/etc/ssh/sshd_config");
    
    let mut count = 0 as usize;
    for line in &lines {
        if line.contains("Match Address 172.16.*") {
            //println!("{}", lines[count+2]);

            if action == "+pubkey" || action == "+publickey" {
                if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") || lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                    println!("ERROR: Internal authentication already contains public key encryption");
                    println!("\nInternal authentication: {}", lines[count+2]);
                }
                else if lines[count+2].contains("AuthenticationMethods \"password\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"publickey,password\"");
                }
                else if lines[count+2].contains("AuthenticationMethods \"\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"publickey\"");
                }
                else {
                    println!("ERROR: Unable to locate AuthenticationMethods (mods::auth::change_internal)");
                }
            }
            else if action == "-pubkey" || action == "-publickey" {
                if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"password\"");
                }
                else if lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                    let input: String = get_input_short("WARNING: You are removing all internal authentication, would you like to proceed? (Y/N): ");
                    if input == "89" || input == "121" {
                        new_lines[count+2] = String::from("  AuthenticationMethods \"\"");
                    }
                    else if input == "78" || input == "110" {
                        println!("Aborting...");
                    }
                    else {
                        println!("Invalid character, aborting...");
                    }
                }
                else if lines[count+2].contains("AuthenticationMethods \"password\"") {
                    println!("ERROR: Internal authentication does not contain [{}]", action);
                }
            }
            else if action == "+pass" || action == "+password" {
                if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") || lines[count+2].contains("AuthenticationMethods \"password\"") {
                    println!("ERROR: Internal authentication already contains password encryption");
                    println!("\nInternal authentication: {}", lines[count+2]);
                }
                else if lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"publickey,password\"");
                }
                else {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"password\"");
                }
            }
            else if action == "-pass" || action == "-password" {
                if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"publickey\"");
                }
                else if lines[count+2].contains("AuthenticationMethods \"password\"") {
                    let input: String = get_input_short("WARNING: You are removing all internal authentication, would you like to proceed? (Y/N): ");
                    if input == "89" || input == "121" {
                        new_lines[count+2] = String::from("  AuthenticationMethods \"\"");
                    }
                    else if input == "78" || input == "110" {
                        println!("Aborting...");
                    }
                    else {
                        println!("Invalid character, aborting...");
                    }
                }
                else if lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                    println!("ERROR: Internal authentication does not contain [{}]", action);
                }
            }
            else if action == "+root_login" {
                if lines[count+1].contains("PermitRootLogin yes") {
                    println!("ERROR: Root user login already allowed");
                }
                else if lines[count+1].contains("PermitRootLogin no") {
                    new_lines[count+1] = String::from("  PermitRootLogin yes");
                }
            }
            else if action == "-root_login" {
                if lines[count+1].contains("PermitRootLogin yes") {
                    new_lines[count+1] = String::from("  PermitRootLogin no");
                }
                else if lines[count+1].contains("PermitRootLogin no") {
                    println!("ERROR: Root user login already disabled");
                }
            }
            else {
                println!("ERROR: Unknown attribute");
                println!("Usage: jarvis --auth --change --external [+/-pubkey, +/-pass, +/-root_login]");
            }
        }

        count = count + 1;
    }

    write_vec_to_file(new_lines);
    ssh::restart();
}

pub fn change_external(action: &String) {
    let lines = lines_from_file("/etc/ssh/sshd_config");
    let mut new_lines = lines_from_file("/etc/ssh/sshd_config");
    
    let mut count = 0 as usize;
    for line in &lines {
        if line.contains("Match Address *,!172.16.0.0/24") {
            if action == "+pubkey" || action == "+publickey" {
                if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") || lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                    println!("ERROR: External authentication already contains public key encryption");
                    println!("\nExternal authentication: {}", lines[count+2]);
                }
                else if lines[count+2].contains("AuthenticationMethods \"password\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"publickey,password\"");
                }
                else if lines[count+2].contains("AuthenticationMethods \"\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"publickey\"");
                }
                else {
                    println!("ERROR: Unable to locate AuthenticationMethods (mods::auth::change_external)");
                }
            }
            else if action == "-pubkey" || action == "-publickey" {
                if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"password\"");
                }
                else if lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                    let input: String = get_input_short("WARNING: You are removing all external authentication, would you like to proceed? (Y/N): ");
                    if input == "89" || input == "121" {
                        new_lines[count+2] = String::from("  AuthenticationMethods \"\"");
                    }
                    else if input == "78" || input == "110" {
                        println!("Aborting...");
                    }
                    else {
                        println!("Invalid character, aborting...");
                    }
                }
                else if lines[count+2].contains("AuthenticationMethods \"password\"") {
                    println!("ERROR: External authentication does not contain [{}]", action);
                }
            }
            else if action == "+pass" || action == "+password" {
                if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") || lines[count+2].contains("AuthenticationMethods \"password\"") {
                    println!("ERROR: External authentication already contains password encryption");
                    println!("\nExternal authentication: {}", lines[count+2]);
                }
                else if lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"publickey,password\"");
                }
                else {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"password\"");
                }
            }
            else if action == "-pass" || action == "-password" {
                if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") {
                    new_lines[count+2] = String::from("  AuthenticationMethods \"publickey\"");
                }
                else if lines[count+2].contains("AuthenticationMethods \"password\"") {
                    let input: String = get_input_short("WARNING: You are removing all external authentication, would you like to proceed? (Y/N): ");
                    if input == "89" || input == "121" {
                        new_lines[count+2] = String::from("  AuthenticationMethods \"\"");
                    }
                    else if input == "78" || input == "110" {
                        println!("Aborting...");
                    }
                    else {
                        println!("Invalid character, aborting...");
                    }
                }
                else if lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                    println!("ERROR: External authentication does not contain [{}]", action);
                }
            }
            else if action == "+root_login" {
                if lines[count+1].contains("PermitRootLogin yes") {
                    println!("ERROR: Root user login already allowed");
                }
                else if lines[count+1].contains("PermitRootLogin no") {
                    new_lines[count+1] = String::from("  PermitRootLogin yes");
                }
            }
            else if action == "-root_login" {
                if lines[count+1].contains("PermitRootLogin yes") {
                    new_lines[count+1] = String::from("  PermitRootLogin no");
                }
                else if lines[count+1].contains("PermitRootLogin no") {
                    println!("ERROR: Root user login already disabled");
                }
            }
            else {
                println!("ERROR: Unknown attribute");
                println!("Usage: jarvis --auth --change --external [+/-pubkey, +/-pass, +/-root_login]");
            }
        }
        count = count + 1;
    }
    write_vec_to_file(new_lines);
    ssh::restart();
}

pub fn status() {
    let my_local_ip = local_ip().unwrap();
    println!("\x1b[1mJARVIS SSH Authentication Status on {:?}\x1b[0m\n", my_local_ip);

    println!("\x1b[4mInternal connections:\x1b[0m");
    println!("Root login: {}", get_root_status(true));
    println!("Authentication method(s): {}\n", get_auth_method(true));

    println!("\x1b[4mExternal connections:\x1b[0m");
    println!("Root login: {}", get_root_status(false));
    println!("Authentication method(s): {}\n", get_auth_method(false));
}


fn get_root_status(internal: bool) -> String {
    let lines = lines_from_file("/etc/ssh/sshd_config");
    let mut count = 0;

    let mut search = String::from("Match Address *,!172.16.0.0/24");

    if internal == true {
        search = String::from("Match Address 172.16.*");
    }

    for line in &lines {
        if line.contains(&search) {
            if lines[count+1].contains("PermitRootLogin yes") {
                return String::from("\x1b[32;1myes\x1b[0m");
            }
            else if lines[count+1].contains("PermitRootLogin no") {
                return String::from("\x1b[31;1mno\x1b[0m");
            }
            else {
                return String::from("ERR@mods::auth::get_root_status");
            }
        }
        count = count + 1;
    }
    return String::from("");
}

fn get_auth_method(internal: bool) -> String {
    let lines = lines_from_file("/etc/ssh/sshd_config");
    let mut count = 0;

    let mut search = String::from("Match Address *,!172.16.0.0/24");

    if internal == true {
        search = String::from("Match Address 172.16.*");
    }

    for line in &lines {
        if line.contains(&search) {
            if lines[count+2].contains("AuthenticationMethods \"publickey,password\"") {
                return String::from("public-key encryption, password");
            }
            else if lines[count+2].contains("AuthenticationMethods \"publickey\"") {
                return String::from("public-key encryption");
            }
            else if lines[count+2].contains("AuthenticationMethods \"password\"") {
                return String::from("password");
            }
            else if lines[count+2].contains("AuthenticationMethods \"\"") {
                return String::from("none");
            }
            else {
                return String::from("ERR@mods::auth::get_auth_method");
            }
        }

        count = count + 1;
    }
    return String::from("")
}

pub fn help() {
    let my_local_ip = local_ip().unwrap();
    println!("\x1b[1mJARVIS SSH Authentication Status on {:?}\x1b[0m\n", my_local_ip);

    println!("'jarvis --auth' commands: ");
    println!("jarvis --auth --change [--internal, --external] [+/-pubkey, +/-pass, +/-root_login]");
    println!("jarvis --auth --output");
    println!("jarvis --auth --help");
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn write_vec_to_file(lines: Vec<String>) {
    let joined = lines.join("\n");
    //println!("{}", joined);
    fs::write("/etc/ssh/sshd_config", joined).expect("Unable to write to file");
}