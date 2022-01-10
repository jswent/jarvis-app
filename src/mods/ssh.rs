use std::process::Command;

pub fn restart() {
    let output = Command::new("service")
                            .arg("ssh")
                            .arg("restart")
                            .output();

}