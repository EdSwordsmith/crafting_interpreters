use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{self, Hash},
    ops::{Add, Div, Mul, Neg, Sub},
    rc::Rc,
};

mod callable;
mod classes;
mod lists;
mod loxfn;
mod nativefn;
mod primitives;

pub use callable::*;
pub use classes::*;
pub use lists::*;
pub use loxfn::*;
pub use nativefn::*;
pub use primitives::*;

use crate::{
    ast::Stmt,
    errors::RuntimeError,
    interpreter::{Environment, Interpreter},
    scanner::Token,
};

pub trait LoxValue: Display {
    fn primitive(&self) -> Option<LoxPrimitive> {
        None
    }

    fn callable(&self) -> Option<Box<dyn LoxCallable>> {
        None
    }

    fn class(&self) -> Option<LoxClass> {
        None
    }

    fn is_truthy(&self) -> bool {
        true
    }

    fn get_property(&self, _name: &Token) -> LoxProperty {
        LoxProperty::Invalid
    }

    fn set_property(&mut self, _name: &Token, _value: &LoxObj) -> Option<LoxObj> {
        None
    }

    fn bind(&self, _this: LoxObj) -> LoxObj {
        nil()
    }

    fn push(&mut self, _item: LoxObj) {
        unimplemented!()
    }

    fn pop(&mut self) -> Option<LoxObj> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct LoxObj(pub Rc<RefCell<dyn LoxValue>>);

impl LoxObj {
    pub fn is_equal(&self, other: &LoxObj) -> LoxObj {
        boolean(self == other)
    }

    pub fn is_diff(&self, other: &LoxObj) -> LoxObj {
        boolean(self != other)
    }

    pub fn callable(&self) -> Option<Box<dyn LoxCallable>> {
        self.0.borrow().callable()
    }

    pub fn set_property(&mut self, name: &Token, value: &LoxObj) -> Option<LoxObj> {
        self.0.borrow_mut().set_property(name, value)
    }

    pub fn bind(&self, this: LoxObj) -> LoxObj {
        self.0.borrow().bind(this)
    }

    pub fn class(&self) -> Option<LoxClass> {
        self.0.borrow().class()
    }
}

impl Display for LoxObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}

impl Hash for LoxObj {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

impl Debug for LoxObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "obj")
    }
}

// It's fine... :D
#[allow(clippy::vtable_address_comparisons)]
impl PartialEq for LoxObj {
    fn eq(&self, other: &Self) -> bool {
        let p1 = self.0.borrow().primitive();
        let p2 = other.0.borrow().primitive();

        if p1.is_some() && p2.is_some() {
            p1 == p2
        } else {
            self.0.as_ptr() == other.0.as_ptr()
        }
    }
}

impl PartialOrd for LoxObj {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let p1 = self.0.borrow().primitive();
        let p2 = other.0.borrow().primitive();
        p1.and_then(|p1| p2.and_then(|p2| p1.partial_cmp(&p2)))
    }
}

impl Eq for LoxObj {}

impl Add for LoxObj {
    type Output = Option<LoxObj>;

    fn add(self, rhs: Self) -> Self::Output {
        let p1 = self.0.borrow().primitive();
        let p2 = rhs.0.borrow().primitive();
        p1.and_then(|p1| p2.and_then(|p2| p1 + p2))
    }
}

impl Sub for LoxObj {
    type Output = Option<LoxObj>;

    fn sub(self, rhs: Self) -> Self::Output {
        let p1 = self.0.borrow().primitive();
        let p2 = rhs.0.borrow().primitive();
        p1.and_then(|p1| p2.and_then(|p2| p1 - p2))
    }
}

impl Mul for LoxObj {
    type Output = Option<LoxObj>;

    fn mul(self, rhs: Self) -> Self::Output {
        let p1 = self.0.borrow().primitive();
        let p2 = rhs.0.borrow().primitive();
        p1.and_then(|p1| p2.and_then(|p2| p1 * p2))
    }
}

impl Div for LoxObj {
    type Output = Option<LoxObj>;

    fn div(self, rhs: Self) -> Self::Output {
        let p1 = self.0.borrow().primitive();
        let p2 = rhs.0.borrow().primitive();
        p1.and_then(|p1| p2.and_then(|p2| p1 / p2))
    }
}

impl Neg for LoxObj {
    type Output = Option<LoxObj>;

    fn neg(self) -> Self::Output {
        self.0.borrow().primitive().and_then(|p| p.neg())
    }
}

// constructors
pub fn number(n: f64) -> LoxObj {
    LoxObj(Rc::new(RefCell::new(LoxPrimitive::Number(n))))
}

pub fn boolean(b: bool) -> LoxObj {
    LoxObj(Rc::new(RefCell::new(LoxPrimitive::Bool(b))))
}

pub fn string(s: String) -> LoxObj {
    LoxObj(Rc::new(RefCell::new(LoxPrimitive::String(s))))
}

pub fn nil() -> LoxObj {
    LoxObj(Rc::new(RefCell::new(LoxPrimitive::Nil)))
}

pub fn native_fn(
    arity: usize,
    function: fn(&mut Interpreter, &[LoxObj]) -> Result<LoxObj, RuntimeError>,
) -> LoxObj {
    LoxObj(Rc::new(RefCell::new(NativeFn(arity, function, None))))
}

pub fn native_method(
    arity: usize,
    function: fn(&mut Interpreter, &[LoxObj]) -> Result<LoxObj, RuntimeError>,
    this: Option<LoxObj>,
) -> LoxObj {
    LoxObj(Rc::new(RefCell::new(NativeFn(arity, function, this))))
}

pub fn lox_fn(stmt: Box<Stmt>, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> LoxObj {
    LoxObj(Rc::new(RefCell::new(LoxFn(stmt, closure, is_initializer))))
}

pub fn lox_class(
    name: String,
    methods: HashMap<String, LoxObj>,
    superclass: Option<Box<LoxClass>>,
) -> LoxObj {
    LoxObj(Rc::new(RefCell::new(LoxClass {
        name,
        methods,
        superclass,
    })))
}

pub fn lox_list() -> LoxObj {
    LoxObj(Rc::new(RefCell::new(LoxList(Vec::new()))))
}
