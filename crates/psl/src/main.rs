use ariadne::{Label, Report, Source};
use psl::{
    generate_codes,
    syntax::{parse_program, parse_token},
};
use std::{
    env::args,
    fs,
    path::PathBuf,
    process::{Command, ExitCode},
    str::FromStr,
};
use winnow::{Located, Parser};

fn main() -> ExitCode {
    let Some(path) = args().nth(1) else {
        eprintln!("please specify file to compile");
        return ExitCode::FAILURE
    };

    let Ok(content) = fs::read_to_string(&path) else {
        eprintln!("file not found: {path}");
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

            Report::build(ariadne::ReportKind::Error, &path, e.offset())
                .with_code(1)
                .with_message(error_message)
                .with_label(Label::new((
                    &path,
                    e.offset()..e.offset() + next_token.len(),
                )))
                .finish()
                .print((&path, Source::from(raw_input)))
                .unwrap();

            return ExitCode::FAILURE;
        }
    };

    let mut output_path = PathBuf::from_str(&path).unwrap();
    let mut output_filename = output_path.file_stem().unwrap().to_owned();
    output_filename.push(".c");
    output_path.set_file_name(output_filename);

    let output = generate_codes(ast);

    fs::write(&output_path, output).unwrap();

    println!("C code generated, run gcc for compilation.");

    let mut executable_path = PathBuf::from_str(&path).unwrap();
    let mut executable_filename = executable_path.file_stem().unwrap().to_owned();
    executable_filename.push(".o");
    executable_path.set_file_name(executable_filename);

    Command::new("gcc")
        .arg(&output_path)
        .arg("-o")
        .arg(&executable_path)
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    Command::new(&executable_path)
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    ExitCode::SUCCESS
}
