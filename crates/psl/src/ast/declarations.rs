use super::{Expression, ExpressionOrBlock, Token, Type};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct VariableDeclaration {
    pub ty: Type,
    pub name: Token,
    pub value: Option<Expression>,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Token,
    pub parameters: Vec<(Type, Token)>,
    pub return_type: Type,
    pub body: ExpressionOrBlock,
}
