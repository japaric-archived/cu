__DATA_LOAD = LOADADDR(.data);

SECTIONS
{
    .text : ALIGN(4)
    {
        link_exceptions_start = .;
        KEEP(*(.exceptions))
        link_exceptions_end = .;
        link_interrupts_start = .;
        KEEP(*(.interrupts))
        link_interrupts_end = .;
        *(.text*)
    } > FLASH

    .data : ALIGN(4)
    {
      __DATA_START = .;
      *(.data*)
      . = ALIGN(4);
      __DATA_END = .;
    } > SRAM AT > FLASH

    /DISCARD/ :
    {
      *(.ARM.exidx*)
      *(.note.gnu.build-id*)
    }

    __STACK_START = ORIGIN(SRAM) + LENGTH(SRAM);
}

/* Sanity checks */
ASSERT ( link_exceptions_start == ORIGIN(FLASH), "exceptions not where expected" );
ASSERT ( link_interrupts_start - ORIGIN(FLASH) == 0x40, "interrupts not where expected" );
ASSERT ( link_exceptions_start < link_exceptions_end, "exceptions not linked in" );
ASSERT ( link_interrupts_start < link_interrupts_end, "interrupts not linked in" );
