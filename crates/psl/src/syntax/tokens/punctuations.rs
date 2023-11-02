use winnow::{combinator::alt, token::tag, Located, PResult, Parser};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_punctuations(s: &mut Located<&str>) -> PResult<Token> {
    alt((
        punct("*", TokenKind::PunctuationAsterisk),
        punct("+", TokenKind::PunctuationPlusSign),
        punct("-", TokenKind::PunctuationHyphenMinus),
        punct("/", TokenKind::PunctuationSolidus),
        punct("%", TokenKind::PunctuationPercent),
        punct("&", TokenKind::PunctuationAmpersand),
        punct("|", TokenKind::PunctuationPipe),
        punct("=", TokenKind::PunctuationEqualsSign),
        punct("!", TokenKind::PunctuationExclamationMark),
        punct("<", TokenKind::PunctuationLessSign),
        punct(">", TokenKind::PunctuationGreaterSign),
    ))
    .parse_next(s)
}

fn punct(
    punct: &str,
    kind: TokenKind,
) -> impl for<'a> Fn(&mut Located<&'a str>) -> PResult<Token> + '_ {
    move |s| {
        tag(punct)
            .with_span()
            .map(token(kind.clone()))
            .parse_next(s)
    }
}
