% space for variable x
t0 res 4
% space for variable y
t1 res 4
% space for literal 1
l1 res 4
% space for literal 2
l2 res 4
% space for literal 3
l3 res 4
% space for arith expression
t2 res 4
% space for arith expression
t3 res 4
% space for literal 10
l10 res 4
% space for arith expression
t4 res 4

entry
% assign literal 1
addi r14, r0,1
sw l1(r0), r14

% assignment
lw r1,l1(r0)
sw t0(r0), r1

% assign literal 2
addi r14, r0,2
sw l2(r0), r14

% assignment
lw r1,l2(r0)
sw t1(r0), r1

% assign literal 3
addi r14, r0,3
sw l3(r0), r14

% arith expression
lw r1, t0(r0)
lw r2, t1(r0)
mul r3, r1, r2
sw t2(r0), r3

% arith expression
lw r1, l3(r0)
lw r2, t2(r0)
add r3, r1, r2
sw t3(r0), r3

% assignment
lw r1,t3(r0)
sw t0(r0), r1

% assign literal 10
addi r14, r0,10
sw l10(r0), r14

% arith expression
lw r1, t0(r0)
lw r2, l1(r0)
add r3, r1, r2
sw t4(r0), r3

% assignment
lw r1,t4(r0)
sw t0(r0), r1


hlt