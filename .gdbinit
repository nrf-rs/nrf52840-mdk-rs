# print demangled symbols by default
set print asm-demangle on

# OpenOCD
#set remote hardware-breakpoint-limit 6
#set remote hardware-watchpoint-limit 4
#target extended-remote :3333
target remote :3333
monitor flash breakpoints 1
monitor semihosting enable
monitor semihosting IOClient 3
monitor reset
load
