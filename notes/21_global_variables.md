# 21 - Global Variables

This part was mostly easy. I did have some difficulty with the `canAssign` as I tried to implement it in a different way, which resulted in causing a different compiler error instead of the intended one. Other than that, I decided to test the constant limit and found out it wasn't working properly as chunk's method for adding a constant was returning a `u8` so it never went over the limit. Fixed this to make it convert to `u8` in the `makeConstant` method in the compiler instead.

## Challenges

1. My first thought was to use an hash table to map identifier names to their indexes in the constants array. The alternative would require iterating through the array to check if the string is already present, which would result in worse performance. However, since we have already allocated a string which calculates its hash and used that for string interning, we won't even have any more aditional perfomance costs if we use an hash table.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/21_reuse_consts).

2. From my previous small experience with compilers from the course, I imagined it would work better if we mapped the globals into indexes and store the values in an array. The array needs to be able to represent that a variable hasn't been initialized yet, so I used `[256]?Value` as its type since the maximum number of global variables is 256. The hash table and array were put in the VM since the compiler is created everytime `interpret` is called and we need to persist the existing globals when running in the REPL.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/21_global_idx).

3. I'm mostly fond of statically typed languages and in this case would favour reporting it as an error. Reporting it as a warning is also a possibility but I've seen a trend of languages which prefer to report everything as compiler errors even what used to be considered only a warning. An example of this is Zig, which I've been using, where, for example, an unused variable or constant inside a function is considered a compiler error and has to be explicitly ignored. Tested this behaviour in Python and JavaScript, where no error or warning was reported.
