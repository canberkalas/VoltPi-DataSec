#include <avr/io.h>
#include <util/delay.h>
#include <stdint.h>

#define UART_DATA_REG UDR0
#define UART_BAUD_RATE 9600
#define F_CPU 16000000UL

uint32_t crc32(const uint8_t* data, size_t length) {
    uint32_t crc = 0xFFFFFFFF;
    while (length--) {
        crc ^= *data++;
        for (uint8_t i = 0; i < 8; i++) {
            if (crc & 1)
                crc = (crc >> 1) ^ 0xEDB88320;
            else
                crc >>= 1;
        }
    }
    return crc ^ 0xFFFFFFFF;
}

void setup() {
    // UART Baud Rate Ayarları
    UBRR0H = (F_CPU / 16 / UART_BAUD_RATE - 1) >> 8;
    UBRR0L = (F_CPU / 16 / UART_BAUD_RATE - 1);
    UCSR0B = (1 << TXEN0); // UART Transmitter'ı aktif etme
    UCSR0C = (1 << UCSZ01) | (1 << UCSZ00); // UART frame formatı ayarları: 8 data bit, 1 stop bit
}

void sendByte(uint8_t byte) {
    while (!(UCSR0A & (1 << UDRE0)));
    UDR0 = byte;
}

void sendData(const uint8_t* data, size_t length) {
    for (size_t i = 0; i < length; i++) {
        sendByte(data[i]);
    }
}

void loop() {
    uint8_t data[] = {0x12, 0x34}; // Örnek veri
    uint32_t crc = crc32(data, sizeof(data));

    // Başlangıç baytını gönder
    sendByte(0x01);

    // Veri uzunluğunu gönder
    sendByte(sizeof(data));

    // Veriyi gönder
    sendData(data, sizeof(data));

    // CRC'yi gönder
    sendData((uint8_t*)&crc, sizeof(crc));

    // Bitiş baytını gönder
    sendByte(0xFF);

    _delay_ms(1000); // 1 saniye bekle
}
