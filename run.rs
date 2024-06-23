use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::io::{self, Read};

fn execute_command(cmd: &str, args: &[&str]) -> io::Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Error executing {}: {}",
            cmd,
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Command execution failed",
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn main() {
    // Arduino kodunu derleyip yükleyin
    println!("Uploading Arduino code...");
    match execute_command("arduino-cli", &["compile", "--fqbn", "arduino:avr:uno", "Arduino/InlineAsmUART"]) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Failed to compile Arduino code: {}", e),
    }

    match execute_command("arduino-cli", &["upload", "-p", "/dev/ttyUSB0", "--fqbn", "arduino:avr:uno", "Arduino/InlineAsmUART"]) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Failed to upload Arduino code: {}", e),
    }

    // Raspberry Pi kodunu derleyip çalıştırın
    println!("Compiling Raspberry Pi code...");
    match execute_command("cargo", &["build", "--release", "--manifest-path", "RaspberryPi/Cargo.toml"]) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Failed to compile Raspberry Pi code: {}", e),
    }

    println!("Running Raspberry Pi code...");
    let mut child_rpi = match Command::new("./target/release/Volt-pi-datasec")
        .current_dir("RaspberryPi")
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to run Pi 2 code: {}", e);
            return;
        }
    };

    // Ubuntu kodunu çalıştırın
    println!("Running Ubuntu code...");
    let mut child_ubuntu = match Command::new("./target/release/your_project_name")
        .current_dir("Ubuntu")
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to run Ubuntu code: {}", e);
            return;
        }
    };

    // Simülasyonun tamamlanmasını bekleyin
    thread::sleep(Duration::from_secs(10));

    // Raspberry Pi ve Ubuntu süreçlerini sonlandırın
    if let Err(e) = child_rpi.kill() {
        eprintln!("Failed to kill Raspberry Pi process: {}", e);
    }
    if let Err(e) = child_ubuntu.kill() {
        eprintln!("Failed to kill Ubuntu process: {}", e);
    }
}
