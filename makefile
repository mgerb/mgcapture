clean:
	rm -rf ./target

build-windows:
	cargo build -r --target x86_64-pc-windows-gnu

build:
	cargo build -r

all: build build-windows
