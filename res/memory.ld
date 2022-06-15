/*
Source: https://sourceware.org/binutils/docs/ld/Scripts.html
Source: https://docs.rs/riscv-rt/latest/riscv_rt/
Source: https://docs.rust-embedded.org/embedonomicon/memory-layout.html
*/

/* 1K = 1 KiBi = 1024 bytes */
MEMORY {
    RAM : ORIGIN = 0x00000000, LENGTH =  256K
}

ENTRY(init)

SECTIONS {
    . = ORIGIN(RAM);
    .text : { *(.text .text.*) } > REGION_TEXT
    .rodata : { *(.rodata) } > REGION_RODATA
    .data : { *(.data) } > REGION_DATA
    . = ALIGN(4);
}

REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);


/* REGION_ALIAS("REGION_BSS", RAM); */
/* REGION_ALIAS("REGION_HEAP", RAM); */
/* REGION_ALIAS("REGION_STACK", RAM); */

