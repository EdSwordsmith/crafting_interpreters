use std::{
    cmp::Ordering,
    fmt::Display,
    hash,
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::{number, string, LoxObj, LoxValue};

#[derive(Debug, Clone, PartialEq)]
pub enum LoxPrimitive {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
}

// This is an hack that I'm not proud of
impl Eq for LoxPrimitive {}

impl hash::Hash for LoxPrimitive {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Display for LoxPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxPrimitive::Number(n) => write!(f, "{n}"),
            LoxPrimitive::Bool(b) => write!(f, "{b}"),
            LoxPrimitive::String(s) => write!(f, "{s}"),
            LoxPrimitive::Nil => write!(f, "nil"),
        }
    }
}

impl LoxValue for LoxPrimitive {
    fn primitive(&self) -> Option<LoxPrimitive> {
        Some(self.clone())
    }

    fn is_truthy(&self) -> bool {
        match self {
            LoxPrimitive::Bool(value) => *value,
            LoxPrimitive::Nil => false,
            _ => true,
        }
    }
}

impl Add for LoxPrimitive {
    type Output = Option<LoxObj>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxPrimitive::Number(n1), LoxPrimitive::Number(n2)) => Some(number(n1 + n2)),
            (LoxPrimitive::String(s1), LoxPrimitive::String(s2)) => Some(string(s1 + s2.as_str())),
            _ => None,
        }
    }
}

impl Sub for LoxPrimitive {
    type Output = Option<LoxObj>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxPrimitive::Number(n1), LoxPrimitive::Number(n2)) => Some(number(n1 - n2)),
            _ => None,
        }
    }
}

impl Mul for LoxPrimitive {
    type Output = Option<LoxObj>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxPrimitive::Number(n1), LoxPrimitive::Number(n2)) => Some(number(n1 * n2)),
            _ => None,
        }
    }
}

impl Div for LoxPrimitive {
    type Output = Option<LoxObj>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxPrimitive::Number(n1), LoxPrimitive::Number(n2)) => Some(number(n1 / n2)),
            _ => None,
        }
    }
}

impl Neg for LoxPrimitive {
    type Output = Option<LoxObj>;

    fn neg(self) -> Self::Output {
        match self {
            LoxPrimitive::Number(n) => Some(number(-n)),
            _ => None,
        }
    }
}

impl PartialOrd for LoxPrimitive {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LoxPrimitive::Number(n1), LoxPrimitive::Number(n2)) => n1.partial_cmp(n2),
            _ => None,
        }
    }
}
