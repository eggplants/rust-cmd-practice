use std::io::BufRead;

pub enum FileBuffer {
    Stdin(std::io::Stdin),
    File(std::fs::File),
}
pub enum NumberMode {
    All,
    NonBlank,
    None,
}

pub fn print_lines(file_buffer: &FileBuffer, number_mode: &NumberMode) {
    let reader: Box<dyn BufRead> = match file_buffer {
        FileBuffer::Stdin(stdin) => Box::new(stdin.lock()),
        FileBuffer::File(file) => Box::new(std::io::BufReader::new(file)),
    };

    match number_mode {
        NumberMode::None => {
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        NumberMode::All => {
            for (lineno, line) in reader.lines().enumerate() {
                println!("{:>6}:\t{}", lineno + 1, line.unwrap());
            }
        }
        NumberMode::NonBlank => {
            let mut lineno = 0;
            for line in reader.lines() {
                let line_str = line.unwrap();
                if line_str == "" {
                    println!("");
                } else {
                    lineno += 1;
                    println!("{:>6}:\t{}", lineno, line_str);
                }
            }
        }
    }
}
