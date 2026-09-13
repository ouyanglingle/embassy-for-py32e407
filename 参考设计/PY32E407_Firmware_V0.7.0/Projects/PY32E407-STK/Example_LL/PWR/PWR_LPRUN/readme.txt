================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了进入和退出lprun模式。

Function descriptions:
This example demonstrates entering and exiting the lprun mode.
================================================================================
测试环境：
测试用板：PY3E407_STK
MDK版本： 5.28
IAR版本： 9.20
GCC版本：GNU Arm Embedded Toolchain 10.3-2021.10

Test environment:
Test board: PY3E407_STK
MDK Version: 5.28
IAR Version: 9.20
GCC Version: GNU Arm Embedded Toolchain 10.3-2021.10
================================================================================
使用步骤：
1. 编译下载程序到MCU，并运行；
2. LED灯灭，进入lprun模式；
3. 按下按键，LED灯闪烁，退出lprun模式；

Example execution steps:
1. Compile and download the program to the MCU and run it;
2. The LED lights to enter lprun mode;
3. Press key, LED blinks, exit lprun mode.
================================================================================
注意事项：
1，演示此样例功能时需要断开swd连接线并重新上电，因为默认情况下，仿真器会把
DBGMCU_CR1.DBG_STDBY置位。
2，注意关闭systick中断，防止该中断唤醒MCU。

Notes:
1.To demonstrate this sample function, it is necessary to disconnect the SWD
cable and power it back on, as by default, the emulator will reset the
DBGMCU_CR1.DBG_STDBY position.
2.Pay attention to turning off the systick interrupt to prevent it from waking
up the MCU.
================================================================================