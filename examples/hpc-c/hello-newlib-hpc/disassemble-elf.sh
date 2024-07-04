#!/bin/bash

riscv64-unknown-elf-objdump -DSrwzC -M att ./build/hello-hpc > hello-hpc.asm

