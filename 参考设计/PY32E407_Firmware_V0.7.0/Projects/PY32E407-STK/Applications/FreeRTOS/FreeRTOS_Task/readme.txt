================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了FreeRTOS三个任务依照优先级大小依次切换的实验。通过串口log，观察任务
切换顺序。

Function descriptions:
This sample demonstrates an experiment in which three FreeRTOS tasks are 
switched in order of priority. Observe the switching order through the serial 
port log.

================================================================================
测试环境：
测试用板：PY32E407_STK
MDK版本：5.28

Test environment:
Test board: PY32E407_STK
MDK Version: 5.28
================================================================================
使用步骤：
1. 编译下载程序到MCU，并运行；
2. 通过串口调试助手打印的log信息，观察任务执行过程。
   任务执行顺序为：Task3->Task2->Task1
   Task1: 打印"Task1 is running"信息。
   Task2: 打印"Task2 is running"信息。
   Task3: 打印"Task3 is running"信息。

Example execution steps:
1. Compile and download the program to the MCU and run it;
2. You can use the log information printed by the serial debugging assistant to 
observe the task execution process.
   Task execution sequence: Task3->Task2->Task1
   Task1: Print the "Task1 is running" message.
   Task2: Print the "Task2 is running" message.
   Task3: Print the "Task3 is running" message.

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
