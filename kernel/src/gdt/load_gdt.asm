[bits 64]

load_gdt:   
    lgdt [rdi]    ;[rdi] gets first parameter from the stack, lgdt sets value in gdtr
    mov ax, 0x10  ;set segment registers to 0x10(except cs)
    mov ds, ax    ;
    mov es, ax    ;
    mov fs, ax    ;
    mov gs, ax    ;
    mov ss, ax    ;
    pop rdi       ;pop instruction pointer
    mov rax, 0x08 ;
    push rax      ;push cs value 0x08 to stack
    push rdi      ;push back instruction pointer
    retfq         ;far return pops value to cs then the instruction pointer like ret would

;makes it global so it can be linked to external objects
global load_gdt
