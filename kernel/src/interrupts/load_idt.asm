[bits 64]

load_idt:
   LIDT [rdi]
   RET

set_interrupts:
   STI
   RET

clear_interrupts:
   CLI
   RET

global load_idt, set_interrupts, clear_interrupts