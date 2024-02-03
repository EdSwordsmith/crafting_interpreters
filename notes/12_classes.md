# Classes 

This was the biggest challenge so far and I almost gave up on completing this interpreter. I started implementing this part and everything was fine until the get expressions but I was already suspecting that the set expressions wouldn't work. I had been working with values directly and making copies, so it wouldn't modify any of the fields of the object and making objects be references would require to rewrite a lot.

I remembered how I used `Rc<RefCell<_>>` for the environment and that I could probably use this for Object. However, I would have some problems pattern matching in the interpreter (at least I think I would) so I decided to completely rewrite how the objects were represented. I decided to make a trait `LoxValue`, which everything that can be a value in the language has to implement and made a struct `LoxObj` that only has a `Rc<RefCell<dyn LoxValue>>`, meaning it's a shareable reference to a value and this would be the type used for everything.

Made an enum for the primitives and implemented `LoxCallable` as a trait and the others as structs which implement that and `LoxValue`. I also implemented the rust traits for the algebraic operations as well as the comparison ones. This allowed me to easily rewrite the code in the interpreter and not require pattern matching on the objects themselves. Lastly, to make it easier to instantiate an object I made some functions for creating each type of objects.

## Challenges

1. WIP

2. WIP

3. Allowing to freely access the fields gives a lot of freedom to the programmer. However, it makes the code depend more on internal structure of the class. It also makes it impossible to have the class perform some operations every time some state is changed, which is only possible if we limit access to that state to getters and setters. Choosing between these seem to be a choice between giving more freedom to those who will use the classes or making life a little bit easier for who programs the classes, since they will know how the class is used and won't have to take into account state being changed without using methods.

