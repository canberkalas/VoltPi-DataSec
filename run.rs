use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::io::{self, Read};

fn check_ufw_status() {
    let output = Command::new("ufw")
        .arg("status")
        .output()
        .expect("Failed to execute UFW status check");

    let status = String::from_utf8_lossy(&output.stdout);
    if !status.contains("Status: active") {
        panic!("UFW is not active. Please enable UFW for security.");
    } else {
        println!("UFW is active.");
    }
}

fn check_open_ports() {
    let output = Command::new("lsof")
        .arg("-i")
        .output()
        .expect("Failed to check open ports");

    let ports = String::from_utf8_lossy(&output.stdout);
    println!("Open ports: \n{}", ports);
}

fn main() {
    // Security checks
    check_ufw_status();
    check_open_ports();

    // Compile and run the main program
    println!("Compiling and running main program...");

    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to compile main program");
    println!("{:?}", output);

    let mut child_main = Command::new("./target/release/volt-pi-datasec")
        .spawn()
        .expect("Failed to run main program");

    // Wait for the main program to complete
    let result = child_main.wait().expect("Failed to wait on main program");
    if result.success() {
        println!("Main program completed successfully.");
    } else {
        eprintln!("Main program failed.");
    }
}
