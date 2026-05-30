use crate::parser::{ast::ast_parser, lexer::lex_file};

pub fn parse_files(files: &Vec<std::path::PathBuf>) -> Result<&Vec<std::path::PathBuf>, String> {

    for file in files {
        let lexed_file: &std::path::PathBuf = lex_file(file)?;
        let ast_file: &std::path::PathBuf = ast_parser(lexed_file)?;
        //println!("{}", ast_file.display());
    }

    return Ok(files);
}