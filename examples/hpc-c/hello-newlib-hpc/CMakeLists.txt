cmake_minimum_required(VERSION 3.10)

# Set riscv target
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(hello-hpc C)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)
set(INC_PATH "/opt/headsail-newlib/cygwin-newlib/riscv64-unknown-elf/include")
set(LIB_PATH "/opt/headsail-newlib/cygwin-newlib/riscv64-unknown-elf/lib")
find_library(NEWLIB NAMES libgloss.a libc.a libg.a libm.a REQUIRED HINTS "${LIB_PATH}")
find_library(LIBGLOSS NAMES libgloss.a REQUIRED HINTS "${LIB_PATH}")


list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")
find_package(RiscvGcc REQUIRED)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0 -march=rv64imac -mabi=lp64 -nostdlib -lgloss -T${CRT0}/linker.lds -mcmodel=medany -Fuse-ld=riscv64-unknown-elf-ld")
# set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0 -march=rv64imac -mabi=lp64 -ffreestanding -nostartfiles -fno-builtin -lc -lg -T${CRT0}/linker.lds -mcmodel=medany -Fuse-ld=riscv64-unknown-elf-ld")

# Add the source file(s)
set(SOURCE_FILES main.c)

# Add the executable target
add_executable(${CMAKE_PROJECT_NAME} ${SOURCE_FILES} ${LIB_PATH}/crt0.o)
target_link_options(${CMAKE_PROJECT_NAME} PUBLIC LINKER:-Map=output.map)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libc.a)
# target_link_libraries(${CMAKE_PROJECT_NAME} ${LIB_PATH}/libg.a)
# target_link_libraries(${CMAKE_PROJECT_NAME} ${LIB_PATH}/libm.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libgloss.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lgcc -lgloss)
