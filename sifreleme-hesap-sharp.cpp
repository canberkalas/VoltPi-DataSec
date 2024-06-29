#include <avr/io.h>
#include <util/delay.h>
#include <stdint.h>
#include "CRC32.h"
#include "Encryption.h"

#define UART_DATA_REG UDR0
#define UART_BAUD_RATE 9600
#define F_CPU 16000000UL

// AES key and IV (Initialization Vector)
const uint8_t aes_key[16] = { /* Your AES key here */ };
const uint8_t aes_iv[16] = { /* Your AES IV here */ };

void setup() {
    // UART Baud Rate Ayarları
    UBRR0H = (F_CPU / 16 / UART_BAUD_RATE - 1) >> 8;
    UBRR0L = (F_CPU / 16 / UART_BAUD_RATE - 1);
    UCSR0B = (1 << TXEN0) | (1 << RXEN0); // UART Transmitter ve Receiver'ı aktif etme
    UCSR0C = (1 << UCSZ01) | (1 << UCSZ00); // UART frame formatı ayarları: 8 data bit, 1 stop bit
}

void sendByte(uint8_t byte) {
    while (!(UCSR0A & (1 << UDRE0)));
    UDR0 = byte;
}

uint8_t receiveByte() {
    while (!(UCSR0A & (1 << RXC0)));
    return UDR0;
}

void sendData(const uint8_t* data, size_t length) {
    for (size_t i = 0; i < length; i++) {
        sendByte(data[i]);
    }
}

void encryptAndSendData(const uint8_t* data, size_t length) {
    uint8_t encrypted_data[256]; // Adjust size as needed
    aes128_cbc_encrypt(data, length, aes_key, aes_iv, encrypted_data);

    uint32_t crc = crc32(encrypted_data, length);

    // Send start byte
    sendByte(0x01);
    // Send data length
    sendByte(length);
    // Send encrypted data
    sendData(encrypted_data, length);
    // Send CRC
    sendByte((crc >> 24) & 0xFF);
    sendByte((crc >> 16) & 0xFF);
    sendByte((crc >> 8) & 0xFF);
    sendByte(crc & 0xFF);
    // Send end byte
    sendByte(0xFF);
}

void detect_processor() {
    uint8_t signature = 0;

    // Inline assembly to read the device signature
    asm volatile (
        "lds %[sig], 0x0000\n\t"  // Read the signature byte from address 0x0000
        : [sig] "=r" (signature)
    );

    // Print the signature byte for debugging
    // Replace this with any specific action you need to take based on the signature
    if (signature == 0x1E) {
        // Example signature check
        // Add specific actions here
    }
}

int main(void) {
    setup();
    detect_processor();

    uint8_t data[] = {0x12, 0x34}; // Örnek veri

    while (1) {
        encryptAndSendData(data, sizeof(data));
        _delay_ms(1000); // 1 saniye bekle
    }
}
