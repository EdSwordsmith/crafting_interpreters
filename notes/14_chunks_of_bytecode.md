# 14 - Chunks of Bytecode

I wanted to keep the same strategy as I had on the first half, where I choose to follow along with a different language which makes me read and understand things better before translating it and implementing what is being done. Since this part is originally in C, I choose to look into Zig which increases the difficulty as I've never programmed in Zig before.

In this part, my struggles were mostly about inexperience in writing Zig as well as knowing I want to implement some things. One of the doubts I had was if I wanted to take advantage of Zig's `std.ArrayList` or make my own for the `Chunk` struct. Format strings are also different from what I'm used to dealing with, so I took quite a bit of time to translate the c format strings to these. Other than that, everything seems to indicate that I'm going to have fun with Zig, as well as with the rest of the book.

## Challenges

1. For this challenge I played a little before settling on this implementation. I had initially thought about storing the start offset for that line and use binary search, which is what I ended up implementing, but I decided to try something else first. I looked into the [wikipedia article of run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding) and wrote in a way where for each line I also stored how many instructions it has. However, the search wouldn't be as efficient as binary search with the other approach so I stepped back and implemented that.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/14_lines).

2. WIP

3. WIP
