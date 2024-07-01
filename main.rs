use std::io::{self, Read, Write};
use std::fs::File;
use std::process::Command;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use crc::crc64;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

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

fn encrypt_data(key: &[u8; 16], iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

fn main() {
    // Security checks
    check_ufw_status();
    check_open_ports();

    // File operations
    let mut file = File::open("/ttyUSB0").expect("Cannot open serial port");
    let mut buffer = [0; 256];

    loop {
        let n = file.read(&mut buffer).expect("Failed to read from serial port");
        if n > 0 {
            let crc = crc64::checksum_ecma(&buffer[..n]);
            let key = [0u8; 16];
            let iv = [0u8; 16];
            let encrypted_data = encrypt_data(&key, &iv, &buffer[..n]);

            println!("Received data: {:?}, CRC-64: {:x}", &buffer[..n], crc);
            println!("Encrypted data: {:?}", encrypted_data);
        }
    }
}
