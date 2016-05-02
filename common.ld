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
    } > rom

    .data : ALIGN(4)
    {
      __DATA_START = .;
      *(.data*)
      . = ALIGN(4);
      __DATA_END = .;
    } > ram AT > rom

    /DISCARD/ :
    {
      *(.ARM.exidx*)
      *(.note.gnu.build-id*)
    }

    __STACK_START = ORIGIN(ram) + LENGTH(ram);
}

/* Sanity checks */
ASSERT ( link_exceptions_start == ORIGIN(rom), "exceptions not where expected" );
ASSERT ( link_interrupts_start - ORIGIN(rom) == 0x40, "interrupts not where expected" );
ASSERT ( link_exceptions_start < link_exceptions_end, "exceptions not linked in" );
ASSERT ( link_interrupts_start < link_interrupts_end, "interrupts not linked in" );
