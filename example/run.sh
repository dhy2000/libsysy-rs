#!/bin/bash
cargo build -r
cp ../target/release/libsysy.a .
gcc -static -o example.elf -I ../include example.c libsysy.a
./example.elf < input.txt