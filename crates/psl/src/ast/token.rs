use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    WhitespaceHorizontal,
    WhitespaceVertical,

    IdentifierIdentifier,

    KeywordRead,
    KeywordWrite,

    PunctuationAsterisk,
    PunctuationPlusSign,
    PunctuationHyphenMinus,
    PunctuationSolidus,
    PonctuationEqualsSign,

    Error,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
    pub span: Range<usize>,
}
