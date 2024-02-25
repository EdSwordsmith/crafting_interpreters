# 21 - Global Variables

This part was mostly easy. I did have some difficulty with the `canAssign` as I tried to implement it in a different way, which resulted in causing a different compiler error instead of the intended one. Other than that, I decided to test the constant limit and found out it wasn't working properly as chunk's method for adding a constant was returning a `u8` so it never went over the limit. Fixed this to make it convert to `u8` in the `makeConstant` method in the compiler instead.

## Challenges

1. WIP

2. WIP

3. WIP
