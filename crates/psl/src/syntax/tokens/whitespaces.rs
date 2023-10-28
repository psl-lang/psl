use winnow::{token::take_while, Located, PResult, Parser};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_whitespace_horizontal(s: &mut Located<&str>) -> PResult<Token> {
    take_while(
        1..,
        [
            '\u{0009}', // CHARACTER TABULATION
            '\u{0020}', // SPACE
            '\u{00AD}', // SOFT HYPHEN
            '\u{00A0}', // NO-BREAK SPACE
            '\u{1680}', // OGHAM SPACE MARK
            '\u{2000}', // EN QUAD
            '\u{2001}', // EM QUAD
            '\u{2002}', // EN SPACE
            '\u{2003}', // EM SPACE
            '\u{2004}', // THREE-PER-EM SPACE
            '\u{2005}', // FOUR-PER-EM SPACE
            '\u{2006}', // SIX-PER-EM SPACE
            '\u{2007}', // FIGURE SPACE
            '\u{2008}', // PUNCTUATION SPACE
            '\u{2009}', // THIN SPACE
            '\u{200A}', // HAIR SPACE
            '\u{200B}', // ZERO WIDTH SPACE
            '\u{200E}', // LEFT-TO-RIGHT MARK
            '\u{200F}', // RIGHT-TO-LEFT MARK
            '\u{202F}', // NARROW NO-BREAK SPACE
            '\u{205F}', // MEDIUM MATHEMATICAL SPACE
            '\u{3000}', // IDEPGRAPHIC SPACE
            '\u{FEFF}', // ZERO WIDTH NO-BREAK SPACE
        ],
    )
    .with_span()
    .map(token(TokenKind::WhitespaceHorizontal))
    .parse_next(s)
}

pub fn parse_whitespace_vertical(s: &mut Located<&str>) -> PResult<Token> {
    take_while(
        1..,
        [
            '\u{000A}', // LINE FEED
            '\u{000B}', // LINE TABULATION
            '\u{000C}', // FORM FEED
            '\u{000D}', // CARRIAGE RETURN
            '\u{0085}', // NEXT LINE
            '\u{2028}', // LINE SEPARATOR
            '\u{2029}', // PARAGRAPH SEPARATOR
        ],
    )
    .with_span()
    .map(token(TokenKind::WhitespaceVertical))
    .parse_next(s)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use winnow::{Located, Parser};

    use crate::{
        ast::{Token, TokenKind},
        syntax::tokens::whitespaces::{parse_whitespace_horizontal, parse_whitespace_vertical},
    };

    #[test]
    fn test_horizontal() {
        assert_eq!(
            parse_whitespace_horizontal.parse(Located::new(" \t")),
            Ok(Token {
                kind: TokenKind::WhitespaceHorizontal,
                span: 0..2,
                content: " \t".to_string()
            }),
        )
    }

    #[test]
    fn test_vertical() {
        assert_eq!(
            parse_whitespace_vertical.parse(Located::new("\n\r")),
            Ok(Token {
                kind: TokenKind::WhitespaceVertical,
                span: 0..2,
                content: "\n\r".to_string()
            }),
        )
    }
}
