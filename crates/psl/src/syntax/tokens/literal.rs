use std::fmt::Display;

use winnow::{
    combinator::{alt, preceded, repeat},
    error::ContextError,
    token::{one_of, take_while},
    Located, PResult, Parser,
};

use crate::ast::{FormatSpecifier, FormatSpecifierFragment, Token, TokenKind};

use super::token;

pub fn parse_literal(s: &mut Located<&str>) -> PResult<Token> {
    alt((parse_integer_literal, parse_format_specifier)).parse_next(s)
}

pub fn parse_integer_literal(s: &mut Located<&str>) -> PResult<Token> {
    alt((parse_hexadecimal, parse_binary, parse_decimal)).parse_next(s)
}

pub fn parse_decimal(s: &mut Located<&str>) -> PResult<Token> {
    may_have_underscores_between(one_of('0'..='9'), one_of('0'..='9'))
        .with_span()
        .map(token(TokenKind::LiteralIntegerDecimal))
        .parse_next(s)
}

pub fn parse_hexadecimal(s: &mut Located<&str>) -> PResult<Token> {
    may_have_underscores_between("0x", one_of(('0'..='9', 'a'..='f', 'A'..='F')))
        .with_span()
        .map(token(TokenKind::LiteralIntegerHexadecimal))
        .parse_next(s)
}

pub fn parse_binary(s: &mut Located<&str>) -> PResult<Token> {
    may_have_underscores_between("0b", one_of(('0', '1')))
        .with_span()
        .map(token(TokenKind::LiteralIntegerBinary))
        .parse_next(s)
}

fn may_have_underscores_between<'a, HeadParser, Head, RestParser, Rest>(
    head: HeadParser,
    rest: RestParser,
) -> impl Parser<Located<&'a str>, String, ContextError>
where
    HeadParser: Parser<Located<&'a str>, Head, ContextError>,
    Head: Display,
    RestParser: Parser<Located<&'a str>, Rest, ContextError>,
    String: FromIterator<Rest>,
{
    (head, repeat(0.., preceded(take_while(0.., '_'), rest)))
        .map(|(h, t): (_, Vec<_>)| format!("{}{}", h, String::from_iter(t)))
}

pub fn parse_format_specifier(s: &mut Located<&str>) -> PResult<Token> {
    (
        '`',
        repeat::<_, _, Vec<_>, _, _>(0.., parse_format_specifier_fragment),
        '`',
    )
        .with_span()
        .map(|((_, seq, _), span)| {
            let content = format!(
                "`{}`",
                seq.iter().map(|frag| frag.to_string()).collect::<String>()
            );
            Token {
                kind: TokenKind::LiteralFormatSpecifier(FormatSpecifier(seq)),
                content,
                span,
            }
        })
        .parse_next(s)
}

pub fn parse_format_specifier_fragment(s: &mut Located<&str>) -> PResult<FormatSpecifierFragment> {
    alt((
        repeat(1.., alt((' ', "\\n".map(|_| '\n')))).map(FormatSpecifierFragment::Whitespace),
        ('{', TokenKind::IdentifierIdentifier, '}')
            .map(|(_, ident, _)| FormatSpecifierFragment::Variable(ident.content)),
    ))
    .parse_next(s)
}
