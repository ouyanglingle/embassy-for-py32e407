================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了通过中断方式，主机发送不定长数据，从机接收不定长数据。主机向从机发送
10字节的数据（0~9），然后从机接收数据（0~9）并通过串口打印；主机向从机发送100
字节数据（1~100），然后从机接收数据（1~100）并通过串口打印；主机向从机发送10
字节的数据（0~9），然后从机接收数据（0~9）并通过串口打印。

Function descriptions:
This example demonstrates how the host sends variable length data and the slave
receives variable length data through interrupt mode. The host sends 10 bytes of
data (0-9) to the slave, and then the slave receives the data (0-9) and prints
it through the serial port; The host sends 100 bytes of data (1-100) to the
slave, and then the slave receives the data (1-100) and prints it through the
serial port; The host sends 10 bytes of data (0-9) to the slave, and then the
slave receives the data (0-9) and prints it through the serial port.
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
3. 先复位从机，然后再复位主机。
4. 复位完成后，按下主机的用户按键，主机和从机开始通讯。
5. 若从机通过串口先后打印0~9，1~100，0~9等3帧数据，则表示主机和从机通信成功；
   否则，主机和从机通信失败。

Example execution steps:
1. turn on the “#define BOARD_IN_MASTER_MODE” macro definition, compile and 
   download the program to the host board.
2. mask the “#define BOARD_IN_MASTER_MODE” macro definition, compile and download 
   the program to the slave board;
3. Reset the slave device first, and then reset the master device.
4. After the reset, press the user button on the master device to initiate 
   communication between the master and slave.
5. If the slave prints 3 frames of data through the serial port, including 0-9,
   1-100, 0-9, it indicates successful communication between the host and the
   slave; Otherwise, communication between the host and slave will fail.
================================================================================
注意事项：
1. 
PC8  -------> I2C3_SCL
PC9  -------> I2C3_SDA
如需修改速率，直接修改I2C_SPEEDCLOCK即可。
RX_MAX_LEN定义了从机单次接收的最大数据长度（当前样例为200byte），用户可根据需要
修改从机程序main.c中宏RX_MAX_LEN的值。
2.
通过USB转TTL模块连接PC与STK板,STK板与USB转TTL模块的连线方式如下；
STK板         USB转TTL模块
PA0(TX)  -->  RX
GND      -->  GND
UART配置为波特率9600。

Notes:
1.
PC8  -------> I2C3_SCL
PC9  -------> I2C3_SDA
To modify the speed, simply change the I2C_SPEEDCLOCK value.
RX_MAX_LEN defines the maximum length of data received by the slave in a single 
pass (200byte for the current sample), and the user can modify the value of the 
macro RX_MAX_LEN in the slave program main.c according to the need.
2.
Connect the PC to the STK board through the USB to TTL module, and the connection
method between the STK board and the USB to TTL module is as follows:
STK board     USB to TTL module
PA0(TX)  -->  RX
GND      -->  GND
UART is configured as BaudRate 9600.
================================================================================