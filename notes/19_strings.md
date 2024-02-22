# 19 - Strings

This part came with the challenge of how to represent different object types, as the approach used in the book's code didn't seem possible to use with Zig. Because of this I went with using a tagged union, similar to how it was done for values. Initially, I had strings just copy the pointer from the slice in the original source code, but I soon realized this wouldn't work as I would need to free object memories and it would be easier to use the same allocator for both the string and the objects themselves, as well as the lifetime of the memory that holds the source code. When using a REPL, the memory of the source code will be free afterwards and this wouldn't work when we need to support instructions and assigning values to variables.

This leads to how I'm managing memory. Until now I was using an arena allocator but this wouldn't be good for objects as they have a different lifetime like I've mentioned. So I've split things into three allocators, the fixed buffer used for the VM's stack, the general purpose allocator for the objects of the language and the arena for everything else. I also realized that the arena wasn't being used the most efficient way possible in the REPL, so I've changed it to both initialize and free it in each REPL iteration. Also realized I needed to ensure the array list for the VM's stack starts with 256 capacity because of how these grow in memory, it would end up running out of memory before reaching the limit of 256.

## Challenges

1. Flexible array members are not possible in Zig and it would also not work well with how I used a tagged union. Because of this I won't attempt this challenge.

2. This seemed easy as I only had to add a flag to tell if a string is owned or not and use that to know if we are able to free the string's memory. This implementation is only good for how the repl is working now as the lifetime of the strings memory will have to be bigger once we have variables. However, it would be perfectly fine for running files as the lifetime of the file's source code is (almost) the entire program.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/19_owned).

3. I'm not a fan of implicit convertions so I would throw a runtime error. Languages such as Python and Julia also handle it this way, and the values of other types have to be explicitly converted into string before being able to concatenate. An alternative is a string interpolation mechanism or some kind of format strings.
