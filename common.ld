ENTRY(__reset)

__DATA_LOAD = LOADADDR(.data);

SECTIONS
{
    .text : ALIGN(4)
    {
        KEEP(*(.exceptions))
        KEEP(*(.interrupts))
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
    }

    __STACK_START = ORIGIN(ram) + LENGTH(ram);
}
