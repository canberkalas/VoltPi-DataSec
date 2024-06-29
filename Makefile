# Makefile for compiling and uploading the Arduino project

MCU = atmega328p
F_CPU = 16000000UL
BAUD = 9600
PROGRAMMER = arduino
PORT = /dev/ttyUSB0
CFLAGS = -DF_CPU=$(F_CPU) -DBAUD=$(BAUD) -mmcu=$(MCU) -Os

SRC = src/InlineAsmUART.cpp src/CRC32.cpp src/Encryption.cpp
OBJ = $(SRC:.cpp=.o)

all: main.hex

main.elf: $(OBJ)
    avr-gcc $(CFLAGS) -o $@ $^

main.hex: main.elf
    avr-objcopy -O ihex -R .eeprom $< $@

upload: main.hex
    avrdude -c $(PROGRAMMER) -p $(MCU) -P $(PORT) -U flash:w:$<

clean:
    rm -f src/*.o main.elf main.hex
