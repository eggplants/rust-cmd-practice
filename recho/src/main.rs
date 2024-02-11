use clap::Parser;

#[derive(Parser)]
#[command(about, author, version)]
struct Args {
    #[arg(value_name = "TEXT", help = "Input text")]
    text: Vec<String>,

    #[arg(short = 'n', long, help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    if args.omit_newline {
        print!("{}", args.text.join(" "));
    } else {
        println!("{}", args.text.join(" "));
    }
}
