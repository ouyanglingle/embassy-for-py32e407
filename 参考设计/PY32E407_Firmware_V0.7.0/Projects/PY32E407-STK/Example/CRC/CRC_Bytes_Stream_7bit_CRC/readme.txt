================================================================================
                                样例使用说明
                             Sample Description
================================================================================
功能描述：
此样例演示了，用户定义的生成多项式由 HAL_CRC_Init() 配置。同时，设置输入或输出数
据均不得反转，使用默认初始值，并指定输入数据类型为字节。

Function descriptions:
In this example, the user-defined generating polynomial is configured by 
HAL_CRC_Init(). At the same time, it is set that neither input or output data must
be reversed, the default init value is used and it is specified that input data 
type is byte.

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
2. 处理一个5字节长的缓冲区，生成第一个CRC。
3. 从17字节长的缓冲区计算第二个CRC。对于后者，不重新初始化CRC计算器，而是使用先
前计算的CRC作为初始值。
4. 从1字节长的缓冲区计算第三个CRC。同样，CRC计算器不重新初始化，而是使用先前计
算的CRC作为初始值。
5. 从2字节长的缓冲区计算第四个CRC。这一次，CRC计算器使用IP默认值0x7F重新初始化，
即7位CRC。
6. 计算出的CRC代码都存储在uwCRCValue变量中。计算完成后，CRC值(uwCRCValue)将与CRC
预期值 (uwExpectedCRCValue_1, 2, 3 或 4) 进行比较。
7. 观察LED灯是否亮起，如果灯亮表示运行成功，否则运行失败。

Example execution steps:
1. Compile and download the program;
2. process a 5-byte buffer to generate the first CRC.
3. compute the second CRC from the 17-byte buffer; in the latter case, the CRC
calculator is not reinitialized, but the previously computed CRC is used as the
initial value.
4. compute the third CRC from the 1-byte buffer; again, the CRC calculator is 
not reinitialized, but uses the previously computed CRC as the initial value.
5. the fourth CRC is calculated from the 2-byte buffer, this time the CRC 
calculator is reinitialized with the IP default value of 0x7F, which is a 7-bit
CRC.
6. The calculated CRC codes are stored in the uwCRCValue variable. When the 
calculation is complete, the CRC value (uwCRCValue) is compared to the CRC 
expected value (uwExpectedCRCValue_1, 2, 3 or 4).
7. Observe whether the LED light is on. If it is on, it indicates successful
operation, otherwise it fails
================================================================================
注意事项：

Notes:

================================================================================