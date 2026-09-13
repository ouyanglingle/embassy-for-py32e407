================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了COMP比较器轮询功能，PB1作为比较器正端输入，1/2VREFINTF作为比较器负端输
入，当PB1的电压大于1/2VREFINT电压时，LED灯亮，小于1/2VREFINT电压时,LED灯灭。

Function descriptions:
This example demonstrates the polling function of the COMP comparator, with PB1
as the positive input and 1/2VREFINT as the negative input. When the voltage of PB1
is greater than 1/2VREFINT voltage, the LED lights up, and when it is less than 
1/2VREFINT voltage, the LED lights up.
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
2. 配置PB1输入1.03V电压，LED灯亮
3. 配置PB1输入1.02V电压, LED灯灭

Example execution steps:
1.Compile and download the program to MCU and run it;
2.Configure PB1 input voltage of 1.03V, LED light on
3.Configure PB1 input voltage of 1.02V, LED light off
================================================================================
注意事项：

Notes:

================================================================================