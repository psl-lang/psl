use std::{borrow::Cow, ops::Range};

use ariadne::{Label, Report, ReportKind, Source};

use self::diagnostic_code::DiagnosticsCode;

mod diagnostic_code;
mod transform;

pub struct Diagnostic<'a> {
    filename: Cow<'a, str>,
    file_content: &'a str,

    code: DiagnosticsCode,

    span: Range<usize>,
    message: Cow<'a, str>,
}

impl Diagnostic<'_> {
    pub fn write(self) {
        Report::build(
            match self.code {
                DiagnosticsCode::Error(_) => ReportKind::Error,
            },
            &self.filename,
            self.span.start,
        )
        .with_code(match self.code {
            DiagnosticsCode::Error(code) => code as u32,
        })
        .with_message(self.message)
        .with_label(Label::new((&self.filename, self.span)))
        .finish()
        .eprint((&self.filename, Source::from(&self.file_content)))
        .unwrap();
    }
}
