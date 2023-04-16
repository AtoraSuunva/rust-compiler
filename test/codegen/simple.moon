align
strbuf res 32

entry
% set up stack
addi r14, r0, topaddr
% "i mentioned this last week-ish in this chat, but i found that the topaddr of 16000 is actually the location that data in r0 is stored
% so at the beginning of the program you just have to decrease r14 by 4" -- mamamia on discord thank you so much
subi r14, r14, 4

% main()
% assignment
% assign literal 1
addi r1, r0, 1
sw 0(r14), r1

% assignment
% assign literal 2
addi r1, r0, 2
sw -4(r14), r1

% assignment
% arith expression
% assign literal 3
addi r1, r0, 3
% arith expression
lw r3, 0(r14)
lw r4, -4(r14)
mul r2, r3, r4
% end arith expression
add r4, r1, r2
% end arith expression
sw 0(r14), r4

% read expr
addi r14, r14, -8
addi r4, r0, strbuf
sw -8(r14), r4
jl r15, getstr
jl r15, strint
addi r14, r14, 8
sw -4(r14), r13
% write expr
% store expr int
lw r4, -4(r14)
% write call
addi r14, r14, -8
sw -8(r14), r4
addi r4, r0, strbuf
sw -12(r14), r4
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% write end, return stack pointer
addi r14, r14, 8
% write expr
% assign literal 11111
addi r4, r0, 11111
% store expr int
add r2, r0, r4
% write call
addi r14, r14, -8
sw -8(r14), r2
addi r2, r0, strbuf
sw -12(r14), r2
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% write end, return stack pointer
addi r14, r14, 8
% while
t0 nop
% rel expression
% assign literal 10
addi r2, r0, 10
lw r1, 0(r14)
clt r4, r1, r2
% end rel expression
bz r4, t1
% while block
% write expr
% store expr int
lw r2, 0(r14)
% write call
addi r14, r14, -8
sw -8(r14), r2
addi r2, r0, strbuf
sw -12(r14), r2
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% write end, return stack pointer
addi r14, r14, 8

% assignment
% arith expression
% assign literal 1
addi r2, r0, 1
lw r3, 0(r14)
add r1, r3, r2
% end arith expression
sw 0(r14), r1

j t0
% end while block
t1 nop
% end while
% if
% rel expression
lw r2, -4(r14)
lw r3, 0(r14)
clt r1, r2, r3
% end rel expression
bz r1, t2
% if block
% write expr
% assign literal 22222
addi r3, r0, 22222
% store expr int
add r2, r0, r3
% write call
addi r14, r14, -8
sw -8(r14), r2
addi r2, r0, strbuf
sw -12(r14), r2
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% write end, return stack pointer
addi r14, r14, 8

% write expr
% store expr int
lw r2, -4(r14)
% write call
addi r14, r14, -8
sw -8(r14), r2
addi r2, r0, strbuf
sw -12(r14), r2
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% write end, return stack pointer
addi r14, r14, 8
j t3
% else block
t2 nop
% write expr
% assign literal 23333
addi r2, r0, 23333
% store expr int
add r3, r0, r2
% write call
addi r14, r14, -8
sw -8(r14), r3
addi r3, r0, strbuf
sw -12(r14), r3
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% write end, return stack pointer
addi r14, r14, 8

% write expr
% store expr int
lw r3, 0(r14)
% write call
addi r14, r14, -8
sw -8(r14), r3
addi r3, r0, strbuf
sw -12(r14), r3
jl r15, intstr
sw -8(r14), r13
jl r15, putstr
% write newline
addi r13, r0, 13
putc r13
addi r13, r0, 10
putc r13
% write end, return stack pointer
addi r14, r14, 8
t3 nop
% end main()

hlt