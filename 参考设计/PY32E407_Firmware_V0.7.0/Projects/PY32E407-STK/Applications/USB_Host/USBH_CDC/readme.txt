================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了USB 主机功能，可以枚举一个通用CDC类设备。循环打印接收到的CDC设备发送
的数据。同时按下按键，会发送数据给CDC设备。

Function descriptions:
This sample demonstrates the USB host functionality, which can enumerate a
general CDC class device. It continuously prints the data sent by the connected
CDC device. Meanwhile, pressing the button will send data to the CDC device.
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
2. 将一个通用CDC类设备接到STK板上USB1_HOST接口上，可以看到串口打印“CDC deceive
connect”。
3. 使用CDC设备给主机发送数据，主机成功接受到数据后，会通过串口打印“receive data :”
和接收到的数据内容。
4. 按下按键，主机会给设备发送TX_BUFF_SIZE bytes的数据，数据内容为“0x5A”，发送数据
前会通过串口打印“Sending data ...” ，数据发送完成后打印“>> Data sent”

Example execution steps:
1. Compile and download the program;
2. Connect a generic CDC class device to the USB1_HOST interface on the STK
board. You will see the serial port print "CDC device connected".
3. Use the CDC device to send data to the host. After the host successfully
receives the data, it will print "receive data:" and the received data content
via the serial port.
4. Press the button, and the host will send TX_BUFF_SIZE bytes of data (with
the content of 0x5A) to the device. Before sending the data, "Sending data ..."
will be printed via the serial port, and after the data is sent, ">> Data sent"
will be printed.
================================================================================
注意事项：
通过USB转TTL模块连接PC与STK板,STK板与USB转TTL模块的连线方式如下；
STK板         USB转TTL模块
PA0(TX)  -->  RX
GND      -->  GND
UART配置为波特率9600。

1. 接收数据量大小可通过RX_BUFF_SIZE 调整，发送数据量大小可通过TX_BUFF_SIZE 调整

Notes:
Connect the PC to the STK board through the USB to TTL module, and the connection
method between the STK board and the USB to TTL module is as follows:
STK board     USB to TTL module
PA0(TX)  -->  RX
GND      -->  GND
UART is configured as BaudRate 9600.

1. The amount of received data can be adjusted via RX_BUFF_SIZE, and the amount
of transmitted data can be adjusted via TX_BUFF_SIZE.
================================================================================