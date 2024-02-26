# 22 - Local Variables

I actually started reading this part while still going through the global variables challenges, which made me think if it wouldn't be possible to implement the globals in the same way. However, I realize this would make it use a lot more of the stack and ends up being "similar" to what happens with C where global variables aren't stored in the stack. The code in this part was mostly easy, the challenge I faced was trying to find a way to represent unitialized variables differently, but I ended up going with the approach used in the book.

## Challenges

1. I had the idea of using an hash map to get the local variable using its name. For this I wanted to use `std.StringHashMap` instead of `Table` since I don't want to have to allocate new objects. My first idea was to store in the hash map the local index, but that didn't work so I changed it to store a pointer to a `Local`. I also added two fields to the `Local` struct, an optional pointer to another which points to the next local with the same name and also its index. When a local is created, the value in the hash map is set and it's either removed or set to the previous value when the variable's scope ends.

I'm not entirely sure if this complexity is worth it or not. When it comes to execution time, it's probably faster since we aren't iterating over all local variables in the scope and checking their names, but it's also using more memory so I guess it comes down to what we favour.

2. I think I said this before in one of the challenges for the tree walking interpreter, but one of the languages I'm mostly familiar is Rust. In Rust shadowing is allowed anywhere and with that expression it would be defining a new variable `a` with the value of the existing variable with the same name. In contrast, Zig doesn't allow shadowing at all even if it's in different scopes.

3. WIP

4. WIP
