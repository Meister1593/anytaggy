mod files;
mod tag;
mod tags;

use assert_cmd::Command;

fn cargo_bin_cmd() -> Command {
    Command::cargo_bin("anytaggy").unwrap()
}

#[test]
fn blank_test() {
    let mut cmd = cargo_bin_cmd();
    cmd.assert().failure().code(2);
}
