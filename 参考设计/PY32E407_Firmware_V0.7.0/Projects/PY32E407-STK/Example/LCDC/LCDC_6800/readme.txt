================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了LCDC驱动6800 LCD屏的功能。

Function descriptions:
This sample demonstrateshe the function of LCDC to drive the LCD screen.
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
2. STK板与LCD模组的连线方式如下；
STK板         LCD模块
PD7       -->  CS
PF4       -->  RS
PD4       -->  RW
PD5       -->  EN
PD14      -->  D0
PD15      -->  D1
PD0       -->  D2
PD1       -->  D3
PE7       -->  D4
PE8       -->  D5
PE9       -->  D6
PE10      -->  D7
GND       -->  GND
3. LCD屏显示 "PUYA"

Example execution steps:
1.Compile and download the program.
2.The connection between the STK board and the LCD module is as follows：
STK board    LCD module
PD7       -->  CS
PF4       -->  RS
PD4       -->  RW
PD5       -->  EN
PD14      -->  D0
PD15      -->  D1
PD0       -->  D2
PD1       -->  D3
PE7       -->  D4
PE8       -->  D5
PE9       -->  D6
PE10      -->  D7
GND       -->  GND
3.The LCD screen displays "PUYA"

================================================================================
注意事项：
样例使用HS0802模组

Notes:
The example uses the HS0802 module
================================================================================