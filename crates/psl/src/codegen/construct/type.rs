use std::fmt::{self, Write};

use crate::ast;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    I32,
    I64,
    Integer,
    Bool,
    Tuple(Vec<Type>),
    Function {
        parameters: Vec<Type>,
        returning: Box<Type>,
    },
    Never,
}

impl TryFrom<ast::Type> for Type {
    type Error = String;

    fn try_from(value: ast::Type) -> Result<Self, Self::Error> {
        match value {
            ast::Type::Simple(token) => match token.content.as_ref() {
                "i32" => Ok(Type::I32),
                "i64" => Ok(Type::I64),
                "bool" => Ok(Type::Bool),
                "tuple0" => Ok(Type::Tuple(Vec::new())),
                _ => Err(format!("THere is no type named {:?}", &token.content)),
            },
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::Integer => write!(f, "{{integer}}"),
            Type::Bool => write!(f, "bool"),
            Type::Tuple(vec) => write!(
                f,
                "({})",
                vec.iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Type::Function {
                parameters,
                returning,
            } => {
                f.write_char('(')?;
                for (idx, param) in parameters.iter().enumerate() {
                    if idx != 0 {
                        f.write_str(", ")?;
                    }
                    param.fmt(f)?;
                }
                f.write_str(") -> ")?;
                returning.fmt(f)?;
                Ok(())
            }
            Type::Never => write!(f, "!"),
        }
    }
}

impl Type {
    pub const UNIT: Type = Type::Tuple(Vec::new());

    pub fn as_c_type(&self) -> String {
        match self {
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::Integer => Type::I32.as_c_type(),
            Type::Bool => "bool".to_string(),
            Type::Tuple(_) => "tuple0".to_string(),
            Type::Function { .. } => todo!("function type is not supported now"),
            Type::Never { .. } => todo!("never type is not supported now"),
        }
    }

    pub fn union_with(self, other: Type) -> Result<Type, String> {
        if self == other {
            return Ok(self);
        }
        match (&self, &other) {
            (Type::I32, Type::Integer) | (Type::Integer, Type::I32) => Ok(Type::I32),
            (Type::I64, Type::Integer) | (Type::Integer, Type::I64) => Ok(Type::I64),
            _ => Err(format!("Type {} is not compatible with {}", self, other)),
        }
    }
}
