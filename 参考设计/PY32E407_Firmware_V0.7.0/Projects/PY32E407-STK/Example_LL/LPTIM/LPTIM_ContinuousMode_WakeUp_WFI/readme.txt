================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了LPTIM连续模式中断唤醒STOP模式。

Function descriptions:
This example demonstrates the LPTIM continuous mode interrupt wake-up STOP mode.
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
1. 编译并下载程序到MCU，并运行。
2. 按下按键进入STOP模式。
3. 163毫秒后唤醒STOP模式并循环进入STOP模式再唤醒。
4. LED等每隔163毫秒翻转一次；

Example execution steps:
1. Compile and download the program to the MCU and run it.
2. Press the button to enter STOP mode.
3. After 163 milliseconds, wake up from STOP mode and repeat the process of 
   entering STOP mode and waking up.
4. LEDs flip every 163 milliseconds.
================================================================================
注意事项：
1，演示此样例功能时需要断开swd连接线并重新上电，因为默认情况下，仿真器会把
DBGMCU_CR.DBG_STOP置位。
2，注意关闭systick中断，防止该中断唤醒MCU。

Notes:
1.To demonstrate this sample function, it is necessary to disconnect the SWD
cable and power it back on, as by default, the emulator will reset the
DBGMCU_CR.DBG_STOP position.
2.Pay attention to turning off the systick interrupt to prevent it from waking
up the MCU.
================================================================================