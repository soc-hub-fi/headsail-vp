set(CMAKE_C_COMPILER $ENV{CC})
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")
get_filename_component(CC_DIR $ENV{CC} DIRECTORY)
set(CMAKE_ASM_COMPILER ${CC_DIR}/riscv$ENV{XLEN}-unknown-elf-as)
set(CMAKE_LINKER ${CC_DIR}/riscv$ENV{XLEN}-unknown-elf-ld)
