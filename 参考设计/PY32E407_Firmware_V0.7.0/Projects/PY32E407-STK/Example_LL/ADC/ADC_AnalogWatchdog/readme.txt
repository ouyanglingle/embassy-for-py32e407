================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了ADC的模拟看门狗功能，当开启模拟看门狗通道的电压值超过上下限时，会进
入看门狗中断。

Function descriptions:
This example demonstrates the analog watchdog function of ADC. When the voltage
value of the analog watchdog channel exceeds the upper and lower limits, it will
enter the watchdog interrupt.
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
1. 编译并下载程序
2. 通道11(PC0)的电压值小于1.65V，LED灯不会被点亮
3. 通道11(PC0)的电压值大于1.65V，LED灯被点亮。

Example execution steps:
1. Compile and download the program and run it
2. the voltage value of channel 11 (PC0) is lower than 1.65V, the LED off
3. the voltage value of channel 11 (PC0) is higher than 1.65V, the LED on
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