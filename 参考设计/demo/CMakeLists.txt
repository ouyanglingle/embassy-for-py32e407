# ==============================================================================
#  demo - Puya PY32E407xE (Cortex-M4) CMake 工程
#
#  目标芯片 : PY32E407xE (PY32E407V1ET7)
#  工具链   : arm-none-eabi-gcc
#  构建     : cmake -B build -DCMAKE_TOOLCHAIN_FILE=cmake/arm-none-eabi-gcc.cmake
#             cmake --build build
#
#  旧工程（EIDE / Makefile）相关的目录保持原样，CMake 只是复用同一批源文件，
#  不改动 Drivers / Core / Middlewares 下的任何既有代码。
# ==============================================================================

cmake_minimum_required(VERSION 3.22)

# 必须在 project() 之前指定，否则 CMake 会先用宿主机编译器探测一遍。
if(NOT CMAKE_TOOLCHAIN_FILE)
    set(CMAKE_TOOLCHAIN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/cmake/arm-none-eabi-gcc.cmake"
        CACHE FILEPATH "ARM 交叉编译工具链文件")
endif()

project(demo
    DESCRIPTION "PY32E407xE demo firmware"
    LANGUAGES C CXX ASM
)

# ==============================================================================
#  1. 目标芯片与构建类型相关配置
# ==============================================================================

set(TARGET_MCU        "PY32E407xE"      CACHE STRING "芯片型号宏")
set(CPU               "cortex-m4"       CACHE STRING "CPU 内核")
set(FPU               "vfpv4-d16"       CACHE STRING "FPU 类型")
set(FLOAT_ABI         "hard"            CACHE STRING "浮点 ABI：soft / softfp / hard")
set(LINKER_SCRIPT
    "${CMAKE_CURRENT_SOURCE_DIR}/EIDE/PY32E407V1ET7.ld"
    CACHE FILEPATH "链接脚本")

# 未显式指定时，默认使用 Debug（等价于 EIDE 的 level-debug）
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "构建类型" FORCE)
    set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS Debug Release MinSizeRel RelWithDebInfo)
endif()

# ==============================================================================
#  2. 源文件
# ==============================================================================

# --- 用户代码 ------------------------------------------------------------------
# CONFIGURE_DEPENDS：新增/删除源文件时自动重新配置，无需手动重跑 cmake
file(GLOB APP_SOURCES CONFIGURE_DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/Core/SRC/*.c"
)

# 以下文件由 MCU Studio 生成但当前工程并未启用（hal_conf.h 中 HAL_SD_MODULE_ENABLED
# 已关闭，且 main.c 未调用 Studio_SDIO_Init），编入会导致链接错误，故排除。
set(EXCLUDED_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/Core/SRC/sdio.c"
)
list(REMOVE_ITEM APP_SOURCES ${EXCLUDED_SOURCES})

# --- HAL 驱动 ------------------------------------------------------------------
# HAL 的每个 .c 内部都由 `#ifdef HAL_XXX_MODULE_ENABLED` 包裹，未启用的模块会编译成
# 空目标文件，因此这里整目录收录是安全的，后续在 py32e407_hal_conf.h 里打开某个
# 模块即可直接使用，无需改动 CMakeLists.txt。
file(GLOB HAL_SOURCES CONFIGURE_DEPENDS
    "${CMAKE_CURRENT_SOURCE_DIR}/Drivers/PY32E407_HAL_Driver/Src/*.c"
)

# --- CMSIS / 启动文件 -----------------------------------------------------------
set(STARTUP_SOURCE "${CMAKE_CURRENT_SOURCE_DIR}/EIDE/startup_py32e407xx.s")
# 显式声明为 ASM，否则 CMake 会把 .s 交给 C 编译器处理
set_source_files_properties(${STARTUP_SOURCE} PROPERTIES LANGUAGE ASM)

set(SOURCES
    ${STARTUP_SOURCE}
    ${APP_SOURCES}
    ${HAL_SOURCES}
)

# ==============================================================================
#  3. 可执行文件目标
# ==============================================================================

add_executable(${CMAKE_PROJECT_NAME} ${SOURCES})

target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/Core/INC"
    "${CMAKE_CURRENT_SOURCE_DIR}/Drivers/CMSIS/Include"
    "${CMAKE_CURRENT_SOURCE_DIR}/Drivers/PY32E407_HAL_Driver/Inc"
    "${CMAKE_CURRENT_SOURCE_DIR}/Drivers/CMSIS/Device/PY32E407/Include"
)

target_compile_definitions(${CMAKE_PROJECT_NAME} PRIVATE
    ${TARGET_MCU}
    USE_HAL_DRIVER
)

# ==============================================================================
#  4. 编译 / 链接选项
# ==============================================================================

set(MCU_OPTIONS
    -mcpu=${CPU}
    -mfpu=${FPU}
    -mfloat-abi=${FLOAT_ABI}
)

target_compile_options(${CMAKE_PROJECT_NAME} PRIVATE
    ${MCU_OPTIONS}
    -mthumb
    -std=gnu11
    -Wall
    -fdata-sections
    -ffunction-sections
    -fno-common
    $<$<CONFIG:Debug>:-Og>
    $<$<CONFIG:Debug>:-g3>
    $<$<CONFIG:Release>:-O2>
    $<$<CONFIG:MinSizeRel>:-Os>
    $<$<CONFIG:RelWithDebInfo>:-O2>
    $<$<CONFIG:RelWithDebInfo>:-g3>
)

# 启动文件由厂商提供，内部使用 `.fpu softvfp`，不能与 hard-float 选项一起汇编，
# 否则新版汇编器会报 FPU/ABI 冲突，故单独降级为 soft 汇编。
set_source_files_properties(${STARTUP_SOURCE} PROPERTIES
    COMPILE_OPTIONS "-mcpu=${CPU};-mthumb;-mfloat-abi=soft"
)

target_link_options(${CMAKE_PROJECT_NAME} PRIVATE
    ${MCU_OPTIONS}
    -mthumb
    -T "${LINKER_SCRIPT}"
    -Wl,-Map=${CMAKE_CURRENT_BINARY_DIR}/${CMAKE_PROJECT_NAME}.map,--cref
    -Wl,--gc-sections
    -Wl,--print-memory-usage
    --specs=nano.specs
    --specs=nosys.specs
    -lm
    -lc
)

# 链接脚本变更时重新链接
set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES
    LINK_DEPENDS "${LINKER_SCRIPT}"
    OUTPUT_NAME  "${CMAKE_PROJECT_NAME}"
)

# ==============================================================================
#  5. 构建后处理：生成 hex / bin 并打印体积
# ==============================================================================

set(ELF_FILE  $<TARGET_FILE:${CMAKE_PROJECT_NAME}>)
set(HEX_FILE  "${CMAKE_CURRENT_BINARY_DIR}/${CMAKE_PROJECT_NAME}.hex")
set(BIN_FILE  "${CMAKE_CURRENT_BINARY_DIR}/${CMAKE_PROJECT_NAME}.bin")
set(LSS_FILE  "${CMAKE_CURRENT_BINARY_DIR}/${CMAKE_PROJECT_NAME}.lss")

add_custom_command(TARGET ${CMAKE_PROJECT_NAME} POST_BUILD
    COMMAND ${CMAKE_OBJCOPY} -O ihex   ${ELF_FILE} ${HEX_FILE}
    COMMAND ${CMAKE_OBJCOPY} -O binary -S ${ELF_FILE} ${BIN_FILE}
    COMMAND ${CMAKE_OBJDUMP} -h -S ${ELF_FILE} > ${LSS_FILE}
    COMMAND ${CMAKE_SIZE} --format=berkeley ${ELF_FILE}
    COMMENT "生成 ${CMAKE_PROJECT_NAME}.hex / .bin / .lss"
    VERBATIM
)

message(STATUS "-----------------------------------------------")
message(STATUS "  目标芯片 : ${TARGET_MCU} (${CPU}, ${FPU}, ${FLOAT_ABI})")
message(STATUS "  构建类型 : ${CMAKE_BUILD_TYPE}")
message(STATUS "  链接脚本 : ${LINKER_SCRIPT}")
message(STATUS "  输出目录 : ${CMAKE_CURRENT_BINARY_DIR}")
message(STATUS "  排除编译 : ${EXCLUDED_SOURCES}")
message(STATUS "-----------------------------------------------")
