# 6 - Parsing Expressions

Coming back to this after my hiatus, I had done some experimenting on my own with some compiler related topics. One of these topics was parser combinators, also known as functional parsing. Being familiar with this technique for implementing a recursive descent parser helped me better understand this chapter.

Writing the parser was relatively easy, I encountered problems because of how I initially tried to pattern match the literal tokens but ended up fixing it. Instead of exceptions, all methods are returning a `Result<Expr, LoxError>`.

I decided to add "rustyline" to the project, so that I can have a better repl experience that supports the arrow keys, having an history during the repl session and other usual keybindings. 

## Challenges
1. The new grammar would be the following:
expression -> comma
comma -> equality ( "," equality )*

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/6_comma_operator).

2. The idea I had for the new grammar would be the following:
expression -> ternary
ternary -> equality ( "?" equality ":" equality )*

I wasn't sure if it would make sense to allow ternary expressions without being surrounded by parenthesis. I don't think this operator being right or left associative would make any difference. All of the following are equivelant: 

```
false ? 5 : false ? 4 : 2 // == 2
(false ? 5 : false) ? 4 : 2 // == false ? 4 : 2 == 2
false ? 5 : (false ? 4 : 2) // == false ? 5 : 2 == 2
```

With that grammar, the operator would be left associative. However, I looked a bit into the solutions from the book because I wanted to check if I was thinking properly. The grammar shown in the book solutions uses a recursive rule and allows for any expression between the "?" and the ":" and is the following:
expression -> conditional
conditional -> equality ( "?" expression ":" conditional )?

With this rule instead, the operator is right associative.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/6_ternary_operator).

3. I needed a little help getting this one right, I was wondering where should the error productions should be placed. I probably should've understood that the only place it makes sense is with the highest precedence, since these errors will be caught here but get a generic "Expect expression." error.
So the idea would be to have the following as the grammar:

expression → equality ;
equality   → comparison ( ( "!=" | "==" ) comparison )* ;
comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term       → factor ( ( "-" | "+" ) factor )* ;
factor     → unary ( ( "/" | "*" ) unary )* ;
unary      → ( "!" | "-" ) unary;
primary    → NUMBER | STRING | "true" | "false" | "nil"
           | "(" expression ")"
           // Rules for the error productions
           | ( "!=" | "==" ) equality
           | ( ">" | ">=" | "<" | "<=" ) comparison
           | ( "+" ) term
           | ( "/" | "*" ) factor ;
           
We don't have an error production starting with "-", because there's an unary expression with this operator. Like before I went to check if my grammar rules were correct looking at the solution, but I found that in the solution's there's a decrement and increment expressions which I couldn't find in the book so far. Anyway I decided to keep what I have and just implement the error productions.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/6_error_productions).
