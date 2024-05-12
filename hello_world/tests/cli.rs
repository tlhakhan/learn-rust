use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn test_hello_world_command() {
    let mut cmd = Command::cargo_bin("hello_world").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn test_true_command() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn test_false_command() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}