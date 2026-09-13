================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了LCDC驱动LCD屏的功能。

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
PC13      -->  BK
PC11      -->  RST
PD7       -->  CS
PF4       -->  RS
PD4       -->  RD
PD5       -->  WR
PD14      -->  D0
PD15      -->  D1
PD0       -->  D2
PD1       -->  D3
PE7       -->  D4
PE8       -->  D5
PE9       -->  D6
PE10      -->  D7
PE11      -->  D8
PE12      -->  D9
PE13      -->  D10
PE14      -->  D11
PE15      -->  D12
PD8       -->  D13
PD9       -->  D14
PD10      -->  D15
GND       -->  GND
3. LCD屏显示蓝色

Example execution steps:
1.Compile and download the program.
2.The connection between the STK board and the LCD module is as follows：
STK board    LCD module
PC13      -->  BK
PC11      -->  RST
PD7       -->  CS
PF4       -->  RS
PD4       -->  RD
PD5       -->  WR
PD14      -->  D0
PD15      -->  D1
PD0       -->  D2
PD1       -->  D3
PE7       -->  D4
PE8       -->  D5
PE9       -->  D6
PE10      -->  D7
PE11      -->  D8
PE12      -->  D9
PE13      -->  D10
PE14      -->  D11
PE15      -->  D12
PD8       -->  D13
PD9       -->  D14
PD10      -->  D15
GND       -->  GND
3.The LCD screen displays blue

================================================================================
注意事项：
样例使用LCD模组使用ILI9341驱动

Notes:
The example uses the ILI9341 driver for the LCD module
================================================================================