cmake_minimum_required(VERSION 3.10)

# Set riscv target
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(hello-hpc C)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")
find_package(RiscvGcc REQUIRED)

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -march=rv64imac -nostartfiles -mabi=lp64 -fno-builtin -mcmodel=medany -T${CRT0}/linker.lds")

# Add the source file(s)
set(SOURCE_FILES main.c)

# Add the executable target
add_executable(${CMAKE_PROJECT_NAME} ${SOURCE_FILES})

# Add header
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${CRT0})
