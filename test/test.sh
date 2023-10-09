#!/bin/bash

gcc -static -o run.elf -I ../include test.c libsysy.a
./run.elf < test.in