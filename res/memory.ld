/*
Source: https://sourceware.org/binutils/docs/ld/Scripts.html
Source: https://docs.rs/riscv-rt/latest/riscv_rt/
Source: https://docs.rust-embedded.org/embedonomicon/memory-layout.html
*/

/* Define memory layout (RAM only architecture) */
MEMORY {
    RAM : ORIGIN = 0x80000000, LENGTH =  256K   /* 1K = 1 KiBi = 1024 bytes */
}

/* Set aliases (since this is a RAM only architecture, all regions are in RAM) */
REGION_ALIAS("REGION_INIT", RAM);
REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_HTIF", RAM);

/* Define sections */
SECTIONS {
    _start = ORIGIN(RAM);
    .text.init ALIGN(0x1000): { *(.init) } > REGION_INIT
    .text : { *(.text .text.*) } > REGION_TEXT
    .tohost ALIGN(0x1000): { *(.tohost ) } > REGION_HTIF
    .rodata ALIGN(0x1000): { *(.rodata .rodata.*) } > REGION_RODATA
    .data ALIGN(0x1000): { *(.data) } > REGION_DATA
    .bss ALIGN(0x1000): { *(.bss) } > REGION_DATA
    _end = .;
}

/* Set entrypoint */
ENTRY(_start)

/* Define stack top */
_stack = ORIGIN(RAM) + LENGTH(RAM);
