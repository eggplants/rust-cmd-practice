use std::io::BufRead;

pub enum FileBuffer {
    Stdin(std::io::Stdin),
    File(std::fs::File),
}

pub enum AmountMode {
    Bytes(usize),
    Lines(usize),
}

pub fn print_lines(file_buffer: &FileBuffer, number_mode: &AmountMode) {
    let mut reader: Box<dyn BufRead> = match file_buffer {
        FileBuffer::Stdin(stdin) => Box::new(stdin.lock()),
        FileBuffer::File(file) => Box::new(std::io::BufReader::new(file)),
    };

    match number_mode {
        AmountMode::Bytes(bytes) => {
            let mut buf = vec![0; *bytes];
            let num_bytes = reader.read(&mut buf).unwrap();
            print!("{}", String::from_utf8_lossy(&buf[..num_bytes]));
        }
        AmountMode::Lines(lines) => {
            let mut buf = String::new();
            for _ in 0..*lines {
                let bytes = reader.read_line(&mut buf).unwrap();
                if bytes == 0 {
                    break;
                }
                print!("{}", buf);
                buf.clear();
            }
        }
    }
}
