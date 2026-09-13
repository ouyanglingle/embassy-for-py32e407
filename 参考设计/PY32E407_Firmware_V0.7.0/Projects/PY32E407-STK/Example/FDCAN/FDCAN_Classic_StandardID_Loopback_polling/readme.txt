================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了采用CAN2.0协议、标准帧、轮询方式的内部回环通信。

Function descriptions:
This sample demonstrates internal loopback communication using CAN2.0 protocol,
standard frames, polling.
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
2. 按下按键，调试助手打印“receive success”，则表明CAN内部回环通信成功。

Example execution steps:
1.Compile and download the program.
2.Press the key and the debugging assistant prints "receive success", which
indicates that CAN internal loop communication is successful. 
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