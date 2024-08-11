use clap::Parser;
use std::path::PathBuf;

mod note;

use note::parse_note;

#[derive(Parser)]
struct Args {
    #[arg()]
    file: PathBuf,
}


fn main() {
    let note = parse_note(Args::parse().file);
    let ts = note.file_summary();

    println!("{}", ts);
}
