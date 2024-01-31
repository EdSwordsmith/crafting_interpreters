# 9 - Control Flow

This part was really easy to complete as it's doing pretty much the same thing as before, the major difference is that we are checking the truthiness of values. Added the new logical expressions as well as the if and while statements to the AST and for loops as syntactic sugar in the parser. After implementing the for loops, I took some time to test it out and play a little with the language.

## Challenges

1. With only the logical expressions and function calls it's possible to do something like the following:
```
cond and (then_branch() or true) or else_branch();
```

If the condition is true then it will evaluate `(then_branch() or true)` which will call the function that has the code for the then branch and evaluate to `true`.
If the condition is false then it will call the function with the code for the else branch.

The other option, which is what I imagine is the author's intended answer since dynamic dispatch was mentioned, is to define two classes. One of these classes represents true values and the other represents false values. Both of these classes will have the same methods which receive functions for the code of each branch. This is similar to how boolean's work in lambda calculus where a boolean value is actually a function that takes two values, one if true and one if false. 

I don't know which language handles if's in this way, but I would imagine one of the more pure object oriented programming languages like Smalltalk.

2. Looping can be achieved using recursion, but it would require tail call optimization. This is important so that the function is able to re-use the same stack frame when calling itself and not cause a stack overflow from multiple levels of recursion which would happen if we were looping with a lot of iterations.

Looping with a for loop is very easily translated into tail recursion, where we add an accumulator argument to the recursive function which corresponds to the variable we are iterating with in a loop, for example:

```python
for i in range(10):
    print(i)

def recur():
    def inner(i):
        if i == 10:
            return

        print(i)
        return inner(i + 1)
    return inner(0)
```

Functional programming languages like Haskell don't feature loops and recursion is used instead. In languages like Common Lisp or Scheme, I believe that loops exists but are instead a macro which will turn into a recursive call.

3. WIP

<!-- The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/8_unit_vars). -->
