# 16 - Scanning on Demand

I'm starting to feel a little bit more confortable around Zig. Got a little ahead of myself when writing the function for the repl. The easiest way would be to do something similar to what is shown in the book and use a method fo read directly into a buffer. However, I wanted to see if I could get it to work without a fixed size buffer, so I used an ArrayList which comes with a writer and I can use a method that takes that without having to pass a maximum length. The runFile was almost identical to the book's, the main difference is I'm using Zig's error union types to return and handle errors.

The Scanner is also almost identical, except I decided to use indexes instead of pointers and since strings in Zig are slices, these already contain the length so that wasn't needed in the Token struct.

While talking to a friend, I realized I left a mistake before. The buffer I'm using for the allocator where the stack will be wasn't big enough. This buffer had 256 bytes when it needed to have 256 values. To fix this I changed it to have the size be 256 times the size of Value, if the Value type is changed then this will be updated accordingly.

## Challenges

1. I would probably need to define a token type that indicates the start of an interpolation. To know if we need to use this, we would check for the characters that start an interpolation when advancing through the chars of a string. Then we can now if it's ok to simply use a normal string token or one that indicates that we are doing string interpolation. As we only care for the expressions inside the interpolation, the characters that indicate it should probably be discarded.

```
"Nested ${"interpolation?! Are you ${"mad?!"}"}"
```

This example would be something like this:

```
TOKEN_STRING_INTERPOLATION "Nested "
TOKEN_STRING_INTERPOLATION "interpolation?! Are you "
TOKEN_STRING "mad?!"
TOKEN_STRING ""
```

Where the usual string token would indicate the end of the string interpolation and hold the remaining text.

2. The easiest way to fix this would be for "<<" and ">>" not being tokens but instead two separate "<" or ">" tokens. I assume these languages do something similar to this.

3. Can't remember other contextual keywords, I only knew the example of async/await in C#. I honestly like them for giving some more freedom in naming variables. For example, I've already written the interpreter in Rust and now doing it in Zig, I came across a reserved keyword "type", which made me have to use a different name for the field with the token's type. To implement these, I would remove the tokens for the keywords and simply use the identifier token as only the parser will know the context and be able to correctly identify if it's an identifier or something else.
