# 13 - Inheritance

This one came with some ups and downs but those were all caused by "skill issue". Trying to understand why super wasn't defined until I realized that my class didn't even have a superclass. Then it was crashing when calling `get_at`, which I thought was when trying to get `super` so I went to debug if `super` was being defined and it was. Turns out it was when I called `get_at` to get `this` because I was giving it the wrong distance (forgot to subtract 1). Other than my skill issue while testing things, it was overall not hard to implement this.

EDIT: While doing this part I removed the call to the `bind` method that sets the value of `this` where the `init` method is called when making a new instance of a class. This made the REPL crash if I attempted to set the value of some property.

# Challenges

1. So I'm not feeling courageous as my master thesis is actually close to this challenge. The feature I would like to add is multiple inheritance, as I've been looking into this because of my master thesis. However, I don't feel confident in how this should be implemented as `find_method` would have to be revisited and a method resolution order would have to be put in place. This is how Python does it but I don't think it is intuitive when we are dealing with a lot of classes. 

So instead I tried to make some kind of extension methods, which came out a little bit limited:
- I didn't want to define a new syntax or tokens so I reused the syntax for setting the value of a property
- The only validation is checking if we are setting it to a callable object, which can be a class or a function
- If an instance was created before the extension method then it won't have that method

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/13_extension).

2. WIP
3. WIP
