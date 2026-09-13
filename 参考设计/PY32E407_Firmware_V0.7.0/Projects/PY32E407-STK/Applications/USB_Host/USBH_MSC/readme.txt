================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了USB 主机功能，可以枚举一个USB U盘设备。在U盘设备中创建一个USBDemo.txt
文件，并在文件中写入"USB Host Library : Mass Storage Example"。

Function descriptions:
This sample demonstrates the USB host functionality, which can enumerate a USB 
flash drive device, create a ​​USBDemo.txt​​ file on the USB device, and write the 
text ​​"USB Host Library: Mass Storage Example"​​ into the file
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
2. 将一个标准U盘设备接到STK板上USB1_HOST接口上
3. 看到串口打印“INFO : FatFs data compare SUCCES” 时,在U盘中成功创建了一个
USBDemo.txt文件，并在文件中写入"USB Host Library : Mass Storage Example"

Example execution steps:
1.Compile and download the program.
2.Connect a standard USB flash drive to the ​​USB1_HOST​​ interface on the STK board.
3.When the serial port prints ​​"INFO: FatFs data compare SUCCES"​​, a ​​USBDemo.txt​​ 
file has been successfully created on the USB flash drive, and the text ​​"USB Host 
Library: Mass Storage Example"​​ is written into the file.
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