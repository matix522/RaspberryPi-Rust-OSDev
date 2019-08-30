# Raspberry PI Battle Royale Operating System
### Videocore: 
 - Resets Arm cores
 --------------------------
 ### Core 1,2,3:
 - Put to sleep
--------------------------
 ###  Core 0:
 - Setup system registers
 - Drop to EL1 
 - Setup GPIO and Miniuart (UART1)
 - Create kernel struct
 - Setup Randomizer
 - Turn On Memory Virtualization (1:1)
 - Enable exception
 - Cause Segmentation Fault
 - Recover from exception
 - Start Random Number Generator Subrutine
 - Prompt for user input
 -------------------------
## Desired Setup Order 
 -------------------------
### Videocore: 

 - Resets Arm cores
 -------------------------
 ###  All Cores:
 - Setup system registers
 - Setup per core stack
 - Drop to EL1 
 - Setup GPIO and MiniUart  (UART1)
 - Turn off CPU cache
 - Enable exception
 - Create level 1 of Memory Menagment Table
 -------------------------
 ### Core 1,2,3:
 - Wait for event
 -------------------------
 ### Core 0:
 - Create SpinLock for MiniUart
 - Create DMA allocator for Videocore Mailbox
 - Create level 2 of Memory Menagment Table
 - Interupt all cores
 -------------------------
 ### All Cores:
 - Turn on Memory Menagment Unit 
 -------------------------
 ### Core 1,2,3: 
 - Wait for event 
 -------------------------
 ### Core 0:
 - Setup Mailbox for Uart (UART0)
 - Flush and remove MiniUart
 - Setup Uart with SpinLock
 - Setup additional devices...
 - Enter krsh mode (Kernel Restricted Shell) 
