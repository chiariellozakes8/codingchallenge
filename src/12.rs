use std::process::Command;

fn main() {
    let command = "echo 'Hello, world!'";
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
