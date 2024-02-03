use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{errors::RuntimeError, interpreter::Interpreter, scanner::Token};

use super::{LoxCallable, LoxObj, LoxValue};

#[derive(Clone)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxObj>,
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxValue for LoxClass {
    fn callable(&self) -> Option<Box<dyn LoxCallable>> {
        Some(Box::new(self.clone()))
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxObj]) -> Result<LoxObj, RuntimeError> {
        let instance = LoxObj(Rc::new(RefCell::new(LoxInstance {
            class: self.clone(),
            fields: HashMap::new(),
        })));

        let init_res = self
            .methods
            .get("init")
            .map(|method| method.bind(instance.clone()))
            .and_then(|method| method.callable())
            .map(|method| method.call(interpreter, args));

        if let Some(Err(err)) = init_res {
            Err(err)
        } else {
            Ok(instance)
        }
    }

    fn arity(&self) -> usize {
        self.methods
            .get("init")
            .and_then(|method| method.callable())
            .map(|method| method.arity())
            .unwrap_or(0)
    }
}

pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, LoxObj>,
}

pub enum LoxProperty {
    Invalid,
    Undef,
    Field(LoxObj),
    Method(LoxObj),
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}

impl LoxValue for LoxInstance {
    fn get_property(&self, token: &Token) -> LoxProperty {
        self.fields
            .get(&token.lexeme)
            .map(|obj| LoxProperty::Field(obj.clone()))
            .or(self
                .class
                .methods
                .get(&token.lexeme)
                .map(|method| LoxProperty::Method(method.clone())))
            .unwrap_or(LoxProperty::Undef)
    }

    fn set_property(&mut self, name: &Token, value: &LoxObj) -> Option<LoxObj> {
        self.fields.insert(name.lexeme.clone(), value.clone());
        Some(value.clone())
    }
}
