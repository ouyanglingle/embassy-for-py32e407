================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了ADC模块的Tempsensor功能和外部触发功能，并通过串口打印出温度值。

Function descriptions:
This example demonstrates the Tempsensor function and external trigger function 
of the ADC module, and prints the temperature value through the serial port.
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
1. 编译下载程序到MCU，并运行；
2. 串口每隔1s打印一次采集到的温度值。

Example execution steps:
1. Compile and download the program and run it;
2. The serial port prints the collected temperature values every 1 second.

================================================================================
注意事项：
1. PY32E407-START V2.0 上L2/C38必须焊接.
2. 通过USB转TTL模块连接PC与STK板,STK板与USB转TTL模块的连线方式如下；
STK板         USB转TTL模块
PA0(TX)  -->  RX
GND      -->  GND
UART配置为波特率9600。

Notes:
1. The L2/C38 on PY32E407-START V2.0 must be welded.
2. Connect the PC to the STK board through the USB to TTL module, and the connection
method between the STK board and the USB to TTL module is as follows:
STK board     USB to TTL module
PA0(TX)  -->  RX
GND      -->  GND
UART is configured as BaudRate 9600.
================================================================================