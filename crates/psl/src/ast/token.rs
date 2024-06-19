use std::{
    fmt::{self, Write},
    ops::Range,
};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum TokenKind {
    WhitespaceHorizontal,
    WhitespaceVertical,

    IdentifierIdentifier,

    LiteralIntegerDecimal,
    LiteralIntegerHexadecimal,
    LiteralIntegerBinary,
    LiteralFormatSpecifier(FormatSpecifier),

    // please sort keyword alphabetically
    KeywordElse,
    KeywordFn,
    KeywordIf,
    KeywordRead,
    KeywordReturn,
    KeywordWhile,
    KeywordWrite,

    // please sort punctuation by its ascii code
    // please name enum variant Punctuation (PascalCase-d unicode name[1])
    // [1]: https://www.unicode.org/charts/PDF/U0000.pdf
    PunctuationExclamationMark,
    PunctuationNumberSign,
    PunctuationDollarSign,
    PunctuationPercentSign,
    PunctuationAmpersand,
    PunctuationLeftParenthesis,
    PunctuationRightParenthesis,
    PunctuationAsterisk,
    PunctuationPlusSign,
    PunctuationComma,
    PunctuationHyphenMinus,
    PunctuationFullStop,
    PunctuationSolidus,
    PunctuationColon,
    PunctuationSemicolon,
    PunctuationLessThanSign,
    PunctuationEqualsSign,
    PunctuationGreaterThanSign,
    PunctuationQuestionMark,
    PunctuationCommercialAt,
    PunctuationLeftSquareBracket,
    PunctuationReverseSolidus,
    PunctuationRightSquareBracket,
    PunctuationCircumflexAccent,
    PunctuationLowLine,
    PunctuationLeftCurlyBracket,
    PunctuationVerticalLine,
    PunctuationRightCurlyBracket,
    PunctuationTilde,

    Error,
    Eof,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
    pub span: Range<usize>,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct FormatSpecifier(pub Vec<FormatSpecifierFragment>);

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum FormatSpecifierFragment {
    Text(String),
    Whitespace(String),
    Variable(String),
}

impl fmt::Display for FormatSpecifierFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatSpecifierFragment::Text(s) => s.fmt(f),
            FormatSpecifierFragment::Whitespace(s) => s.fmt(f),
            FormatSpecifierFragment::Variable(s) => {
                f.write_char('{')?;
                s.fmt(f)?;
                f.write_char('}')?;
                Ok(())
            }
        }
    }
}
