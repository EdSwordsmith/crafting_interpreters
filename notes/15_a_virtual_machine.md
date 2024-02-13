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

2. WIP

3. WIP

4. WIP
