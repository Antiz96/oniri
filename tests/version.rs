// Import modules
// assert_cmd::Command to run a command and assert on exit code & stdout/stderr
// predicates::str::contains to predicate command output
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
