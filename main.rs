use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::fs::File;
use std::io::{self, Read, Write};
use crc::crc64;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn decrypt_data(key: &[u8; 16], iv: &[u8; 16], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
    cipher.decrypt_vec(ciphertext).unwrap()
}

fn crc64_check(data: &[u8]) -> u64 {
    crc64::checksum_ecma(data)
}

fn memory_check() {
    unsafe {
        asm!(
            "
            mov r0, #0x20000000
            ldr r1, [r0]
            cmp r1, #0
            bne error
            ",
            options(nostack, nomem)
        );
    }
}

fn main() {
    // Security checks
    memory_check();

    // File operations
    let mut file = File::open("/dev/serial0").expect("Cannot open serial port");
    let mut buffer = [0; 256];

    loop {
        let n = file.read(&mut buffer).expect("Failed to read from serial port");
        if n > 0 {
            let crc = crc64_check(&buffer[..n]);
            let key = [0u8; 16];
            let iv = [0u8; 16];
            let decrypted_data = decrypt_data(&key, &iv, &buffer[..n]);

            println!("Received encrypted data: {:?}", &buffer[..n]);
            println!("Decrypted data: {:?}", decrypted_data);
            println!("CRC-64: {:x}", crc);
        }
    }
}
