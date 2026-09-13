================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了USB虚拟串口功能。
USB虚拟串口主要通过USB CDC ACM类实现，将其模拟成一个VCP设备，当插在电脑上时，
可以显示成一个串口设备。

Function descriptions:
This sample demonstrates the USB virtual serial port (VCP) functionality . The 
USB VCP is mainly implemented using the USB CDC ACM class, simulating a VCP 
device. When connected to a computer, it appears as a serial port device. 
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
2. 运行,电脑端出现一个虚拟串口设备，可以使用该设备和外部串口通讯，连接方法如下：
STK 板     串口模块
PC12(TX) --> RX
PD2(RX)  --> TX
GND --> GND

Example execution steps:
1.Compile and download the program.
2.Run, the computer side appears a virtual serial port device, you can use the 
device and external serial port communication, connection method is as follows:
STK board   serial port 
PC12(TX) --> RX
PD2(RX)  --> TX
GND --> GND

================================================================================
注意事项：
不支持全双工

Notes:
Full duplex is not supported
================================================================================