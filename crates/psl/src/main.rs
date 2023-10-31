mod diagnostics;

use psl::{generate_codes, syntax::parse_program};
use std::{
    borrow::Cow,
    env::args,
    fs,
    path::PathBuf,
    process::{Command, ExitCode},
    str::FromStr,
};
use winnow::{Located, Parser};

use crate::diagnostics::Diagnostic;

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
            Diagnostic::from_parse_error(Cow::Borrowed(&path), e).write();

            return ExitCode::FAILURE;
        }
    };

    let mut output_path = PathBuf::from_str(&path).unwrap();
    let mut output_filename = output_path.file_stem().unwrap().to_owned();
    output_filename.push(".c");
    output_path.set_file_name(output_filename);

    let output = generate_codes(ast);

    fs::write(&output_path, output).unwrap();

    eprintln!("C code generated, run gcc for compile.");

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

    eprintln!("Run the program");
    eprintln!("-------------------------------------------");

    Command::new(&executable_path)
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    ExitCode::SUCCESS
}
