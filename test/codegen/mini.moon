entry
% space for variable x
t0 res 4

% space for variable y
t1 res 8

% space for variable z
t2 res 32

% space for literal 1
l1 res 4
addi r14,r0,1
sw l1(r0),r14

% space for literal 2.1
l2_1 db 205,204,204,204,204,204,0,64

% space for literal 3
l3 res 4
addi r14,r0,3
sw l3(r0),r14

% space for literal 2
l2 res 4
addi r14,r0,2
sw l2(r0),r14

% arith expression
lw r1,l1(r0)
lw r2,l2(r0)
t3 res 4
mul r3,r1,r2
sw t3(r0),r3

% arith expression
lw r1,l3(r0)
lw r2,t3(r0)
t4 res 4
add r3,r1,r2
sw t4(r0),r3

% space for literal 10
l10 res 4
addi r14,r0,10
sw l10(r0),r14

% arith expression
lw r1,t0(r0)
lw r2,l1(r0)
t5 res 4
add r3,r1,r2
sw t5(r0),r3
halt