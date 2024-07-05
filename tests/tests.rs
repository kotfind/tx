use assert_cmd::Command;
use indoc::indoc;

#[test]
fn basic() {
    Command::cargo_bin("tx")
        .unwrap()
        .args(["2 1"])
        .write_stdin(indoc! {r#"
            A B C
            1 2 3
            4 5 6
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            B A
            2 1
            5 4
        "#});
}

#[test]
fn named_columns() {
    Command::cargo_bin("tx")
        .unwrap()
        .args(["B A"])
        .write_stdin(indoc! {r#"
            A B C
            1 2 3
            4 5 6
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            2 1
            5 4
        "#});
}

#[test]
fn print_header() {
    Command::cargo_bin("tx")
        .unwrap()
        .args(["-h", "2 1"])
        .write_stdin(indoc! {r#"
            A B C
            1 2 3
            4 5 6
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            B A
            2 1
            5 4
        "#});
}

#[test]
fn has_header_no_print() {
    Command::cargo_bin("tx")
        .unwrap()
        .args(["-H", "2 1"])
        .write_stdin(indoc! {r#"
            A B C
            1 2 3
            4 5 6
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            2 1
            5 4
        "#});
}
