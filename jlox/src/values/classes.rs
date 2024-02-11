use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{errors::RuntimeError, interpreter::Interpreter, scanner::Token};

use super::{LoxCallable, LoxObj, LoxValue, Methods};

#[derive(Clone, Debug)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxObj>,
    pub superclass: Option<Box<LoxClass>>,
}

impl LoxClass {
    pub fn find_method(&self, name: &str) -> Vec<LoxObj> {
        let mut methods = self
            .superclass
            .clone()
            .map(|class| class.find_method(name))
            .unwrap_or(vec![]);

        if let Some(method) = self.methods.get(name) {
            methods.push(method.clone())
        }

        methods
    }
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

    fn class(&self) -> Option<LoxClass> {
        Some(self.clone())
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, args: &[LoxObj]) -> Result<LoxObj, RuntimeError> {
        let instance = LoxObj(Rc::new(RefCell::new(LoxInstance {
            class: self.clone(),
            fields: HashMap::new(),
        })));

        let init_methods = self.find_method("init");
        let init_res = if init_methods.is_empty() {
            None
        } else {
            let method = init_methods.bind(instance.clone());
            method
                .callable()
                .map(|method| method.call(interpreter, args))
        };

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
    Method(Vec<LoxObj>),
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}

impl LoxValue for LoxInstance {
    fn get_property(&self, token: &Token) -> LoxProperty {
        let field = self
            .fields
            .get(&token.lexeme)
            .map(|obj| LoxProperty::Field(obj.clone()));

        let methods = self.class.find_method(&token.lexeme);
        let methods = if methods.is_empty() {
            None
        } else {
            Some(LoxProperty::Method(methods))
        };

        field.or(methods).unwrap_or(LoxProperty::Undef)
    }

    fn set_property(&mut self, name: &Token, value: &LoxObj) -> Option<LoxObj> {
        self.fields.insert(name.lexeme.clone(), value.clone());
        Some(value.clone())
    }
}
