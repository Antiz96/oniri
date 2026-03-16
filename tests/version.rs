// Import the assert_cmd and predicates modules to respectively assert on stdout/stderr and check exit codes
use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn version_arg() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("--version")
        .assert()
        .success()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}
