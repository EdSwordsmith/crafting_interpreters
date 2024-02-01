# 11 - Resolving and Binding

This part made me afraid that I would not actually be able to complete it due to the challenges of translating everything to Rust. Wrote some code that has questionable quality, like making `Expr`, `Token` and `TokenType` implement `Eq` and `Hash`, which while not technically correct it wasn't exactly relevant as the only moment when locals would be stored are expressions related with variables, so no problems should arise. It's possible that the error handling in the `Resolver` may not be perfect, also due to translating Java exceptions into `Result`. Either way, I'm slightly happy with the result.

## Challenges

1. The body of the function will only execute once the function is called and when that happens the function is already properly defined. So there's no issue in using the function name inside the function as that code will only execute when the function is called after already being defined.

2. I've tested this with Rust and it's allowed. As I've mentioned before, shadowing is allowed always. Maybe it's because I'm very used to this but I agree with this idea as only after that declaration that the name refers to a different variable so it makes sense to allow the expression in the initializer to refer the previous variable.

3. WIP

4. WIP
