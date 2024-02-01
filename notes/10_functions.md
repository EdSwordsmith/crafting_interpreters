# 10 - Functions 

Doing this project in Rust comes with the challenge of having to adapt some patterns and needing to investigate what works as a replacement for the original code. In a previous part, I implemented `Environment` as having a vector of scopes. This did not take into account what would be needed once we had function calls and closures. Because of this I had to find a way to have a structure similar to a Linked List and keep it similar to how it was implemented in the book. To do this I used `Rc<RefCell<Environment>>` so that every environment is reference counted and can be mutated in different places.

Refactoring `Environment` wasn't the only challenge, I also had to find a way to represent functions. This is simple in the book as it uses the `Object` class for everything, but I'm using an enum that has every possible data type in Lox as a variant and I need this enum to implement the `Clone` (to be able to "copy" values around with ease) and `PartialEq` (to use with the equality and inequality operators). Initially I attempted to create a trait `LoxCallable` similar to the interface created in the book and use trait objects but I found it wasn't trivial to use. I ended up defining another enum `Callable` with two variants `NativeFn` and `LoxFn`. Added a string with the name of the native function so that it could be compared when implementing `PartialEq`.

To have a little fun, I also wrote a macro for defining native functions. Turning this:
```rust
environment.borrow_mut().define(
    "clock".into(),
    Object::Callable(NativeFn("clock".into(), 0, |_, _| {
        Ok(Object::Number(
            Utc::now().timestamp_millis() as f64 / 1000.0,
        ))
    })),
);
```

Into this:
```rust
native_fn!(environment, "clock", 0, |_, _| {
    Ok(Object::Number(
        Utc::now().timestamp_millis() as f64 / 1000.0,
    ))
});
```

## Challenges

1. Had to go look a little bit at [Smalltalk](https://learnxinyminutes.com/docs/smalltalk/). I knew smalltalk saw methods completely as messages being passed to an object, but I wasn't fully aware of how the syntax was. From what I could gather, when passing arguments it needs to have a label before and these labels together are actually what the message or method name is. This means that if there are more arguments then it's a completely different method so there's no need for validating the number of arguments being passed.

2. WIP

<!-- The code for this challenge can be found [here](https://github.com/EdSwordsmith/crafting_interpreters/tree/9_break). -->

3. Function arguments and its body are on the same scope. Right now, my current implementation allows this, as it allows shadowing or redefinition of variables on the same scope. However, I've checked and the author considers this a bug and this will most likely be dealt with afterwards.
I'm not opposed to shadowing, but again that is most likely because of the languages I'm mostly used to, for example rust where I can shadow variables and define a new one with the same name even if it has a different type.
