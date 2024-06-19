use winnow::{
    combinator::{alt, cut_err, opt, preceded, separated1, separated_pair, terminated},
    Located, PResult, Parser,
};

use crate::{
    ast::{ExpressionOrBlock, FunctionDeclaration, TokenKind},
    syntax::{
        expressions::parse_expression,
        fragments::{block::parse_block, r#type::parse_type, separator::parse_separator},
    },
};

pub fn parse_function_declaration(s: &mut Located<&str>) -> PResult<FunctionDeclaration> {
    (
        TokenKind::KeywordFn,
        cut_err((
            TokenKind::WhitespaceHorizontal,
            TokenKind::IdentifierIdentifier,
            TokenKind::PunctuationLeftParenthesis,
            opt(terminated(
                separated1(
                    separated_pair(
                        parse_type,
                        TokenKind::WhitespaceHorizontal,
                        TokenKind::IdentifierIdentifier,
                    ),
                    (TokenKind::PunctuationComma, TokenKind::WhitespaceHorizontal),
                ),
                opt(TokenKind::PunctuationComma),
            )),
            TokenKind::PunctuationRightParenthesis,
            opt(TokenKind::WhitespaceHorizontal),
            parse_type,
            opt(TokenKind::WhitespaceHorizontal),
            alt((
                parse_block.map(ExpressionOrBlock::Block),
                preceded(
                    (
                        TokenKind::PunctuationEqualsSign,
                        opt(TokenKind::WhitespaceHorizontal),
                    ),
                    cut_err(parse_expression),
                )
                .map(ExpressionOrBlock::Expression),
            )),
            parse_separator,
        )),
    )
        .map(
            |(_, (_, name, _, parameters, _, _, return_type, _, body, _))| FunctionDeclaration {
                name,
                parameters: parameters.unwrap_or(Vec::new()),
                return_type,
                body,
            },
        )
        .parse_next(s)
}
