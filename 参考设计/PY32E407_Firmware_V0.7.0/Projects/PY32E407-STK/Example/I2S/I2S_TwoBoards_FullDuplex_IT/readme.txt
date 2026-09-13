================================================================================
                                样例使用说明
                                Sample Description
================================================================================
功能描述:
此样例是利用中断对串口外设接口（SPI）与外部设备以全双工串行方式进行通信的演示，
主设备提供通信时钟SCK，通过MOSI/MISO引脚发送/接收数据。从设备通过MOSI/MISO引脚接
收/发送数据。数据以主机提供的SCK沿同步被移位，完成全双工通信。

Function descriptions:
This sample is a demonstration of using interrupts to communicate with a serial 
peripheral interface (SPI) and an external device in full-duplex serial mode. 
The master device provides the communication clock SCK and sends/receives data 
through the MOSI/MISO pin. The slave device receives/transmits data through the 
MOSI/MISO pins. The data is shifted synchronously along the SCK provided by the 
master to complete full-duplex communication.
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
使用步骤:
1. 选择两块 STK 板，一块作为主机，一块作为从机。
2.打开"#define BOARD_IN_MASTER_MODE"宏定义，编译程序下载到主机。
3.屏蔽"#define BOARD_IN_MASTER_MODE"宏定义，编译程序下载到从机。
4. 连接主机和从机的引脚（箭头指向为信号传输方向）：
   主机 MASTER:         从机 SLAVE:
   SD(PA7)     <---->   SD(PA7)
   WS(PA4)     ----->   WS(PA4)
   CK(PA5)     ----->   CK(PA5)
   GND         <---->   GND
5. 主机和从机上电后，按下主机和从机的复位按键。
6. 先按下从机的用户按键运行从机程序，再按下主机的用户按键运行主机程序。
7. 观察主机和从机的 LED 灯，当主机和从机的 LED 灯由常暗转为常亮状态，则表示主机和从机
   收发数据成功；当主机或从机的 LED 灯处于闪烁状态，则表示主机和从机收发数据失败。

Example execution steps:
1. Select two STK boards, one as the master and one as the slave.
2. Open the “#define BOARD_IN_MASTER_MODE” macro definition, compile the program 
   to download to the host.
3. Mask “#define BOARD_IN_MASTER_MODE” macro definition, compile the program to 
   download to the slave.
4. Connect the pins of the master and slave boards (direction indicated by the 
   arrow):
   Master MASTER:         Slave SLAVE:
   SD(PA7)     <---->   SD(PA7)
   WS(PA4)     ----->   WS(PA4)
   CK(PA5)     ----->   CK(PA5)
   GND         <---->   GND
5. Power on the master and slave boards, and press the reset buttons on both.
6. Press the user button on the slave to run the slave program first, then press 
   the user button on the master to run the master program.
7. Observe the LEDs on the master and slave boards. When the LEDs on both the 
   master and slave boards change from dim to constantly on, it indicates that 
   the master and slave have successfully transmitted and received data. If the 
   LEDs on the master or slave are blinking, it indicates a failure in data 
   transmission.
================================================================================
注意事项:

Note:
================================================================================
