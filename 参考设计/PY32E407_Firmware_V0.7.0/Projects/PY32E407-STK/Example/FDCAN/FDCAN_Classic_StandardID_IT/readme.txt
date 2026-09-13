================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了采用CAN2.0协议标准帧中断方式与PCAN-View的通信功能，MCU首先自动向
PCAN-View发送ID为0x111的2帧数据，PCAN-View接收到数据后，然后手动通过PCAN-View
向MCU发送ID为0x111的2帧数据。

Function descriptions:
This sample demonstrates the communication function between CAN2.0 protocol
standard frame interrupt mode and PCAN-View. MCU automatically sends 2 frames
of data with ID 0x111 to PCAN-View at first. After PCAN-View receives the data,
MCU manually sends 2 frames of data with ID 0x111 to MCU through PCAN-View.
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
2. 通过USB转CAN模块连接PC、CAN转接板、STK板,连线方式如下；
STK板               CAN转接板
PE0(CAN_RX)  -->     CAN_RX  
PE1(CAN_TX)  -->     CAN_TX
GND          -->     GND

CAN转接板           USB转CAN模块
CAN-H         -->    CAN-H
CAN-L         -->    CAN-L
3. 打开PCAN-View，配置PCAN-View连接方式为：CAN2.0，Clock Frequency：20MHz，
Nominal Bit Rate：500kBit/s，Data Bit Rate：不设置，Standard
4. 复位STK板，观察PCAN-View是否接先后收到ID为0x111的2帧数据（数据长度均为8byte），
第1帧数据为：0x10, 0x32, 0x54, 0x76, 0x98, 0x00, 0x11, 0x22；
第2帧数据为：0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00；
如果PCAN-View收到这样的2帧数据，则MCU侧CAN发送数据成功；否则发送数据失败
5. PCAN-View向MCU侧CAN，先后发送ID为0x111的2帧数据（数据长度均为8byte），
第1帧数据为：0x10, 0x32, 0x54, 0x76, 0x98, 0x00, 0x11, 0x22；
第2帧数据为：0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00；
如果MCU侧打印“receive success”，则表明MCU侧CAN接收数据成功；否则接收失败。

Example execution steps:
1.Compile and download the program.
2. Connect the PC, CAN adapter board, and STK board through the USB to CAN module,
and the connection method is as follows:
STK board           CAN adapter board
PE0(CAN_RX)  -->     CAN_RX  
PE1(CAN_TX)  -->     CAN_TX
GND          -->     GND

CAN adapter board     USB to CAN module
CAN-H           -->    CAN-H
CAN-L           -->    CAN-L
3. Open PCAN View and configure the PCAN View connection method as CAN2.0, Clock
Frequency: 20MHz, Nominal Bit Rate: 500kBit/s, Data Bit Rate: not set, Standard
4. Reset STK board and observe whether PCAN-View receives 2 frames of data with
ID 0x111 successively (data length is 8 bytes)，
The first frame of data is: 0x10, 0x32, 0x54, 0x76, 0x98, 0x00, 0x11, 0x22；
The second frame of data is: 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00；
If PCAN-View receives such 2 frames of data, MCU side CAN sends data
successfully; otherwise, sending data fails. 
5. PCAN-View sends 2 frames of data with ID of 0x111 to CAN of MCU side
successively (data length is 8 bytes)，
The first frame of data is: 0x10, 0x32, 0x54, 0x76, 0x98, 0x00, 0x11, 0x22；
The second frame of data is: 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00；
If the MCU side prints "receive success", it indicates that the MCU side CAN
receives data successfully; otherwise, the reception fails.
================================================================================
注意事项：
通过USB转TTL模块连接PC与STK板,STK板与USB转TTL模块的连线方式如下；
STK板         USB转TTL模块
PA0(TX)  -->  RX
GND      -->  GND
UART配置为波特率9600。

Notes:
Connect the PC to the STK board through the USB to TTL module, and the connection
method between the STK board and the USB to TTL module is as follows:
STK board     USB to TTL module
PA0(TX)  -->  RX
GND      -->  GND
UART is configured as BaudRate 9600.
================================================================================