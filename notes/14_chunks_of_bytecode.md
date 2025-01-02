# 14 - Chunks of Bytecode

I wanted to keep the same strategy as I had on the first half, where I choose to follow along with a different language which makes me read and understand things better before translating it and implementing what is being done. Since this part is originally in C, I choose to look into Zig which increases the difficulty as I've never programmed in Zig before.

In this part, my struggles were mostly about inexperience in writing Zig as well as knowing I want to implement some things. One of the doubts I had was if I wanted to take advantage of Zig's `std.ArrayList` or make my own for the `Chunk` struct. Format strings are also different from what I'm used to dealing with, so I took quite a bit of time to translate the c format strings to these. Other than that, everything seems to indicate that I'm going to have fun with Zig, as well as with the rest of the book.

## Challenges

1. For this challenge I played a little before settling on this implementation. I had initially thought about storing the start offset for that line and use binary search, which is what I ended up implementing, but I decided to try something else first. I looked into the [wikipedia article of run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding) and wrote in a way where for each line I also stored how many instructions it has. However, the search wouldn't be as efficient as binary search with the other approach so I stepped back and implemented that.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/14_lines).

2. Again, this seemed easy to implement and the struggles I encountered were more about not being that used to writing Zig. In the disassembler I wasn't getting how I can shift left to combine the bytes to form the index of the constant, so I ended up just multiplying by 256. 

On the downsides of having two instructions. One is that we are using one OpCode, meaning there's one less instruction type which we can define. It may also increase the complexity of our VM, and make it slower due to how the cache works since we now have instructions that use a few more bytes than usual, but I think the most important point is not being able to have as many opcodes as we would be if we didn't have this, which most likely isn't even a big issue.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/14_constant_long).

3. I had left this challenge unanswered for quite a while. My understanding of how the heap works is more high level, so I wasn't sure of how I should answer this. Even after looking at [musl's source code](https://git.musl-libc.org/cgit/musl/tree/src/malloc/oldmalloc/malloc.c), my understanding hasn't improved a lot. I need to look at this better in the future. 

What I could understand is that blocks of heap-allocated memory keep a pointer to the next and previous block of memory and can be marked as "in use" or "free". I also found a function which partly shows how blocks can be reused, by splitting a block into two when less memory is required.

As for the "Hardcore mode" part of the challenge, I did not write a reallocate function in my implementation since I'm using Zig's `std.ArrayList`, but I do know how arena/buffer allocators work. I can use malloc at the start to allocate a big enough buffer which will serve as the heap. I need to keep a pointer to the next free chunk and every time a new allocation is done I would increase this pointer. This would make freeing only work when trying to free the last allocated chunk.
