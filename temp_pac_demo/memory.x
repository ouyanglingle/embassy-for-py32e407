/* PY32E407V1ET7 内存布局（来源：py32e407xE.h / 参考手册 §3.4 / 数据手册 §4 存储器地址）
 * FLASH:   0x08000000, 512 KB (FLASH_END = 0x0807FFFF)
 * SRAM1:   0x20000000,  96 KB
 * SRAM2:   0x20018000,  16 KB （与 SRAM1 连续）
 * CCMSRAM: 0x10000000,  32 KB —— 同一块物理 RAM 的别名窗口，
 *                             还映射到 SRAM2 末端 0x2001C000（0x2001C000..0x20023FFF），
 *                             与 SRAM1/SRAM2 拼成连续的 144 KB 空间。
 *                             CCM SRAM 是总线矩阵上的普通 Slave：CPU(I/D/S bus) 与
 *                             DMA1/DMA2/ETH DMA 均可访问（参考手册 §3.2.4/§3.2.5），
 *                             因此直接把 RAM 扩到 144K 即可当常规 SRAM 用。
 * 注意：CCM SRAM 整块带奇偶校验（选项位 SRAM_PE）。若启用奇偶校验，需先用软件初始化该区，
 *       否则首次读可能触发 NMI；不启用则无此限制（默认 SRAM_PE=0 即“许可/使能”）。
 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 144K
}