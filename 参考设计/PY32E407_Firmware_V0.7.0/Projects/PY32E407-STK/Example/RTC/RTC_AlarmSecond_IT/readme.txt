================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示RTC的秒中断和闹钟中断功能，每次秒中断，在中断函数中会打印字符
“RTC_IT_SEC”，并且输出实时时间。

Function descriptions:
This sample demonstrates the second interrupt and alarm interrupt functionality 
of the RTC. Each time the second interrupt occurs, the interrupt function will 
print the string "RTC_IT_SEC" and output the current RTC count time.
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
1. 编译并下载程序到MCU；
2. 按下STK复位按键，调试助手每秒钟都能收到字符串：RTC_IT_SEC和当前RTC计数时间

Example execution steps:
1.Compile and download the program to the MCU.
2.Press the reset button on the STK board. The serial terminal should receive 
the string "RTC_IT_SEC" and the current RTC count time every second.
================================================================================
注意事项：
通过USB转TTL模块连接PC与STK板,STK板与USB转TTL模块的连线方式如下；
STK板         USB转TTL模块
PA0(TX)  -->  RX
GND      -->  GND
UART配置为波特率9600。

Notes:
Connect the PC to the STK board through the USB to TTL module, and the connection
method between the STK board and the USB to TTL module is as follows:
STK board     USB to TTL module
PA0(TX)  -->  RX
GND      -->  GND
UART is configured as BaudRate 9600.
================================================================================