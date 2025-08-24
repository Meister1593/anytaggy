use assert_cmd::Command;

#[test]
fn blank_test() {
    let mut cmd = Command::cargo_bin("anytaggy").unwrap();
    cmd.assert().failure();
}
