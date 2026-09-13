================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了USB 主机功能，可以枚举一个HID mouse设备。循环打印鼠标的当前位置信息。

Function descriptions:
This sample demonstrates the USB host functionality, capable of enumerating a HID
(Human Interface Device) mouse and continuously printing the current position 
information of the mouse in a loop.
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
2. 将一个标准鼠标设备接到STK板上USB1_HOST接口上，可以看到串口打印“mouse connect”,
然后鼠标移动，串口打印位置信息

Example execution steps:
1.Compile and download the program.
2.Connect a standard mouse device to the USB1_HOST interface on the STK board, 
and you will see the serial port print "mouse connect". Then, as you move the 
mouse, the serial port will print the position information.
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