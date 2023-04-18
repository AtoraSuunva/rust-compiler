align
strbuf res 32
regbuf res 16

entry
% set up stack
addi r14, r0, topaddr
% required to avoid overwriting r0
subi r14, r14, 4
jl r15, main
hlt

% func main():
main nop
% push return address
sw 0(r14), r15
% assignment
% assign literal 0
addi r1, r0, 0
sw -4(r14), r1

% assignment
% assign literal 10
addi r1, r0, 10
sw -8(r14), r1

% while
t4 nop
% rel expression
% assign literal 10
addi r1, r0, 10
lw r3, -4(r14)
clt r2, r3, r1
% end rel expression
bz r2, t5
% while block
% Write()
% Save registers
addi r12, r0, regbuf
sw 0(r12), r1
sw -4(r12), r2
sw -8(r12), r3
sw -12(r12), r4
% store expr int
lw r1, -4(r14)
% write call
addi r14, r14, -16
sw -8(r14), r1
addi r1, r0, strbuf
sw -12(r14), r1
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% Restore registers
addi r12, r0, regbuf
lw r1, 0(r12)
lw r2, -4(r12)
lw r3, -8(r12)
lw r4, -12(r12)
% write end, return stack pointer
addi r14, r14, 16


% while
t2 nop
% rel expression
% assign literal 20
addi r1, r0, 20
lw r4, -8(r14)
clt r3, r4, r1
% end rel expression
bz r3, t3
% while block
% Write()
% Save registers
addi r12, r0, regbuf
sw 0(r12), r1
sw -4(r12), r2
sw -8(r12), r3
sw -12(r12), r4
% store expr int
lw r1, -8(r14)
% write call
addi r14, r14, -16
sw -8(r14), r1
addi r1, r0, strbuf
sw -12(r14), r1
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% Restore registers
addi r12, r0, regbuf
lw r1, 0(r12)
lw r2, -4(r12)
lw r3, -8(r12)
lw r4, -12(r12)
% write end, return stack pointer
addi r14, r14, 16


% assignment
% arith expression
% assign literal 1
addi r1, r0, 1
lw r5, -8(r14)
add r4, r5, r1
% end arith expression
sw -8(r14), r4


% if
% rel expression
% assign literal 20
addi r4, r0, 20
lw r5, -8(r14)
ceq r1, r5, r4
% end rel expression
bz r1, t0
% if block
% Write()
% Save registers
addi r12, r0, regbuf
sw 0(r12), r1
sw -4(r12), r2
sw -8(r12), r3
sw -12(r12), r4
% assign literal 99
addi r4, r0, 99
% store expr int
add r5, r0, r4
% write call
addi r14, r14, -16
sw -8(r14), r5
addi r5, r0, strbuf
sw -12(r14), r5
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% Restore registers
addi r12, r0, regbuf
lw r1, 0(r12)
lw r2, -4(r12)
lw r3, -8(r12)
lw r4, -12(r12)
% write end, return stack pointer
addi r14, r14, 16

j t1
% else block
t0 nop
t1 nop
j t2
% end while block
t3 nop
% end while

% assignment
% arith expression
% assign literal 1
addi r3, r0, 1
lw r4, -4(r14)
add r5, r4, r3
% end arith expression
sw -4(r14), r5

j t4
% end while block
t5 nop
% end while
% return
lw r15, 0(r14)
jr r15
% end func main()