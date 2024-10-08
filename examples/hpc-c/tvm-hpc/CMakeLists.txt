cmake_minimum_required(VERSION 3.13)
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

project(headsail-tvm C ASM)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")
find_package(RiscvGcc REQUIRED)
find_package(DlaDriver REQUIRED)
find_package(Newlib REQUIRED)

# Choose target model
option(USE_PERF_IMAGE_CLASSIFICATION "Perform image classification benchmark" OFF)
option(USE_PERF_KEYWORDS_SPOTTING "Perform keyword spotting benchmark" OFF)
option(USE_PERF_VISUAL_WAKE_WORDS "Perform visual wake word benchmark" OFF)
option(USE_ACCELERATOR "Use accelerator" ON)

if(USE_ACCELERATOR)
    set(PYTHON_ACCELERATOR_PARAM "--annotate-graph")
endif()

if (USE_PERF_IMAGE_CLASSIFICATION)
    set(MODEL_NAME perf_image_classification)
    set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/models/pretrainedResnet_quant.tflite)
    set(INPUT_TYPE "int8")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D IMAGE_CLASSIFICATION")
elseif (USE_PERF_KEYWORD_SPOTTING)
    set(MODEL_NAME perf_keyword_spotting)
    set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/models/kws_ref_model.tflite)
    set(INPUT_TYPE "float")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D KEYWORD_SPOTTING")
elseif (USE_PERF_VISUAL_WAKEUP_WORD)
    set(MODEL_NAME perf_visual_wakeup_word)
    set(ONNX_MODEL ${CMAKE_CURRENT_SOURCE_DIR}/models/vww_96_int8.tflite)
    set(INPUT_TYPE "float")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D VISUAL_WAKEUP_WORD")
else()
    message(FATAL_ERROR "No model selected")
endif()

# Build codegen library
execute_process(COMMAND python ${CMAKE_CURRENT_SOURCE_DIR}/build_module.py -m ${MODEL_NAME} -p ${ONNX_MODEL} -o ./model_c -t ${INPUT_TYPE} ${PYTHON_ACCELERATOR_PARAM})

# Add C-runtime 0
set(CRT0 ${CMAKE_CURRENT_SOURCE_DIR}/../crt0)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/../cmake")

# Set GCC compile options for RISC-V 64-bit architecture with IMAC extensions
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -g -O0 -march=rv64imac -mabi=lp64 -nostdlib -T${CRT0}/linker_sdram.lds -mcmodel=medany")

set(TVM_SOURCES $ENV{TVM_HOME}/build/standalone_crt/src)
set(TVM_CODEGEN ${CMAKE_CURRENT_SOURCE_DIR}/build/model_c/codegen/host)

# Gather all lib*.c files from a specific directory
file(GLOB LIB_SOURCES ${TVM_CODEGEN}/src/default_lib*.c)
message(GLOB="${LIB_SOURCES}")

# Add the source files
set(SOURCE_FILES
    src/aot.c
    ${LIB_SOURCES}
)

# Add the executable target
add_executable(${CMAKE_PROJECT_NAME} ${SOURCE_FILES} ${LIB_PATH}/crt0.o)

# Link newlib
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libc.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libm.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${LIB_PATH}/libgloss.a)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE -lc -lm -lgloss -lgcc)

# Include TVM headers
target_include_directories(${CMAKE_PROJECT_NAME} PUBLIC src)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/standalone_crt/include)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/standalone_crt/include/dlpack)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE $ENV{TVM_HOME}/build/standalone_crt/include/tvm)

target_include_directories(${CMAKE_PROJECT_NAME} PUBLIC ${TVM_CODEGEN}/include)

# Include DLA-FFI
add_dependencies(${CMAKE_PROJECT_NAME} build_dla)
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE ${DLA_INCLUDE_DIR})
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE ${DLA_LIBRARIES})
