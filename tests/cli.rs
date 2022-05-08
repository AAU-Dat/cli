use assert_cmd::Command;
// use predicates::prelude::*;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("aau").unwrap();
    cmd.assert().success();
}
