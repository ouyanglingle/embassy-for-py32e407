================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述:
此样例演示了ESMC的memory mapping功能，把预先编译好的bin文件，下载到P25Q128H芯片中，
然后把P25Q128H地址映射到0x00000000，主程序跳转到0x00000000地址开始执行bin文件中的
程序，样例中bin程序执行的任务是闪烁LED（PB5）灯。

Function descriptions:
This example demonstrates the memory mapping function of ESMC. The pre compiled
bin file is downloaded to the P25Q128H chip, and then the P25Q128H address is mapped
to 0x00000000. The main program jumps to the 0x00000000 address to start executing
the program in the bin file. In the example, the bin program executes the task
of flashing the LED (PB5) light.
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
2.程序对FLASH进行Sector擦除，并将bin中内容写入FLASH中.
3.读取FLASH程序，并进行比较，如果正确，则运行外挂FLASH的程序,LED闪烁,否则LED灭。

Example execution steps:
1.Compile and download the program to MCU and run it;
2. The program performs sector erase on FLASH and And write the contents of the
bin into FLASH.
3. Read the FLASH program and compare it. If it is correct, run the external 
FLASH program and the LED will flash. Otherwise, the LED will turn off.
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