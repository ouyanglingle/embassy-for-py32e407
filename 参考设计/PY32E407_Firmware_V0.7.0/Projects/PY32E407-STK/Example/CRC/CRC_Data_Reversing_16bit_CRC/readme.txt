================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了用户定义的生成多项式由 HAL_CRC_Init()配置。同时，输入数据反转功能设
置为全字位反转，输出数据反转功能也已启用（只有位级反转选项可用）。计算出16位长的
CRC值，将其存储在 uwCRCValue 变量中，然后与存储在 uwExpectedCRCValue_reversed 变
量中的 CRC 预期值进行比较。

Function descriptions:
In this example, the user-defined generating polynomial is configured by 
HAL_CRC_Init(). At the same time, input data reversal feature is set to bit 
reversal on full word.Output data reversal is enabled as well (only bit-level 
reversal option is available).Additionally, the default init value is discarded
and a user-defined one is used instead. The 16-bit long CRC is computed, stored
in uwCRCValue variable then compared to the CRC expected value stored in 
uwExpectedCRCValue_reversed variable.

================================================================================
测试环境：
测试用板：PY32E407_STK
MDK版本： 5.28
IAR版本： 9.20
GCC版本：GNU Arm Embedded Toolchain 10.3-2021.10

Test environment:
Test board: PY32E407_STK
MDK Version: 5.28
IAR Version: 9.20
GCC Version: GNU Arm Embedded Toolchain 10.3-2021.10
================================================================================
使用步骤：
1. 编译并下载程序；
2. 观察LED灯是否亮起，如果灯亮表示运行成功，否则运行失败

Example execution steps:
1.Compile and download the program.
2. Observe whether the LED light is on. If it is on, it indicates successful
operation, otherwise it fails

================================================================================
注意事项：

Notes:

================================================================================