# 6 - Parsing Expressions

Coming back to this after my hiatus, I had done some experimenting on my own with some compiler related topics. One of these topics was parser combinators, also known as functional parsing. Being familiar with this technique for implementing a recursive descent parser helped me better understand this chapter.

Writing the parser was relatively easy, I encountered problems because of how I initially tried to pattern match the literal tokens but ended up fixing it. Instead of exceptions, all methods are returning a `Result<Expr, LoxError>`.

I decided to add "rustyline" to the project, so that I can have a better repl experience that supports the arrow keys, having an history during the repl session and other usual keybindings. 

## Challenges
1. The new grammar would be the following:
comma -> equality ( "," equality )*

