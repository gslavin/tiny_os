ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* Ensure that the multiboot header is at the beginning
         * Use KEEP so the header isn't garbage collected
         */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }
}

