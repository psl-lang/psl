use std::ops::Range;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum TokenKind {
    WhitespaceHorizontal,
    WhitespaceVertical,

    IdentifierIdentifier,

    KeywordRead,
    KeywordWrite,
    KeywordIf,
    KeywordThen,
    KeywordElse,

    PunctuationAsterisk,
    PunctuationPlusSign,
    PunctuationHyphenMinus,
    PunctuationSolidus,
    PunctuationPercent,
    PunctuationAmpersand,
    PunctuationPipe,
    PunctuationEqualsSign,
    PunctuationLessSign,
    PunctuationGreaterSign,
    PunctuationExclamationMark,

    Error,
    Eof,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
    pub span: Range<usize>,
}
