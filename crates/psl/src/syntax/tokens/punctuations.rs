use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_punctuations(s: &mut Located<&str>) -> PResult<Token> {
    // please sort alt by enum declaration order
    alt([
        punct("!", TokenKind::PunctuationExclamationMark),
        punct("#", TokenKind::PunctuationNumberSign),
        punct("$", TokenKind::PunctuationDollarSign),
        punct("%", TokenKind::PunctuationPercentSign),
        punct("&", TokenKind::PunctuationAmpersand),
        punct("(", TokenKind::PunctuationLeftParenthesis),
        punct(")", TokenKind::PunctuationRightParenthesis),
        punct("*", TokenKind::PunctuationAsterisk),
        punct("+", TokenKind::PunctuationPlusSign),
        punct(",", TokenKind::PunctuationComma),
        punct("-", TokenKind::PunctuationHyphenMinus),
        punct(".", TokenKind::PunctuationFullStop),
        punct("/", TokenKind::PunctuationSolidus),
        punct(":", TokenKind::PunctuationColon),
        punct(";", TokenKind::PunctuationSemicolon),
        punct("<", TokenKind::PunctuationLessThanSign),
        punct("=", TokenKind::PunctuationEqualsSign),
        punct(">", TokenKind::PunctuationGreaterThanSign),
        punct("?", TokenKind::PunctuationQuestionMark),
        punct("@", TokenKind::PunctuationCommercialAt),
        punct("[", TokenKind::PunctuationLeftSquareBracket),
        punct("\\", TokenKind::PunctuationReverseSolidus),
        punct("]", TokenKind::PunctuationRightSquareBracket),
        punct("^", TokenKind::PunctuationCircumflexAccent),
        punct("_", TokenKind::PunctuationLowLine),
        punct("{", TokenKind::PunctuationLeftCurlyBracket),
        punct("|", TokenKind::PunctuationVerticalLine),
        punct("}", TokenKind::PunctuationRightCurlyBracket),
        punct("~", TokenKind::PunctuationTilde),
    ])
    .parse_next(s)
}

fn punct(
    punct: &str,
    kind: TokenKind,
) -> impl for<'a> Fn(&mut Located<&'a str>) -> PResult<Token> + '_ {
    move |s| punct.with_span().map(token(kind.clone())).parse_next(s)
}
