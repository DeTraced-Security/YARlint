use crate::cli::Args;
use crate::cli::output::{
    print_valid_file_summary,
    print_file_summary,
};

use crate::filesystem::collect_yara_files;
use crate::parser::parser::parse_files;
use crate::validation::validate_files;

pub fn yarlint_pipeline(args: &Args) -> Result<(), String> {

    let files: Vec<std::path::PathBuf> = collect_yara_files(
        &args.path,
        args.recursive,
        args.depth,
    )?;

    print_file_summary(files.len());


    let valid_files: Vec<std::path::PathBuf> = validate_files(&files)?;

    print_valid_file_summary(valid_files.len());

    let parsed_files: &Vec<std::path::PathBuf>  = parse_files(&valid_files)?;

    Ok(())
}