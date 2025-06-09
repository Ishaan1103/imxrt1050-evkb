MEMORY
{
  FLASH (rx) : ORIGIN = 0x60000000, LENGTH = 1024K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 512K
}

_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .text : {
    *(.text*)
    *(.rodata*)
    _etext = .;
  } > FLASH

  .data : {
    _sdata = .;
    *(.data*)
    _edata = .;
  } > RAM AT > FLASH

  .bss : {
    _sbss = .;
    *(.bss*)
    *(COMMON)
    _ebss = .;
  } > RAM

  . = ALIGN(4);
  _end = .;
}
