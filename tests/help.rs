//! Test the --help argument

use assert_cmd::Command;

#[test]
fn help_arg() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("--help")
        .assert()
        .success();
}
