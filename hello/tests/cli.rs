use std::process::Command;
use assert_cmd :: Command as assert_cmd_Command;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {

    let mut cmd = Command::new("ls");
    let res = cmd.output();

    assert!(res.is_ok());
}

#[test]
fn hello_runs() {
    let mut cmd = assert_cmd_Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
    cmd.assert().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = assert_cmd_Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = assert_cmd_Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn abort() {
    let mut cmd = assert_cmd_Command::cargo_bin("abort").unwrap();
    cmd.assert().failure();
}