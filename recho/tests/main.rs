use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn run_with_no_arg() -> TestResult {
    Command::cargo_bin("recho")?
        // .unwrap()
        .assert()
        .success()
        .stdout("\n")
        .stderr("");
    Ok(())
}

#[test]
fn run_with_several_posargs() {
    Command::cargo_bin("recho").unwrap()
        // .arg("te")
        // .arg("st")
        // .arg("!\n\n\n")
        .args(&["te", "st", "!\n\n\n"])
        .assert()
        .success()
        .stdout("te st !\n\n\n\n")
        .stderr("");
}

#[test]
fn run_with_omit_newline_shortopt() {
    Command::cargo_bin("recho")
        .unwrap()
        .arg("-n")
        .arg("te")
        .arg("st")
        .arg("!\n\n\n")
        .assert()
        .success()
        .stdout("te st !\n\n\n")
        .stderr("");
}

#[test]
fn run_with_omit_newline_longopt() {
    Command::cargo_bin("recho")
        .unwrap()
        .arg("--omit-newline")
        .assert()
        .success()
        .stdout("")
        .stderr("");
}
