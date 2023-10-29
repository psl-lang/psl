use std::{env::args, fs, process::ExitCode};

use line_span::LineSpanExt;
use psl::syntax::{parse_program, parse_token};
use unicode_width::UnicodeWidthStr;
use winnow::{Located, Parser};

fn main() -> ExitCode {
    let Some(file) = args().nth(1) else {
        eprintln!("please specify file to compile");
        return ExitCode::FAILURE
    };

    let Ok(content) = fs::read_to_string(&file) else {
        eprintln!("file not found: {file}");
        return ExitCode::FAILURE
    };

    let ast = match parse_program
        .with_span()
        .parse(Located::new(content.as_ref()))
    {
        Ok(ast) => ast,
        Err(e) => {
            let next_token = parse_token
                .parse_next(&mut Located::new(&e.input()[e.offset()..]))
                .map(|token| token.content)
                .unwrap_or("".to_string());

            let mut error_message = e.inner().to_string();
            if error_message.is_empty() {
                error_message = format!("unexpected token {next_token:?}");
            }
            eprintln!("compilation error: {}", error_message);

            if let Some(range) = e.input().find_prev_line_range(e.offset()) {
                eprintln!("  | {}", &e.input()[range]);
            }

            let range = e.input().find_line_range(e.offset());
            eprintln!("  | {}", &e.input()[range.clone()]);

            eprintln!(
                "  | {}{}",
                " ".repeat(UnicodeWidthStr::width(&e.input()[range.start..e.offset()])),
                "^".repeat(next_token.len()),
            );

            if let Some(range) = e.input().find_next_line_range(e.offset()) {
                eprintln!("  | {}", &e.input()[range]);
            }

            return ExitCode::FAILURE;
        }
    };

    println!("{ast:#?}");

    ExitCode::SUCCESS
}
