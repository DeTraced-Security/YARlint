use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to scan
    #[arg(short, long)]
    pub path: String,

    /// Recursively scan directories
    #[arg(short, long)]
    pub recursive: bool,

    /// Max Depth for Directories
    #[arg(short, long)]
    pub depth: Option<usize>,

}