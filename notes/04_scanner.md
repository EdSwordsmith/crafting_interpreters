# 4 - Scanner

This one came with the challenge of adapting the code to rust, static variables and methods in a class aren't a possibility so I made a struct `Lox` that is only created once and passed it to the `Scanner` struct to be able to call the `error` method.

Another possibility, very likely a better one, is to make an `ErrorReporter` struct and pass it around to what may need to use it. I can still change my mind and switch to this later.

The next adaptation I made was the token representation. I kept it similar with a `Token` struct and a `TokenType` enum, but for the value of literals I added properties to the `TokenType::String` and `TokenType::Number`, a `String` and a `f64` respectively.

## Challenges
1. I had to search a bit for this one. Both Python and Haskell don't use brackets or some kind of `do/begin` and `end` to indicate a block, but instead use identation levels. As a result, some tokens depend on the identation level, making their grammars not regular.

2. Both Ruby and CoffeeScript allow function calls without parentheses, spaces help say what is the sequence of function calls.

3. Some languages could use comments with preprocessor directives and maybe when having a language transpiling to another language, having the comments will help making the result human readable.
