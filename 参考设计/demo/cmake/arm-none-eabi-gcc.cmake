# ------------------------------------------------------------------------------
# 交叉编译工具链文件：GNU Arm Embedded Toolchain (arm-none-eabi-gcc)
#
# 用法：在配置阶段传入
#   cmake -B build -DCMAKE_TOOLCHAIN_FILE=cmake/arm-none-eabi-gcc.cmake
# 若工具链不在 PATH 中，追加 -DTOOLCHAIN_BIN_DIR=<gcc 所在的 bin 目录>
# ------------------------------------------------------------------------------

set(CMAKE_SYSTEM_NAME      Generic)
set(CMAKE_SYSTEM_PROCESSOR arm)

# 裸机交叉编译时，CMake 默认会尝试链接一个可执行文件来探测编译器，这一步必然失败。
# 声明为 STATIC_LIBRARY 即可跳过链接测试。
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)

set(TOOLCHAIN_PREFIX  "arm-none-eabi-" CACHE STRING "GNU Arm 工具链前缀")
set(TOOLCHAIN_BIN_DIR ""              CACHE PATH   "GNU Arm 工具链 bin 目录（留空则自动在 PATH 中查找）")

# --- 定位工具链各组件 -----------------------------------------------------------
find_program(TOOLCHAIN_GCC   NAMES ${TOOLCHAIN_PREFIX}gcc     HINTS "${TOOLCHAIN_BIN_DIR}" REQUIRED)
find_program(TOOLCHAIN_GXX   NAMES ${TOOLCHAIN_PREFIX}g++     HINTS "${TOOLCHAIN_BIN_DIR}")
find_program(TOOLCHAIN_AR    NAMES ${TOOLCHAIN_PREFIX}ar      HINTS "${TOOLCHAIN_BIN_DIR}" REQUIRED)
find_program(TOOLCHAIN_OBJCOPY NAMES ${TOOLCHAIN_PREFIX}objcopy HINTS "${TOOLCHAIN_BIN_DIR}" REQUIRED)
find_program(TOOLCHAIN_OBJDUMP NAMES ${TOOLCHAIN_PREFIX}objdump HINTS "${TOOLCHAIN_BIN_DIR}")
find_program(TOOLCHAIN_SIZE  NAMES ${TOOLCHAIN_PREFIX}size    HINTS "${TOOLCHAIN_BIN_DIR}" REQUIRED)

set(CMAKE_C_COMPILER   "${TOOLCHAIN_GCC}")
set(CMAKE_ASM_COMPILER "${TOOLCHAIN_GCC}")
if(TOOLCHAIN_GXX)
    set(CMAKE_CXX_COMPILER "${TOOLCHAIN_GXX}")
endif()

set(CMAKE_AR      "${TOOLCHAIN_AR}"      CACHE FILEPATH "Archiver")
set(CMAKE_OBJCOPY "${TOOLCHAIN_OBJCOPY}" CACHE FILEPATH "Objcopy")
set(CMAKE_OBJDUMP "${TOOLCHAIN_OBJDUMP}" CACHE FILEPATH "Objdump")
set(CMAKE_SIZE    "${TOOLCHAIN_SIZE}"    CACHE FILEPATH "Size")

# --- 查找策略：程序用宿主机（构建机）的，库/头文件只去目标机的 sysroot 找 ---------
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)

# --- 输出文件后缀 ---------------------------------------------------------------
set(CMAKE_EXECUTABLE_SUFFIX_C    ".elf")
set(CMAKE_EXECUTABLE_SUFFIX_CXX  ".elf")
set(CMAKE_EXECUTABLE_SUFFIX_ASM  ".elf")
