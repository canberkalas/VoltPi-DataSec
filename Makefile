.PHONY: all clean upload

all: compile_uno compile_pi

compile_uno:
    @echo "Compiling Arduino code..."
    cd UNO && make

compile_pi:
    @echo "Compiling Raspberry Pi code..."
    cd PI2 && cargo build --release

upload:
    @echo "Uploading Arduino code..."
    cd UNO && make upload

clean:
    @echo "Cleaning up..."
    cd UNO && make clean
    cd PI2 && cargo cleans
