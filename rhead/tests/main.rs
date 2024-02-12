use assert_cmd::Command;

use std::{fs, path::Path};

const PROG_NAME: &str = "rhead";

fn get_input_pathstr(file_name: &str) -> String {
    Path::new("tests")
        .join("inputs")
        .join(file_name)
        .to_str()
        .unwrap()
        .to_owned()
}

#[test]
fn run_with_empty_file() {
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .arg(get_input_pathstr("empty.txt"))
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn run_with_invalid_file_path() {
    let path = get_input_pathstr("invalid.txt");
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .arg(&path)
        .assert()
        .failure()
        .stdout("")
        .stderr(format!(
            "{}: {}: No such file or directory\n",
            PROG_NAME, path
        ));
}

#[test]
fn run_with_multiple_files() {
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .arg(get_input_pathstr("single.txt"))
        .arg(get_input_pathstr("multi.txt"))
        .assert()
        .success()
        .stdout(
            [
                "==> tests/inputs/single.txt <==",
                "てすと👌",
                "",
                "==> tests/inputs/multi.txt <==",
                "ᒼ☊ᐣ",
                "↑これ、カービィっぽくないすか？",
                "ᒼ☊ᐣ",
                "↑これ、カービィっぽくないすか？",
                "ᒼ☊ᐣ",
                "↑これ、カービィっぽくないすか？",
                "ᒼ☊ᐣ",
                "↑これ、カービィっぽくないすか？",
                "ᒼ☊ᐣ",
                "↑これ、カービィっぽくないすか？\n",
            ]
            .join("\n"),
        )
        .stderr("");
}

#[test]
fn run_with_files_including_invalid() {
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .arg(get_input_pathstr("single.txt"))
        .arg(get_input_pathstr("invalid.txt"))
        .assert()
        .failure()
        .stdout(["==> tests/inputs/single.txt <==", "てすと👌\n\n"].join("\n"))
        .stderr(format!(
            "{}: tests/inputs/invalid.txt: No such file or directory\n",
            PROG_NAME
        ));
}

#[test]
fn run_with_number_opt() {
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .args(["-c", "3"])
        .arg(get_input_pathstr("single.txt"))
        .arg(get_input_pathstr("multi.txt"))
        .arg(get_input_pathstr("multi_with_blank.txt"))
        .assert()
        .success()
        .stdout(
            [
                "==> tests/inputs/single.txt <==",
                "て",
                "==> tests/inputs/multi.txt <==",
                "ᒼ",
                "==> tests/inputs/multi_with_blank.txt <==",
                "\u{3000}",
            ]
            .join("\n"),
        )
        .stderr("");
}

#[test]
fn run_with_number_nonblank_opt() {
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .args(&["-n", "2"])
        .arg(get_input_pathstr("single.txt"))
        .arg(get_input_pathstr("multi.txt"))
        .arg(get_input_pathstr("multi_with_blank.txt"))
        .assert()
        .success()
        .stdout(
            [
                "==> tests/inputs/single.txt <==",
                "てすと👌",
                "",
                "==> tests/inputs/multi.txt <==",
                "ᒼ☊ᐣ",
                "↑これ、カービィっぽくないすか？",
                "",
                "==> tests/inputs/multi_with_blank.txt <==",
                "\u{3000}\u{3000}\u{3000}\u{3000}！あ!",
                "\n",
            ]
            .join("\n"),
        )
        .stderr("");
}

#[test]
fn run_with_stdin_input() {
    let input = fs::read_to_string(get_input_pathstr("multi_with_blank.txt")).unwrap();
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .args(["-n", "1"])
        .write_stdin(input)
        .assert()
        .success()
        .stdout("\u{3000}\u{3000}\u{3000}\u{3000}！あ!\n")
        .stderr("");
}
