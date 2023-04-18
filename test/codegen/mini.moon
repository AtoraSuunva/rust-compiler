align
strbuf res 32
l2_1 db 205,204,204,204,204,204,0,64

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
% assign literal 1
addi r4, r0, 1
sw -4(r14), r4

% assignment
% assign literal 2.1
sw r4, l2_1
sw -8(r14), r4

% assignment
% arith expression
% assign literal 3
addi r4, r0, 3
% arith expression
% assign literal 1
addi r5, r0, 1
% assign literal 2
addi r6, r0, 2
mul r7, r5, r6
% end arith expression
add r6, r4, r7
% end arith expression
sw -4(r14), r6

% Call function add(Integer, Integer)
% assign literal 1
addi r6, r0, 1
% arith expression
% assign literal 2
addi r7, r0, 2
% assign literal 3
addi r4, r0, 3
add r5, r7, r4
% end arith expression
% Push parameters
addi r14, r14, -8
% Push parameter 'a'
sw -4(r14), r6
% Push parameter 'b'
sw -8(r14), r5
% Call function
jl r15, f0_add
% Return stack pointer
addi r14, r14, 8
% while
t0 nop
% rel expression
% assign literal 10
addi r5, r0, 10
lw r4, -4(r14)
clt r6, r4, r5
% end rel expression
bz r6, t1
% while block
% write expr
% store expr int
lw r5, -4(r14)
% write call
addi r14, r14, -48
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
addi r14, r14, 48

% assignment
% arith expression
% assign literal 1
addi r5, r0, 1
lw r7, -4(r14)
add r4, r7, r5
% end arith expression
sw -4(r14), r4

j t0
% end while block
t1 nop
% end while
% return
lw r15, 0(r14)
jr r15
% end func main()

% func add(Integer, Integer):
f0_add nop
% push return address
sw 0(r14), r15
% arith expression
lw r4, -4(r14)
lw r5, -8(r14)
add r6, r4, r5
% end arith expression
add r13, r0, r6
% return
lw r15, 0(r14)
jr r15
% end func add(Integer, Integer)