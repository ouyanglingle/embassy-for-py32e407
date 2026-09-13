================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了USB鼠标功能。
USB鼠标主要通过USB HID类实现，将其模拟成一个USB鼠标设备。设备程序每隔1秒钟
向主机发送一次鼠标相对位置。在PC端，您可以看到鼠标每隔1秒移动一次位置。

Function descriptions:
This sample demonstrates the USB mouse functionality.
The USB mouse is mainly implemented using the USB HID class, simulating a USB 
mouse device.The device program sends the mouse relative position to the host 
every 1 second.On the PC side, you can observe the mouse moving its position 
every 1 second.
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
3. PC端鼠标每隔1秒钟移动一次位置；

Example execution steps:
1. Download and run the program;
2. Connect the STK board to the PC computer using the USB cable;
3. The mouse position will be updated every 1 second on the PC side.

================================================================================
注意事项：

Notes:
================================================================================