/**
  ******************************************************************************
  * @file    ll_shim.c
  * @brief   ll_shim.h 的"定义模式"实例化 —— 为 LL 头里的 static inline 函数
  *          生成外部可见包装（<官方名>__ext）。
  ******************************************************************************
  * 本文件本身不写任何函数体：函数体由 ll_shim.h 里的 SHIM_* 表在
  * `PY32_LL_SHIM_IMPL` 打开时展开出来。
  *
  * 与 bindgen 的分工（同一份头、两种展开）：
  *   - build.rs 的 cc 编译**本文件** → 展开出定义 → 符号进 libpy32e407_ll.a；
  *   - build.rs 的 bindgen 解析 wrapper.h（其中 include 了 ll_shim.h，
  *     不带 PY32_LL_SHIM_IMPL）→ 展开出原型 → 生成 Rust 声明。
  *
  * 因此**新增/修改 shim 函数只需改 ll_shim.h 一处**，两边自动同步。
  * 详见 ll_shim.h 顶部的说明。
  */
#define PY32_LL_SHIM_IMPL
#include "ll_shim.h"
