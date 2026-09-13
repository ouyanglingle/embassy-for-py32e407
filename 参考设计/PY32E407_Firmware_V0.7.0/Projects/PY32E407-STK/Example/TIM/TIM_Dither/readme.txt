================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
本例程输出抖动模式下的PWM信号

Function descriptions:
This sample outputs the PWM signal in dithering mode

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
2. 用逻分或者示波器连接通道PA8(TIM1_CH1)
3. 查看发现PWM信号周期为1ms和1.1ms交替，每个周期高电平时长都为500us

Example execution steps:
1. compile and download the program to MCU and run it;
2. Observe channel PA8(TIM1_CH1) with logic analyzer or oscilloscope
3. The PWM signal period was found to be alternating between 1ms and 1.1ms,
and the duration of the high level in each cycle was 500us.

================================================================================
注意事项：
若想要改变抖动模式下16个周期内发生抖动的周期数量，可以改变
APP_SetDitherFractionnal_ARR(&TimHandle, 0X8);中0X8的值，范围为0X0~0XF

Notes:
If you want to change the number of cycles in which dithering occurs within 16
cycles in dithering mode, you can change the value of 0X8 in
APP_SetDitherFractionnal_ARR(&TimHandle, 0X8); in the range of 0X0~0XF
================================================================================