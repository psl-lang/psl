use std::borrow::Cow;

use psl::syntax::parse_token;
use winnow::{
    error::{ContextError, ParseError},
    Located, Parser,
};

use crate::diagnostics::{
    diagnostic_code::{DiagnosticsCode, ErrorCode},
    Diagnostic,
};

impl<'a> Diagnostic<'a> {
    pub fn from_parse_error(
        filename: Cow<'a, str>,
        value: ParseError<Located<&'a str>, ContextError>,
    ) -> Self {
        let next_token = parse_token
            .parse_next(&mut Located::new(&value.input()[value.offset()..]))
            .unwrap();

        let mut message = value.inner().to_string();
        if message.is_empty() {
            message = format!("unexpected token {:?}", next_token.content);
        }

        Self {
            filename,
            file_content: value.input(),

            code: DiagnosticsCode::Error(ErrorCode::Syntax),
            message: Cow::Owned(message),
            span: value.offset()..value.offset() + next_token.content.len(),
        }
    }
}
