================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了采用CANFD协议扩展帧轮询方式与PCAN-View的通信功能，MCU首先自动向
PCAN-View发送64byte数据0x0~0x3F，PCAN-View接收到数据后，然后手动通过PCAN-View
向MCU发送ID为0x1234567F的64byte数据，MCU会自动将接收到数据通过串口打出。

Function descriptions:
This example demonstrates the communication function between the CANFD protocol
extension frame polling method and the PCAN View. The MCU first automatically
sends 64byte data 0x0~0x3F to the PCAN View. After the PCAN View receives the
data, it manually sends 64byte data with ID 0x1234567F to the MCU through the
PCAN View. The MCU will automatically print the received data through the serial
port.
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
PE2(CAN_RX)  -->     CAN_RX  
PE3(CAN_TX)  -->     CAN_TX
GND          -->     GND

CAN转接板           USB转CAN模块
CAN-H         -->    CAN-H
CAN-L         -->    CAN-L
3. 打开PCAN-View，配置PCAN-View连接方式为：CANFD，Clock Frequency：20MHz，
Nominal Bit Rate：500kBit/s，Data Bit Rate：2MBit/s，Extended
4. 复位STK板，观察PCAN-View是否接收到64byte数据0x0~0x3F，PCAN-View接收到表明
STK板发送数据成功，否则表明STK板发送数据失败；
5. PCAN-View接收到数据后，手动通过PCAN-View发送ID为0x12345678的64byte数据，
观察串口是否打印PCAN-View发送的数据，如果串口打印，表明STK板接收数据成功，否则
STK板接收数据失败。

Example execution steps:
1.Compile and download the program.
2. Connect the PC, CAN adapter board, and STK board through the USB to CAN module,
and the connection method is as follows:
STK board           CAN adapter board
PE2(CAN_RX)  -->     CAN_RX  
PE3(CAN_TX)  -->     CAN_TX
GND          -->     GND

CAN adapter board     USB to CAN module
CAN-H           -->    CAN-H
CAN-L           -->    CAN-L
3. Open PCAN View and configure the PCAN View connection method as CANFD, Clock
Frequency: 20MHz, Nominal Bit Rate: 500kBit/s, Data Bit Rate: 2MBit/s, Extended
4. Reset the STK board and observe if PCAN View has received 64byte data from
0x0 to 0x3F. PCAN View's reception indicates that the STK board has successfully
sent data, otherwise it indicates that the STK board has failed to send data;
5. After receiving the data, PCAN View manually sends 64byte data with ID
0x12345678 through PCAN View. Observe whether the serial port prints the data
sent by PCAN View. If the serial port prints, it indicates that the STK board
has successfully received the data, otherwise the STK board has failed to
receive the data.
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