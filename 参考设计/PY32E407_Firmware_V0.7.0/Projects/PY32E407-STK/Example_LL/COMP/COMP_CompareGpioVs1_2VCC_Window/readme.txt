================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了COMP比较器的window功能，比较器1正端用比较器2的正端(PA7)作为输入，
1/2VCC作为比较器负端输入，当PA7的电压值小于1.65V时,LED灯灭，大于1.65V时,LED灯亮。

Function descriptions:
This example demonstrates the window function of the COMP. The positive terminal
of the COMP1 is connected the positive terminal of the COMP2(PA7).1/2VCC used 
as the negative terminal of the COMP1.When the voltage value of PA7 is less 
than 1.65V, the LED is off, and when it is greater than 1.65V, the LED is on.
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
2. PA7输入大于1.65V电压，LED灯亮；
3. PA7输入小于1.65V电压，LED灯灭。

Example execution steps:
1. Compile and download the program and run it
2. when the PA7 input is greater than 1.65V,LED on
3. When the PA7 input is less than 1.65V,LED off
================================================================================
注意事项：

Notes:

================================================================================