/* use::std::process::Command; */
use assert_cmd::Command;

#[test]
fn works() {
    Command::cargo_bin("hello")
        .unwrap()
        .assert()
        .success()
        .stdout("Hello, world!\n");
}

#[test]
fn works_with_single_arg() {
    Command::cargo_bin("hello")
        .unwrap()
        .args(&["te", "st"])
        .assert()
        .success()
        .stdout("Hello, world, [te,st]!\n");
}
