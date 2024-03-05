# 23 - Jumping Back and Forth

I really enjoyed this part and I found no obstacles that were hard to overcome. The bytecode generated is mostly similar to what I remember doing in my Compilers course, where we actually compiled to a Stack Machine IR so it's similar to this bytecode and the biggest difference is that the IR used Labels for jumps and, since the compiler wasn't single pass, the increment would actually be compiled after the loop's statement which would allow us to have one jump less. Other than that, I don't have much more to say.

## Challenges

1. This challenge ended up being more difficult than I had initially anticipated. For comparing the value in each case I needed a way to duplicate the last value on the stack. For this I initially went with reusing in a weird way `OP_GET_LOCAL` but I ended up adding a new instruction `OP_DUP` just for this. I also initially thought of using a while loop with a different condition (while match case) but it ended up getting stuck and I remembered the condition we use for parsing blocks. After that, I also almost didn't remember that I needed to make the jump if false go to the switch's end in case there's no `default:`. After all these things, it was a fun challenge.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/23_switch).

2. Again, most of this was similar to what I did on my Compilers course, with the exception that I didn't have to "pop" the values stored on the stack as I had to do here. Initially, I used `endScope` to emit `OP_POP` for each local variable in the scope when a `continue` is found and `beginScope` to re-create the scope, as variables will be redeclared afterwards. However, I realized that the continue might be more than 1 scope away from ouside the loop, so similar to how I'm keeping track of the offset for the current loop, I keep track of the depth when a loop starts so that I emit the right amount of `OP_POP`.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/23_continue).

3. I had some trouble trying to find useful ideas, I'm just going to describe one and a possible syntax. I believe a common pattern using loops is to iterate over something and use an if statement inside to essencially only execute the body of the loop when that condition is met. So my idea would be to add an optional condition to the while loop syntax which, if present, will essentially make the while loop's statement only run when the condition is met.

```
var a = 10;
while (a > 0; a % 2 == 0) {
    // Do something
}
```

Other than this, I talked with some friends and some joke ideas came up, such as a `maybe` statement which acts like an `if` however it randomly chooses between both branches.
