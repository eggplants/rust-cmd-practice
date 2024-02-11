use assert_cmd::Command;

use std::{fs, path::Path};

const PROG_NAME: &str = "rcat";

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
        .stderr(format!("rcat: {}: No such file or directory\n", path));
}

#[test]
fn run_with_multiple_files() {
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .arg(get_input_pathstr("single.txt"))
        .arg(get_input_pathstr("multi.txt"))
        .assert()
        .success()
        .stdout(["test a b c", "te", "st\n"].join("\n"))
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
        .stdout("test a b c\n")
        .stderr("rcat: tests/inputs/invalid.txt: No such file or directory\n");
}

#[test]
fn run_with_number_opt() {
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .arg("-n")
        .arg(get_input_pathstr("single.txt"))
        .arg(get_input_pathstr("multi.txt"))
        .arg(get_input_pathstr("multi_with_blank.txt"))
        .assert()
        .success()
        .stdout(
            [
                "     1:\ttest a b c",
                "     1:\tte",
                "     2:\tst",
                "     1:\tte",
                "     2:\t",
                "     3:\t",
                "     4:\ts",
                "     5:\tt\n",
            ]
            .join("\n"),
        )
        .stderr("");
}

#[test]
fn run_with_number_nonblank_opt() {
    Command::cargo_bin(PROG_NAME)
        .unwrap()
        .arg("-b")
        .arg(get_input_pathstr("single.txt"))
        .arg(get_input_pathstr("multi.txt"))
        .arg(get_input_pathstr("multi_with_blank.txt"))
        .assert()
        .success()
        .stdout(
            [
                "     1:\ttest a b c",
                "     1:\tte",
                "     2:\tst",
                "     1:\tte",
                "",
                "",
                "     2:\ts",
                "     3:\tt\n",
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
        .arg("-b")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(["     1:\tte", "", "", "     2:\ts", "     3:\tt\n"].join("\n"))
        .stderr("");
}
