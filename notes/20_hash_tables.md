# 20 - Hash Tables

Initially when I saw there was a part called "Hash Tables" I thought I would skip it as Zig already comes with an implementation in the standard library. Thankfully I decided to read it to ensure I wasn't missing anything. If I hadn't, I would have missed the section on "string interning". So I decided to use this to write a wrapper around Zig's `std.HashMap` using a function that receives `comptime` args for the value type and a config, which allowed my wrapper to stay generic. 

Similar to the book, I added the hash as a field to a new string struct. I made the wrapper work with `*Obj` as the type of the key always and only meant to be used with strings (also added an assertion that should fail if it's used with a different kind of object). To also help with this, I wrote a method for creating a new string which handles the string interning as well as initializes the `String` structure (calculates and stores the hash in a field). The hash is being produced with the same algorithm as in the book.

## Challenges

1. WIP

2. WIP

3. WIP
