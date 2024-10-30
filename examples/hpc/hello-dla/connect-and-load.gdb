# HPC is exposed at to 3335
target extended-remote :3335
load
backtrace
# Break on _start, then continue until start
# This fixes a problem where the device just freezes when executing from SDRAM
break _start
c
