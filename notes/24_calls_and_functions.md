# 24 - Calls and Functions 

It has been a while since I've last worked on this. I had to take a break so I could focus on my master's thesis, which I successfully finished and defended last November.

This time, it seemed like I found a bigger challenge than those I previously encountered, as it was harder to figure out how to adapt the code in the book while taking into account the different approaches I've taken in implementing the interpreter. One of the differences that had the most impact was my decision not to use global variables, which actually simplified one thing as I didn't need to add a pointer to the "enclosing" compiler. These difficulties happened mostly due to being away from this for a long time and having to remember everything that had been done so far.

I intend to revisit a lot of this code to refactor some stuff. Right now I'm printing everything to stderr, using Zig's `std.debug.print`, so I need to make print statements print to the stdout in the future, and I've also discovered a feature that can improve how tagged unions are used in switch expressions.

It's good to be back!

## Challenges

1. WIP

2. WIP

3. WIP

4. WIP
