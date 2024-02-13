# 15 - A Virtual Machine

This part was fun and I spent a little bit of time looking at Zig's allocators. Instead of making my own stack for the VM I used Zig's `std.ArrayList`, but since in the original code the stack was implemented using a fixed length array I used this as an excuse to try the `FixedBufferAllocator` passing it a buffer of bytes with the max size possible. The biggest different from the original is not making the VM a global variable and used Zig's error types instead of defining an enum for `InterpretResult`.

## Challenges

1. 
```
1 * 2 + 3
```
CONST 1
CONST 2
MULTIPLY
CONST 3 
ADD


```
1 + 2 * 3
```
CONST 1
CONST 2
CONST 3
MULTIPLY
ADD

```
3 - 2 - 1
```
CONST 3
CONST 2
SUB
CONST 1
SUB

```
1 + 2 * 3 - 4 / -5
```
CONST 1
CONST 2
CONST 3
MULTIPLY
ADD
CONST 4
CONST 5
NEGATE
SUB

2.
```
4 - 3 * -2
```

Without OP_NEGATE:
CONST 4
CONST 3
CONST 0
CONST 2
SUB
MULTIPLY
SUB

Without OP_SUBTRACT:
CONST 4
CONST 3
CONST 2
NEGATE
MULTIPLY
NEGATE
ADD

It makes sense to have these common operations execute more efficiently with just one instruction. It probably would also make sense to have instructions for incrementing, decrementing similar to machine code instructions.

3. WIP

4. WIP
