use std::process::Command;

#[test]
fn test_ls_command() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}