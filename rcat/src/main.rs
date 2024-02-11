use clap::{CommandFactory, Parser};
use rcat::{print_lines, FileBuffer, NumberMode};
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, author, version)]
struct Args {
    #[arg(value_name = "FILE", help = "Input File")]
    files: Vec<String>,

    #[arg(
        short = 'b',
        long,
        group = "number_lines",
        help = "Number nonempty output lines, overrides -n"
    )]
    number_nonblank: bool,

    #[arg(short, long, group = "number_lines", help = "Number all output lines")]
    number: bool,
}

fn main() {
    let args = Args::parse();
    let number_mode = if args.number {
        NumberMode::All
    } else if args.number_nonblank {
        NumberMode::NonBlank
    } else {
        NumberMode::None
    };

    if args.files.len() == 0 {
        let stdin = FileBuffer::Stdin(std::io::stdin());
        print_lines(&stdin, &number_mode);
    }

    for file_pathstr in args.files {
        let file = PathBuf::from(file_pathstr.clone());
        if !file.is_file() {
            eprintln!(
                "{}: {}: No such file or directory",
                Args::command().get_name(),
                file_pathstr,
            );
            std::process::exit(1)
        }
        let file_buf = FileBuffer::File(File::open(file).unwrap());
        print_lines(&file_buf, &number_mode);
    }
}
