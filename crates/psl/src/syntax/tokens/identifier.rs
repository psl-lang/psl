use unicode_ident::{is_xid_continue, is_xid_start};
use winnow::{
    combinator::alt,
    token::{any, tag, take_until1, take_while},
    Located, PResult, Parser,
};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_identifier_identifier(s: &mut Located<&str>) -> PResult<Token> {
    alt((
        (tag("`"), take_until1("`"), tag("`"))
            .map(|(q1, content, q2)| format!("{q1}{content}{q2}")),
        (
            any.verify(|ch| is_xid_start(*ch)),
            take_while(0.., |ch: char| is_xid_continue(ch)),
        )
            .map(|(start, cont)| format!("{start}{cont}")),
    ))
    .with_span()
    .map(token(TokenKind::IdentifierIdentifier))
    .parse_next(s)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use winnow::{Located, Parser};

    use crate::{
        ast::{Token, TokenKind},
        syntax::tokens::identifier::parse_identifier_identifier,
    };

    #[test]
    fn test_quoted_identifier() {
        assert_eq!(
            parse_identifier_identifier.parse(Located::new("`test example`")),
            Ok(Token {
                kind: TokenKind::IdentifierIdentifier,
                span: 0..14,
                content: "`test example`".to_string()
            }),
        )
    }

    #[test]
    fn test_simple_identifier() {
        assert_eq!(
            parse_identifier_identifier.parse(Located::new("a0")),
            Ok(Token {
                kind: TokenKind::IdentifierIdentifier,
                span: 0..2,
                content: "a0".to_string()
            }),
        )
    }
}
