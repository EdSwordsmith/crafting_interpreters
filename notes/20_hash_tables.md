# 20 - Hash Tables

Initially when I saw there was a part called "Hash Tables" I thought I would skip it as Zig already comes with an implementation in the standard library. Thankfully I decided to read it to ensure I wasn't missing anything. If I hadn't, I would have missed the section on "string interning". So I decided to use this to write a wrapper around Zig's `std.HashMap` using a function that receives `comptime` args for the value type and a config, which allowed my wrapper to stay generic. 

Similar to the book, I added the hash as a field to a new string struct. I made the wrapper work with `*Obj` as the type of the key always and only meant to be used with strings (also added an assertion that should fail if it's used with a different kind of object). To also help with this, I wrote a method for creating a new string which handles the string interning as well as initializes the `String` structure (calculates and stores the hash in a field). The hash is being produced with the same algorithm as in the book.

## Challenges

1. All it was needed was change the `hash` and `eql` methods used by the hash map. For numbers, the hash is simply their bit representation and I simply picked random hash values for the other value types (nil, true/false). Since we have Zig's `std.HashMap`, not many changes were actually required to achieve this.

As for supporting class instances as keys, it's not as simple. We could simply use the address in memory of the object as it's hash value, but maybe the user would want to have two class instances with the same values to have the same hash and act as equal, so we could also allow classes to have the `hash` and `eql` methods to allow some level of customization. I believe this is the default behaviour in Java. In contrast, python does not allow mutable data structures to be used as keys and class instances have to implement these methods in order to be used as keys.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/20_value_keys).

2. WIP

3. WIP
