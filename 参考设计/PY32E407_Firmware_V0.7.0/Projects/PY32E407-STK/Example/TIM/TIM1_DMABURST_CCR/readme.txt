================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了在TIM1中使用DMA连续两次burst传输数据的功能，burst每传输一次更新两
个寄存器，CCR1,CCR2,TIM1的通道1和通道2输出PWM信号，通过逻辑分析仪监测，可看到
两个通道都输出周期为100ms的PWM信号，信号的占空比会从第一次的20%，第二次20%，第
三次50%,第四次及后续变为80%，此时两次burst传输完成，并且CCR1,CCR2均更新完毕。

Function descriptions:
This sample demonstrates the use of DMA in TIM1 two consecutive burst
transmission of data, burst every transmission to update the two registers,
CCR1, CCR2, TIM1 channel 1 and channel 2 output PWM signals, through the
logic analyser to monitor the output of the two channels can be seen as a
period of 100ms of the PWM signal, the duty cycle of the signal will be from
the first time the 20%, the second 20%, the third 50%, the fourth and
subsequent to 80%, when the two burst transmission is complete, and CCR1,
CCR2 are updated.
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
1. 编译下载程序并运行；
2. 通过逻辑分析仪捕捉上述两个通道的电平，判断pwm输出信号是否正确

Example execution steps:
1. compile and download the program to MCU and run it;
2.Capture the level of the above two channels through the logic analyzer to 
determine whether the pwm output signal is correct
================================================================================
注意事项：
1. 引脚
CH1   ->  PA8
CH2   ->  PA9

Notes:
1. Pins
CH1   ->  PA8
CH2  ->  PA9
================================================================================