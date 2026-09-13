================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了USB 低功耗和remote wake up功能。
将其模拟成一个USB键盘设备用于唤醒PC，当PC睡眠时会挂起设备，设备会进入低功耗模式，
此时PC可以重新通讯以唤醒设备，如果设备想远程唤醒主机可以按下按键发出远程唤醒信号。

Function descriptions:
This sample demonstrates the USB low-power consumption and remote wake-up 
functionality. It simulates a USB keyboard device to wake up a PC. When the PC 
enters sleep mode, it suspends the device, which then transitions into a 
low-power consumption mode. During this time, the PC can re-establish 
communication to wake up the device. If the device wants to remotely wake up the
host, it can press a button to send a remote wake-up signal.
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
3. 当PC睡眠时，设备会进入低功耗，放PC重新发起通讯时，设备会退出低功耗
4. 当PC睡眠时，按下按键，设备会远程唤醒主机
5. startkit上JP3的PA9和VBUS需要短接

Example execution steps:
1. Download and run the program;
2. Connect the STK board to the PC computer using the USB cable;
3. When the PC is in sleep mode, the device will enter low power consumption. 
When the PC initiates communication again, the device will exit low power consumption.
4. When the PC is in sleep mode, pressing the button will allow the device to 
remotely wake up the host.
5. The PA9 and VBUS of JP3 on startkit need to be short-circuited
================================================================================
注意事项：

Notes:
================================================================================