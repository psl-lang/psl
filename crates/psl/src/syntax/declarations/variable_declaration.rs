use winnow::{
    combinator::{cut_err, opt},
    error::{StrContext, StrContextValue},
    Located, PResult, Parser,
};

use crate::{
    ast::{TokenKind, VariableDeclaration},
    syntax::{
        expressions::parse_expression,
        fragments::{r#type::parse_type, separator::parse_separator},
    },
};

pub fn parse_variable_declaration(s: &mut Located<&str>) -> PResult<VariableDeclaration> {
    (
        parse_type,
        TokenKind::WhitespaceHorizontal,
        TokenKind::IdentifierIdentifier,
        opt((
            opt(TokenKind::WhitespaceHorizontal),
            TokenKind::PunctuationEqualsSign,
            opt(TokenKind::WhitespaceHorizontal),
            cut_err(parse_expression).context(StrContext::Expected(StrContextValue::Description(
                "expression",
            ))),
        ))
        .map(|opt| opt.map(|(_, _, _, expr)| expr)),
        cut_err(parse_separator),
    )
        .map(|(ty, _, name, value, _)| VariableDeclaration { ty, name, value })
        .parse_next(s)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use winnow::{Located, Parser};

    use crate::{
        ast::{Token, TokenKind, Type, VariableDeclaration},
        syntax::declarations::variable_declaration::parse_variable_declaration,
    };

    #[test]
    fn test_uninitialized() {
        assert_eq!(
            parse_variable_declaration.parse(Located::new("i32 a")),
            Ok(VariableDeclaration {
                ty: Type::Simple(Token {
                    kind: TokenKind::IdentifierIdentifier,
                    content: "i32".to_string(),
                    span: 0..3
                }),
                name: Token {
                    kind: TokenKind::IdentifierIdentifier,
                    content: "a".to_string(),
                    span: 4..5
                },
                value: None,
            }),
        )
    }
}
