use clap::{CommandFactory, Parser};
use rhead::{print_lines, AmountMode, FileBuffer};
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, author, version)]
struct Args {
    #[arg(value_name = "FILE", help = "Input File")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long,
        group = "amount",
        value_name = "LINES",
        help = "Number all output lines"
    )]
    lines: Option<usize>,

    #[arg(
        short = 'c',
        long,
        group = "amount",
        value_name = "BYTES",
        help = "Print the first NUM bytes of each file"
    )]
    bytes: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let amount_mode: AmountMode = if args.bytes.is_some() {
        AmountMode::Bytes(args.bytes.unwrap())
    } else if args.lines.is_some() {
        AmountMode::Lines(args.lines.unwrap())
    } else {
        AmountMode::Lines(10)
    };

    let file_len = args.files.len();
    if file_len == 0 {
        let stdin = FileBuffer::Stdin(std::io::stdin());
        print_lines(&stdin, &amount_mode);
    }

    for (file_idx, file_pathstr) in args.files.iter().enumerate() {
        let file = PathBuf::from(file_pathstr.clone());
        if !file.is_file() {
            eprintln!(
                "{}: {}: No such file or directory",
                Args::command().get_name(),
                file_pathstr,
            );
            std::process::exit(1)
        }

        if file_len > 1 {
            println!("==> {} <==", file_pathstr);
        }

        let file_buf = FileBuffer::File(File::open(file).unwrap());
        print_lines(&file_buf, &amount_mode);

        if file_idx != file_len - 1 {
            println!("")
        }
    }
}
