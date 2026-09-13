================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了USART同步通信功能，USART设备作为主机、SPI设备作为从机，采用DMA方式
通信。在USART设备提供同步时钟下，主从机完成全双工通信。

Function descriptions:
This sample demonstrates USART synchronous communication, with USART devices
as masters and SPI devices as slaves, communicating in DMA mode.  With
synchronous clock provided by USART equipment, master and slave complete
full duplex communication.
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
1. 打开“#define TRANSMITTER_BOARD”宏定义，编译并下载程序到主机板。
2. 屏蔽“#define TRANSMITTER_BOARD”宏定义，编译并下载程序到从机板。
3. USART主机和SPI从机连线方式如下；
USART主机：         SPI从机：
CK(PA8)    ----->   SCK(PA5)
TX(PA9)    ----->   MOSI(PA7)
RX(PA10)   <-----   MISO(PA6)
4.主从机上电；
5.首先按下从机复位按键，运行从机程序；其次按下主机复位按键，运行主机程序；最后按下
主机用户按键，主从机通信；
6.观察现象，如果主机、从机通讯成功，则主从机小灯会亮起，反之常暗。

Example execution steps:
1. Turn on the "#define TRANSMITTER_BOARD" macro definition, compile and download 
the program to the master board. 
2. Mask the "#define TRANSMITTER_BOARD" macro definition, compile and download 
the program to the slave board. 
3. The USART master and SPI slave are wired as follows;
USART Master:       SPI Slave:
CK(PA8)    ----->   SCK(PA5)
TX(PA9)    ----->   MOSI(PA7)
RX(PA10)   <-----   MISO(PA6)
4. Power up the master and slave;
5. Firstly, press the slave reset button to run the slave program; secondly, 
press the host reset button to run the host program; finally, press the host user
key, master and slave communicate;
6. Observe the phenomenon, if the host and slave communication is successful, 
the master and slave small light will be lit, and vice versa often dark.
================================================================================
注意事项：

Notes:

================================================================================