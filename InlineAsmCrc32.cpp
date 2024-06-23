#include <avr/io.h>
#include <util/delay.h>

#define UART_DATA_REG UDR0

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
    UBRR0H = 0;
    UBRR0L = 103; // 9600 baud rate için
    UCSR0B = (1 << TXEN0); // UART Transmitter'ı aktif etme
    UCSR0C = (1 << UCSZ01) | (1 << UCSZ00); // UART frame formatı ayarları: 8 data bit, 1 stop bit
}

void loop() {
    uint8_t data[] = {0x12, 0x34}; // Örnek veri
    uint32_t crc = crc32(data, sizeof(data));

    // Başlangıç baytını gönder
    asm volatile (
        "ldi r16, 0x01\n\t"
        "out %0, r16\n\t"
        : : "I" (_SFR_IO_ADDR(UART_DATA_REG)) : "r16"
    );

    // Veri uzunluğunu gönder
    asm volatile (
        "ldi r16, 0x04\n\t" // 4 bayt veri (örnek)
        "out %0, r16\n\t"
        : : "I" (_SFR_IO_ADDR(UART_DATA_REG)) : "r16"
    );

    // Veriyi gönder
    asm volatile (
        "ldi r16, 0x12\n\t"
        "ldi r17, 0x34\n\t"
        "out %0, r16\n\t"
        "out %1, r17\n\t"
        : : "I" (_SFR_IO_ADDR(UART_DATA_REG)), "I" (_SFR_IO_ADDR(UART_DATA_REG)) : "r16", "r17"
    );

    // CRC'yi gönder
    asm volatile (
        "mov r16, %0\n\t"
        "out %1, r16\n\t"
        : : "r" (crc), "I" (_SFR_IO_ADDR(UART_DATA_REG)) : "r16"
    );

    // Bitiş baytını gönder
    asm volatile (
        "ldi r16, 0xFF\n\t"
        "out %0, r16\n\t"
        : : "I" (_SFR_IO_ADDR(UART_DATA_REG)) : "r16"
    );

    _delay_ms(1000); // 1 saniye bekle
}
