__DATA_LOAD = LOADADDR(.data);

SECTIONS
{
  .vector_table :
  {
    link_stack_pointer_start = .;
    LONG(ORIGIN(SRAM) + LENGTH(SRAM))
    link_stack_pointer_end = .;
    link_reset_start = .;
    KEEP(*(.reset))
    link_reset_end = .;
    link_exceptions_start = .;
    KEEP(*(.exceptions))
    link_exceptions_end = .;
    link_interrupts_start = .;
    KEEP(*(.interrupts))
    link_interrupts_end = .;
  } > FLASH

  .text :
  {
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
}

/* Sanity checks */
ASSERT(link_stack_pointer_start == ORIGIN(FLASH), "initial stack pointer value not where expected");
ASSERT(link_reset_start - ORIGIN(FLASH) == 4, "reset not where expected");
ASSERT(link_exceptions_start - ORIGIN(FLASH) == 8, "exceptions not where expected");
ASSERT(link_interrupts_start - ORIGIN(FLASH) == 0x40, "interrupts not where expected");
ASSERT(link_stack_pointer_start < link_stack_pointer_end, "initial stack pointer value not linked in");
ASSERT(link_reset_start < link_reset_end, "reset not linked in");
ASSERT(link_exceptions_start < link_exceptions_end, "exceptions not linked in");
ASSERT( link_interrupts_start < link_interrupts_end, "interrupts not linked in");
