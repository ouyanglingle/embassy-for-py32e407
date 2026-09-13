================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了两块电路板之间在中断模式下进行LPUART传输，发送/接收在两块电路板之间
进行。LPUART配置为115200，数据位8，停止位1，校验位None。

Function descriptions:
This sample demonstrates LPUART transmission between two boards in interrupt mode,
with transmit/receive taking place between the two boards.The LPUART is configured
for 115200, with data bit 8, stop bit 1, and parity bit None.
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
1. 开启“#define TRANSMITTER_BOARD”宏定义，编译下载程序到开发板1。
2. 屏蔽“#define TRANSMITTER_BOARD”宏定义，编译下载程序到开发板2。
3. 首先按下开发板2复位按键，运行程序; 其次按下开发板1复位按键，最后按下开发板1上
的用户按键。
4. 开发板1发送数据到开发板2，开发板2接收完数据后，再发送数据到开发板1上。
5. 开发板1比较发送的数据和接收的数据。如果相同，则LED2亮起，否则LED2熄灭。
6. 开发板2比较发送的数据和接收的数据。如果相同，则LED2亮起，否则LED2熄灭。

Example execution steps:
1. Enable the "#define TRANSMITTER_BOARD" macro definition, compile and download
the program to development board 1.
2. Mask the "#define TRANSMITTER_BOARD" macro definition, compile and download
the program to the development board 2.
3. First, press the reset button on board 2 to run the program; second, press the
reset button on board 1, and finally press the user button on board 1.
4. Development board 1 sends data to development board 2, and after development 
board 2 receives the data, it sends data to development board 1.
5. Development board 1 compares the sent data with the received data. If they are
the same, then LED2 lights up, otherwise LED2 goes off.
6. Development board 2 compares the sent data with the received data. If they are
the same, then LED2 lights up, otherwise LED2 goes off.

================================================================================
注意事项：
两个开发板的连线方式如下；
   _________________________                       _________________________
  |           ______________|                     |______________           |
  |          |LPUART        |                     |LPUART        |          |
  |          |              |                     |              |          |
  |          |           TX |_____________________| RX           |          |
  |          |              |                     |              |          |
  |          |              |                     |              |          |
  |          |              |                     |              |          |
  |          |           RX |_____________________| TX           |          |
  |          |              |                     |              |          |
  |          |______________|                     |______________|          |
  |                         |                     |                         |
  |                         |                     |                         |
  |                      GND|_____________________|GND                      |
  |__PY32_Board 1___________|                     |__PY32_Board 2___________|

STK板1        STK板2
Tx(PC01)  --> Rx(PC0)
Rx(PC0)   <-- Tx(PC01)
GND       --> GND

Notes:
The two development boards are wired as follows;
   _________________________                       _________________________
  |           ______________|                     |______________           |
  |          |LPUART        |                     |LPUART        |          |
  |          |              |                     |              |          |
  |          |           TX |_____________________| RX           |          |
  |          |              |                     |              |          |
  |          |              |                     |              |          |
  |          |              |                     |              |          |
  |          |           RX |_____________________| TX           |          |
  |          |              |                     |              |          |
  |          |______________|                     |______________|          |
  |                         |                     |                         |
  |                         |                     |                         |
  |                      GND|_____________________|GND                      |
  |__PY32_Board 1___________|                     |__PY32_Board 2___________|

STK Board 1   STK Board 2
Tx(PC01)  --> Rx(PC0)
Rx(PC0)   <-- Tx(PC01)
GND       --> GND

================================================================================