use winnow::Parser;
use winnow::{Located, PResult};

use crate::ast::{TokenKind, Type};

pub fn parse_type(s: &mut Located<&str>) -> PResult<Type> {
    TokenKind::IdentifierIdentifier
        .map(Type::Simple)
        .parse_next(s)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use winnow::{Located, Parser};

    use crate::{
        ast::{Token, TokenKind, Type},
        syntax::fragments::r#type::parse_type,
    };

    #[test]
    fn test_simple_type() {
        assert_eq!(
            parse_type.parse(Located::new("i32")),
            Ok(Type::Simple(Token {
                kind: TokenKind::IdentifierIdentifier,
                span: 0..3,
                content: "i32".to_string()
            })),
        )
    }
}
