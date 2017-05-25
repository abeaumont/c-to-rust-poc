all: main

main: src/main.c libexported.a
	cc -o main src/main.c target/debug/libexported.a -lpthread -ldl

libexported.a: src/lib.rs
	cargo build
