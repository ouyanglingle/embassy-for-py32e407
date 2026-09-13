================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了I2C通过DMA方式进行通讯，主机先向从机发送15byte数据，然后再接收从机
发送的15byte数据,主机、从机接收数据成功后，主机和从机板上的小灯处于“常亮”状态。

Function descriptions:
This sample demonstrates I2C communication using DMA. The master device 
sends 15 bytes of data to the slave device and then receives 15 bytes of data 
from the slave. When both the master and slave successfully transmit and receive
data, the LEDs on both boards will be constantly lit.
================================================================================
测试环境：
测试用板：PY32E407_STK
MDK版本： 5.28
IAR版本： 9.20
GCC 版本：GNU Arm Embedded Toolchain 10.3-2021.10

Test environment:
Test board: PY32E407_STK
MDK Version: 5.28
IAR Version: 9.20
GCC Version: GNU Arm Embedded Toolchain 10.3-2021.10
================================================================================
使用步骤：
1. 打开“#define BOARD_IN_MASTER_MODE”宏定义，编译下载程序到主机板；
2. 屏蔽“#define BOARD_IN_MASTER_MODE”宏定义，编译下载程序到从机板；
3. 先复位从机，然后再复位主机
4. 复位完成后，按下主机user按键，主从开始通讯
5. 观察主从机的LED灯，当主机和从机LED灯由常暗转为常亮状态，则表明主机、从机收发数
   据成功；当主机或从机LED灯处于闪烁状态，则表明主机、从机收发数据失败。

Example execution steps:
1. turn on the “#define BOARD_IN_MASTER_MODE” macro definition, compile and 
   download the program to the host board.
2. mask the “#define BOARD_IN_MASTER_MODE” macro definition, compile and download 
   the program to the slave board;
3. Reset the slave first, and then reset the master.
4. After the reset is complete, press the user button on the master to initiate 
   communication between master and slave.
5. Observe the LEDs on the master and slave boards. When both LEDs are constantly 
   lit, it indicates successful data transmission and reception between the master 
   and slave. If any of the LEDs are blinking, it indicates data transmission or 
   reception failure.
================================================================================
注意事项：
PC8  -------> I2C3_SCL
PC9  -------> I2C3_SDA
如需修改速率，直接修改I2C_SPEEDCLOCK即可。

Notes:
PC8  -------> I2C3_SCL
PC9  -------> I2C3_SDA
To modify the speed, simply change the I2C_SPEEDCLOCK value.
================================================================================