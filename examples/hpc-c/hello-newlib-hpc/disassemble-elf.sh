#!/bin/bash

riscv64-unknown-elf-objdump -DSrwzC ./build/hello-hpc > hello-hpc.asm

