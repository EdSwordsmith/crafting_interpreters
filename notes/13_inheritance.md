# 13 - Inheritance

This one came with some ups and downs but those were all caused by "skill issue". Trying to understand why super wasn't defined until I realized that my class didn't even have a superclass. Then it was crashing when calling `get_at`, which I thought was when trying to get `super` so I went to debug if `super` was being defined and it was. Turns out it was when I called `get_at` to get `this` because I was giving it the wrong distance (forgot to subtract 1). Other than my skill issue while testing things, it was overall not hard to implement this.

# Challenges

1. WIP
2. WIP
3. WIP
