use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // Compile and upload Arduino code
    println!("Uploading Arduino code...");
    let output = Command::new("arduino-cli")
        .args(&["compile", "--fqbn", "arduino:avr:uno", "UN0"])
        .output()
        .expect("Failed to compile Arduino code");
    println!("{:?}", output);

    let output = Command::new("arduino-cli")
        .args(&["upload", "-p", "/ttyUSB0", "--fqbn", "arduino:avr:uno", "UN0"])
        .output()
        .expect("Failed to upload Arduino code");
    println!("{:?}", output);

    // Compile and run Raspberry Pi code
    println!("Compiling Raspberry Pi code...");
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("PI2")
        .output()
        .expect("Failed to compile Raspberry Pi code");
    println!("{:?}", output);

    println!("Running Raspberry Pi code...");
    let mut child_rpi = Command::new("./target/release/pi2")
        .current_dir("PI2")
        .spawn()
        .expect("Failed to run Raspberry Pi code");

    // Run Ubuntu code
    println!("Running Ubuntu code...");
    let mut child_ubuntu = Command::new("./release/main")
        .current_dir("main")
        .spawn()
        .expect("Failed to run Ubuntu code");

    // Wait for the simulation to complete
    thread::sleep(Duration::from_secs(11);

    // Terminate Raspberry Pi and Ubuntu processes
    child_rpi.kill().expect("Failed to kill Raspberry Pi process");
    child_ubuntu.kill().expect("Failed to kill Ubuntu process");
}
