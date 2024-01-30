# 8 - Statements and State 

For this part I had to define a `Stmt` enum, similar to the `Expr` enum and also a visitor for this new type. In the parser, since a statement can produce more than one parsing error (for instance if we are looking at a block, we want all of the errors in the statements inside and not just the first) I had to make the error in the `Result` be a vector instead of a singular error.

The other major difference from the book is how I implemented `Environment`. In Rust it's not as easy to make a structure work like a linked list, so instead I decided to make this have one field with a vector of hash maps. With this we iterate through the vector instead of traversing the many environments, push and pop elements to the vector to introduce or end a scope, etc.

I also took the liberty to make the `assign` method of `Environment` return a result with the value itself.

## Challenges

1. WIP

2. For this, I had to rewrite how I represent variable declarations in the AST, as I made it have an expression initializer always even when there isn't one, giving it as default the literal `nil` inside the parser. Other than that I had to make it so the `Environment` stores an optional to distinguish between between initialized and not initialized variables that are also declared. With this I could return an additional error inside the `get` method in `Environment`.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/8_unit_vars).

3. I expected it to print out `3`, since we are introducing a variable local to a new scope to the value of the existing one `+ 2`. However, in Python it threw an error stating that it's not possible to initialize a local variable with its own value. I've also tested in Rust, where the value `3` was also printed. 

I'm not sure what users will expect this to do as I can see it may look a bit ambiguous. I'm a little bit more familiar with Rust, which not only allows us to shadow variables by declaring new ones with the same as it also has blocks which work similarly, but I also understand that a user might want to try and declare a variable recursively maybe should not be allowed.

