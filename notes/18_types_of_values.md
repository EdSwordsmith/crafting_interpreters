# 18 - Types of Values

This part started fairly simple as Zig comes with tagged unions as a feature. Because of this I don't require some of the macros the book's code shows, such as `AS_NUMBER` or `NUMBER_VAL`. The biggest obstacle came from the `runtimeError` function, as Zig doesn't support variadic arguments, so I used `anytype` as the type of the second argument similar to how Zig does in their print functions. However, the obstacle didn't come from this but from a very mysterious compiler error, which I eventually figured out was because I needed to specify one of the arguments as `comptime`. Also, instead of the `BINARY_OP` macro I also had to write a function, made it take a `comptime` arg of a function that takes two `f64` and returns a `Value`.

## Challenges

1. Like we saw in a previous challenge, it's possible to replace subtraction with either (0 - value) or negating the operand before adding.

2. Some common instructions such as comparing with zero (less than 0, equal to 0, greater than 0) could be useful.
