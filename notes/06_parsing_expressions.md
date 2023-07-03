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

