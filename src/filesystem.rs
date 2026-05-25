pub mod collect;
pub mod filters;

pub use collect::collect_yara_files;
pub use filters::is_yara_file;