# 13 - Inheritance

This one came with some ups and downs but those were all caused by "skill issue". Trying to understand why super wasn't defined until I realized that my class didn't even have a superclass. Then it was crashing when calling `get_at`, which I thought was when trying to get `super` so I went to debug if `super` was being defined and it was. Turns out it was when I called `get_at` to get `this` because I was giving it the wrong distance (forgot to subtract 1). Other than my skill issue while testing things, it was overall not hard to implement this.

EDIT: While doing this part I removed the call to the `bind` method that sets the value of `this` where the `init` method is called when making a new instance of a class. This made the REPL crash if I attempted to set the value of some property.

# Challenges

1. WIP
2. WIP
3. WIP
