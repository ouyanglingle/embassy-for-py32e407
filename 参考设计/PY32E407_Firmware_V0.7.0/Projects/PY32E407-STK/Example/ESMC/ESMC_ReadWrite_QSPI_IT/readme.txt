================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述:
此样例演示了ESMC在间接模式下的中断数据传输功能，对P25Q128H的芯片进行擦除，写入数据，
读取数据，然后把读取的数据和写入的数据进行对比，数据正确则LED灯亮，否则LED灯熄灭。

Function descriptions:
This example demonstrates the interrupt data transmission function of ESMC in
indirect mode, erasing, writing data, reading data on the P25Q128H chip, and then
comparing the read data with the written data. If the data is correct, the LED
will ON, otherwise the LED will off.
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
2.程序对FLASH进行Sector擦除，并写入4096个字节.
3.读取4096个字节，并进行比较，如果正确则LED亮，否则LED灭.

Example execution steps:
1.Compile and download the program to MCU and run it;
2. The program performs sector erase on FLASH and writes 4096 bytes
3. Read 4096 bytes and compare them. If they are correct, the LED will light up;
otherwise, the LED will turn off
================================================================================
注意事项:
PY32E407-START V2.0 上R43/R44/R46/R47/R48/49需移除.R50/R51/R52/R53/R55/R56需保留。
接线方式:
P25Q128H         MCU
CLK         ->   PA3
SS5（CS）   ->   PD2
IO0（SI）   ->   PB1
IO1（SO）   ->   PB0
IO2（WP）   ->   PA7
IO3（HOLD） ->   PA6

跳转后的程序的SystemCoreClock默认值需设置和当前程序的SystemCoreClock值一致。

Notes:
R43/R44/R46/R47/R48/49 on PY32E407-START V2.0 needs to be removed,R50/R51/R52/
R53/R55/R56 to be retained.
Wiring method:
P25Q128H         MCU
CLK         ->   PA3
SS5 (CS)    ->   PD2
IO0 (SI)    ->   PB1
IO1 (SO)    ->   PB0
IO2 (WP)    ->   PA7
IO3 (HOLD)  ->   PA6

The default SystemCoreClock value of the program after the jump should be set 
to be consistent with the SystemCoreClock value of the current program.
================================================================================