cmake_minimum_required(VERSION 3.10)

# Set riscv target
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(RISCV_Compiler C)

set(CMAKE_C_COMPILER $ENV{CC})
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")
set(CMAKE_ASM_COMPILER $ENV{CC}/../riscv32-unknown-elf-as)
set(CMAKE_LINKER $ENV{CC}/../riscv32-unknown-elf-ld)

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -march=rv64imac -nostartfiles -mabi=lp64 -fno-builtin -T${CMAKE_CURRENT_SOURCE_DIR}/linker.lds")

# Add the source file(s)
set(SOURCE_FILES main.c boot.h)

# Add the executable target
add_executable(main ${SOURCE_FILES})
