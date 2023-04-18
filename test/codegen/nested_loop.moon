align
strbuf res 32

entry
% set up stack
addi r14, r0, topaddr
% "i mentioned this last week-ish in this chat, but i found that the topaddr of 16000 is actually the location that data in r0 is stored
% so at the beginning of the program you just have to decrease r14 by 4" -- mamamia on discord thank you so much
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
% write expr
% store expr int
lw r1, -4(r14)
% write call
addi r14, r14, -12
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
% write end, return stack pointer
addi r14, r14, 12

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
% write expr
% store expr int
lw r1, -8(r14)
% write call
addi r14, r14, -12
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
% write end, return stack pointer
addi r14, r14, 12

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
% write expr
% assign literal 99
addi r4, r0, 99
% store expr int
add r5, r0, r4
% write call
addi r14, r14, -12
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
% write end, return stack pointer
addi r14, r14, 12
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