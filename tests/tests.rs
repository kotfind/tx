use assert_cmd::Command;
use indoc::indoc;

#[test]
fn print_header() {
    Command::cargo_bin("tx")
        .unwrap()
        .args([r#"2 1"#, "--print-header"])
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
fn column_numbers_select() {
    Command::cargo_bin("tx")
        .unwrap()
        .args([r#"2 1"#])
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
fn column_names_select() {
    Command::cargo_bin("tx")
        .unwrap()
        .args([r#"B A"#])
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
