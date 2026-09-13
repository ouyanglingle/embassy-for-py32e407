================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了主机SPI通过轮询方式进行通讯，从机使用FLASH外设芯片P25Q64，按下user
按键，主机先向从机写15bytes数据为0x1~0xf，然后再从FLASH中将写入的数据读出，读取成
功后，主机板上的小灯处于“常亮”状态。

Function descriptions:
This sample demonstrates the host SPI communication through polling, the slave 
uses FLASH peripheral chip P25Q64, press the user button, the host first to the 
slave to write 15bytes of data for the 0x1 ~ 0xf, and then from the FLASH will 
be written to read out the data, read the success of the host board, the small 
light is in the “always on” state! After successful reading, the small light on 
the host board is in the “always on” state.
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
2. 按下主机user按键，擦除外接FLASH第一个Sector页；
3. 写15bytes数据0x1-0xf到刚擦除的Sector页；
4. 从FLASH的第一个Sector页读取15bytes数据；
5. 比较读出的数据与写入的数据是否相同。主机的LED灯由常暗转为常亮状态，则表明主机
收发数据成功；当主机LED灯处于闪烁状态，则表明主机收发数据失败。

Example execution steps:
1. compile and download the program to MCU and run it;
2. Press the host user button to erase the first Sector of the external FLASH;
3. Write 15bytes of data 0x1-0xf to the Sector that was just erased;
4. read 15bytes of data from the first Sector of FLASH;
5. Compare whether the read data and the written data are the same. When the LED 
of the host turns from dark to light, it indicates that the host is successful in
sending and receiving data; when the LED of the host is in flashing state, it 
indicates that the host fails to send and receive data.
================================================================================
注意事项：
1、主机与从机引脚连接(箭头指向为信号传输方向) 
主机MASTER：         FLASH：
SCK(PB3)   ----->    SCK
MISO(PA6)  <-----    MISO
MOSI(PA7)  ----->    MOSI
NSS(PA4)   ----->    NSS
2、本样例使用SPI与板载FLASH通信。需将电阻R50、R51、R52、R53、R55、R56移除，将电
阻R43、R44、R46、R47、R48、R49焊接，否则通信失败。（具体参考开发板用户指南）

Note:
1、The host is connected to the SLAVE pin (the arrow points in the signal 
transmission direction).
MASTER：             FLASH：
SCK(PB3)   ----->    SCK
MISO(PA6)  <-----    MISO
MOSI(PA7)  ----->    MOSI
NSS(PA4)   ----->    NSS
2、This sample uses SPI to communicate with the on-board FLASH, it is necessary
to remove resistors R50, R51, R52, R53, R55, R56, and weld resistors R43, R44,
R46, R47, R48, R49, otherwise the communication fails.(Refer to the development
board user's guide for details)
================================================================================