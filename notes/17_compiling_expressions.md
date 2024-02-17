# 17 - Compiling Expressions

Before reading it, I was a little bit scared of the parsing algorithm, but it turned out to be easier to understand than what I expected. Biggest difference from the code in the book is that I'm trying to avoid global variables and made a `Compiler` struct instead. I've looked a little bit ahead and I'm aware I will have to rewrite this to have a different name since there will be a struct with the same name.

Another thing I know I will eventually have to rewrite is how chunks are passed to the compiler. I made it so the compiler actually initializes the chunk itself but I've realized that there will be other chunks for functions and classes in the future. It won't need a big change so I'm also going to leave that to be done as future work.

I had initially moved the trace execution flag to a new flags file so that I would have both flags on the same file. However, I've found how to have the build system receive these options from arguments (left them as true always for now) and passed them as a module to the code.

## Challenges

1. I really like this challenge. So the expression I want to parse is the following:
```
(-1 + 2) * 3 - -4
```

The trace produced is the following:

- `expression()`
  - `parsePrecedence(PREC_ASSIGNMENT)`
    - `grouping()`
      - `expression()`
        - `parsePrecedence(PREC_ASSIGNMENT)`
          - `unary()`
            - `parsePrecedence(PREC_UNARY)`
              - `number()`
          - `binary()`
            - `parsePrecedence(PREC_FACTOR)`
              - `number()`
    - `binary()`
      - `parsePrecedence(PREC_UNARY)`
        - `number()`
    - `binary()`
      - `parsePrecedence(PREC_FACTOR)`
        - `unary()`
          - `parsePrecedence(PREC_UNARY)`
            - `number()`

2. I believe TOKEN\_LEFT\_PAREN will also have an infix operation when we get to function calls, which also happens in C. Another example in C is the '*' which can be used as a prefix for pointer dereferencing and as infix for multiplication.

3. WIP
