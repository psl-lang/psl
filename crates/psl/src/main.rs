use ariadne::{Label, Report, Source};
use psl::{
    generate_codes,
    syntax::{parse_program, parse_token},
};
use std::{env::args, fs, process::ExitCode};
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

    let ast = match parse_program.parse(Located::new(content.as_ref())) {
        Ok(ast) => ast,
        Err(e) => {
            let next_token = parse_token
                .parse_next(&mut Located::new(&e.input()[e.offset()..]))
                .map(|token| token.content)
                .unwrap();

            let mut error_message = e.inner().to_string();
            if error_message.is_empty() {
                error_message = format!("unexpected token {next_token:?}");
            }

            let raw_input: &str = &e.input();

            Report::build(ariadne::ReportKind::Error, &file, e.offset())
                .with_code(1)
                .with_message(error_message)
                .with_label(Label::new((
                    &file,
                    e.offset()..e.offset() + next_token.len(),
                )))
                .finish()
                .print((&file, Source::from(raw_input)))
                .unwrap();

            return ExitCode::FAILURE;
        }
    };

    let output = generate_codes(ast);

    println!("{output}");

    ExitCode::SUCCESS
}
