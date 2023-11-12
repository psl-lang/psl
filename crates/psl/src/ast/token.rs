use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    WhitespaceHorizontal,
    WhitespaceVertical,

    IdentifierIdentifier,

    KeywordRead,
    KeywordWrite,
    KeywordIf,
    KeywordThen,
    KeywordElse,

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

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
    pub span: Range<usize>,
}
