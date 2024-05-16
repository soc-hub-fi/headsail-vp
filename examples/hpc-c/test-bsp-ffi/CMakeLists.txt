cmake_minimum_required(VERSION 3.10)

# Set riscv target
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(RISCV_Compiler C)

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")
find_package(RiscvGcc REQUIRED) # CMAKE_C_COMPILER, CMAKE_ASM_COMPILER, CMAKE_LINKER
find_package(Bsp REQUIRED) # BSP_INCLUDE_DIR, BSP_LIBRARIES

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
target_include_directories(main PRIVATE ${BSP_INCLUDE_DIR})

# Link bsp static lib
target_link_libraries(main ${BSP_LIBRARIES})
