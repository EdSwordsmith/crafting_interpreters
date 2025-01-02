# 24 - Calls and Functions 

It has been a while since I've last worked on this. I had to take a break so I could focus on my master's thesis, which I successfully finished and defended last November.

This time, it seemed like I found a bigger challenge than those I previously encountered, as it was harder to figure out how to adapt the code in the book while taking into account the different approaches I've taken in implementing the interpreter. One of the differences that had the most impact was my decision not to use global variables, which actually simplified one thing as I didn't need to add a pointer to the "enclosing" compiler. These difficulties happened mostly due to being away from this for a long time and having to remember everything that had been done so far.

I intend to revisit a lot of this code to refactor some stuff. Right now I'm printing everything to stderr, using Zig's `std.debug.print`, so I need to make print statements print to the stdout in the future, and I've also discovered a feature that can improve how tagged unions are used in switch expressions.

It's good to be back!

## Challenges

1. After some searching, I couldn't find an equivalent to C's register keyword for Zig. As such, I can't really perform these benchmarks, and my thoughts are pure speculation. I don't believe the performance gain would be significant enough to justify the extra code complexity.

2. This challenge didn't require many changes. Had to use a struct for representing native functions instead of simply using function pointers, so that I could use a field to store the function's arity. And had to make `callValue` check the arity before calling the function pointer.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/24_arity).

3. WIP

4. WIP
