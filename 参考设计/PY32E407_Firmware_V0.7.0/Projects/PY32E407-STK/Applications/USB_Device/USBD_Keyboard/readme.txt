================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了USB键盘功能。
USB键盘主要通过USB HID类实现，将其模拟成一个USB键盘设备。设备程序运行后，1s切换
一次大小写

Function descriptions:
This sample demonstrates the USB keyboard functionality.
The USB keyboard is mainly implemented using the USB HID class, simulating a 
USB keyboard device. After the device program runs, it toggles the case 
sensitivity (caps lock state) once every 1 second
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
1. 下载并运行程序；
2. 通过USB连接线，一端连接STK板，另一端连接PC电脑；
3. 设备程序运行后，1s切换一次大小写

Example execution steps:
1. Download and run the program;
2. Connect the STK board to the PC computer using the USB cable;
3. After the device program runs, it toggles the case sensitivity 
(caps lock state) once every 1 second.
================================================================================
注意事项：

Notes:
================================================================================