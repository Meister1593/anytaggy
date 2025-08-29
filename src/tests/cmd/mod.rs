mod files;
mod rm_tags;
mod tag;
mod tags;
mod untag;

use assert_cmd::Command;

fn cargo_bin_cmd() -> Command {
    Command::cargo_bin("../../debug/anytaggy").unwrap()
}

#[test]
fn blank_test() {
    let mut cmd = cargo_bin_cmd();
    cmd.assert().failure().code(2);
}
