# 11 - Resolving and Binding

This part made me afraid that I would not actually be able to complete it due to the challenges of translating everything to Rust. Wrote some code that has questionable quality, like making `Expr`, `Token` and `TokenType` implement `Eq` and `Hash`, which while not technically correct it wasn't exactly relevant as the only moment when locals would be stored are expressions related with variables, so no problems should arise. It's possible that the error handling in the `Resolver` may not be perfect, also due to translating Java exceptions into `Result`. Either way, I'm slightly happy with the result.

EDIT: After starting one of the challenges, I ran the examples to see if I had broken anything and realized that I had. So in my locals hash map, it doesn't check if it's the same instance like in Java but it's actually comparing and seeing if they match and it's possible to have multiple expressions of a variable in the same line with different scopes if we have a for loop. To fix this, I added a bytes field to the `Token` struct which made it so that these will never be considered the same token.

## Challenges

1. The body of the function will only execute once the function is called and when that happens the function is already properly defined. So there's no issue in using the function name inside the function as that code will only execute when the function is called after already being defined.

2. I've tested this with Rust and it's allowed. As I've mentioned before, shadowing is allowed always. Maybe it's because I'm very used to this but I agree with this idea as only after that declaration that the name refers to a different variable so it makes sense to allow the expression in the initializer to refer the previous variable.

3. Thought about adding enum that would be used instead of a boolean to check if variables were used or not, setting variables to used when lookup is called. Then I can check if these are unused when a scope ends if there are any unused variables when `end_scope` is called. The errors I'm returning were very lazy and just indicate that there are unused variables.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/11_unused).

4. The idea is to store indexes related to the variables so that we look them up through those indexes instead of through their names. For that I need to have some way to map the variable itself to that index and that was where I had to do something extra. The locals hash map already maps expressions with variables to the depth of the scope it should be in, we can do something similar, but right now that hash isn't efficient to make this whole challenge efficient. Because of that, I changed it so that the hash function for a `Token` will return only that bytes field I added.

After that it was only a little bit tiring, as I had to also change globals to be an hashmap as these aren't going to have a slot number and rewrite everywhere variables are interacted with to make it check if there is an environment or not.

The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/11_var_indexes).
