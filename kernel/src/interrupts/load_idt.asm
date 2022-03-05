[bits 64]

load_idt:
   LIDT [rdi]
   STI
   RET

global load_idt