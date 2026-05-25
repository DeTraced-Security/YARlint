use clap::Parser;

use yarlint::app::yarlint_pipeline;
use yarlint::cli::Args;
use yarlint::cli::output::{
    print_error,
    print_recursive_warning,
    print_scan_start,
};

///
fn main() {
    let args = Args::parse();

    print_scan_start(&args.path);

    if args.recursive {
        print_recursive_warning();
    }

    if let Err(err) = yarlint_pipeline(&args) {
        print_error(&err.to_string());
        std::process::exit(1);
    }
}