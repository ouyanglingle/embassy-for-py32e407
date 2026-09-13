================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述:
此样例演示了ESMC对PSRAM的XIP模式的读写功能。对AP1604进行XIP模式写入1024个数据，
读取回来并且比较。

Function descriptions:
This example demonstrates ESMC's read and write functionality for XIP mode of 
PSRAM. Write 1024 data to AP1604 and Read the data back in XIP modes and compare
them.
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
使用步骤:
1.编译下载程序，并运行.
2.程序对PSRAM 从0x90000000地址开始写入1024个0xFFFFFFFF,读取回来并比较。
3.程序对PSRAM 从0x90000000地址开始写入0x0到0x3ff,读取回来并比较。
4.以上比较的数据都正确的话LED灯亮，否则LED灭

Example execution steps:
1.Compile and download the program to MCU and run it;
2. The program writes 1024 0xFFFFFFFs to PSRAM starting from address 0x90000000.
Read it back and compare it.
3. The program writes 0-0x3ff to PSRAM starting from address 0x90000000.Read it 
back, and compares it.
4. If the data compared above is correct, the LED light will turn on; otherwise,
the LED will turn off
================================================================================
注意事项:
PY32E407-START V2.0 上R43/R44/R46/R47/R48/49需移除.R50/R51/R52/R53/R55/R56需保留。
接线方式:
AP1604           MCU
CLK         ->   PE10
SS0（CS）   ->   PE11
IO0（SI）   ->   PE15
IO1（SO）   ->   PE13
IO2         ->   PE14
IO3         ->   PE9

Notes:
R43/R44/R46/R47/R48/49 on PY32E407-START V2.0 needs to be removed,R50/R51/R52/
R53/R55/R56 to be retained.
Wiring method:
AP1604           MCU
CLK         ->   PE10
SS0（CS）   ->   PE11
IO0（SI）   ->   PE15
IO1（SO）   ->   PE13
IO2         ->   PE14
IO3         ->   PE9

================================================================================