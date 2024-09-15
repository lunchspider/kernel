ENTRY(_start)

SECTIONS {
    # must match STAGE_4_DST address in bios/stage-2/src/main.rs
    . = 2M;

    .start : {
        *(.start)
    }
    .text : {
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.data .data.*)
    }
    .bss : {
        *(.bss .bss.*)
    }
}
