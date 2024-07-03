
CC=riscv64-unknown-elf-gcc
CSOURCE=boot.c
ASMSOURCE=crt0.s
CFLAGS=-c -march=rv64imac -mabi=lp64 -mcmodel=medany -g
OUTPUT=crt0.o

from-c:
	${CC} ${CFLAGS} ${CSOURCE} -o ${OUTPUT}

from-asm:
	${CC} ${CFLAGS} ${ASMSOURCE} -o ${OUTPUT}

clean:
	rm crt0.o