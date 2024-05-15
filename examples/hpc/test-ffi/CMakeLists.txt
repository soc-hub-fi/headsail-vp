cmake_minimum_required(VERSION 3.10)

# Set riscv target
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(RISCV_Compiler C)

set(CMAKE_C_COMPILER $ENV{CC})
message(CMAKE_C_COMPILER="${CMAKE_C_COMPILER}")
set(CMAKE_ASM_COMPILER $ENV{CC}/../riscv${XLEN}-unknown-elf-as)
set(CMAKE_LINKER $ENV{CC}/../riscv${XLEN}-unknown-elf-ld)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)

# Add BSP variables
set(BSP_PROJECT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../headsail-bsp-ffi)
set(BSP_TARGET_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../headsail-bsp-ffi/target)
set(BSP_BUILD_DIR_DEBUG ${BSP_TARGET_DIR}/riscv64imac-unknown-none-elf/debug)
set(BSP_BUILD_DIR_RELEASE ${BSP_TARGET_DIR}/riscv64imac-unknown-none-elf/release)
message(BSP_PROJECT_DIR=${BSP_PROJECT_DIR})

# Build BSP
add_custom_command(
    OUTPUT ${BSP_BUILD_DIR_RELEASE}/libheadsail_bsp_ffi.a
    COMMAND cd ${BSP_PROJECT_DIR} && just build-hpc
    WORKING_DIRECTORY ${BSP_PROJECT_DIR}
    COMMENT "Building bsp"
)

add_custom_target(build_hpc DEPENDS ${BSP_BUILD_DIR_RELEASE}/libheadsail_bsp_ffi.a)

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -Os -march=rv64imac -nostartfiles -mabi=lp64 -fno-builtin -T${CRT0}/linker.lds")

# Add the source file(s)
set(SOURCE_FILES main.c)

# Add the executable target
add_executable(main ${SOURCE_FILES})

# Add dependencies to ensure build_hpc is executed before building the main target
add_dependencies(main build_hpc)

# Add C-runtime 0 header
target_include_directories(main PRIVATE ${CRT0})

# Add BSP header
target_include_directories(main PRIVATE ${BSP_TARGET_DIR})

# Link bsp static lib
target_link_libraries(main ${BSP_BUILD_DIR_RELEASE}/libheadsail_bsp_ffi.a)
message(STATUS ${BSP_BUILD_DIR_RELEASE})
