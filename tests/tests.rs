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

#[test]
fn smart_split() {
    Command::cargo_bin("tx")
        .unwrap()
        .args(["AGE"])
        .write_stdin(indoc! {r#"
            ID  NAME          AGE
            1   Ivan Ivanov   18
            2   Peter Petrov  23
            3   John Johnson  50
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            18
            23
            50
        "#});
}

#[test]
fn basic_condition() {
    Command::cargo_bin("tx")
        .unwrap()
        .args([r#"NAME if AGE = "18""#])
        .write_stdin(indoc! {r#"
            ID  NAME          AGE
            1   Ivan Ivanov   18
            2   Peter Petrov  23
            3   John Johnson  50
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            Ivan Ivanov
        "#});
}

#[test]
fn condition_or() {
    Command::cargo_bin("tx")
        .unwrap()
        .args([r#"NAME if AGE = "18" or ID = "2""#])
        .write_stdin(indoc! {r#"
            ID  NAME          AGE
            1   Ivan Ivanov   18
            2   Peter Petrov  23
            3   John Johnson  50
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            Ivan Ivanov
            Peter Petrov
        "#});
}

#[test]
fn condition_and() {
    Command::cargo_bin("tx")
        .unwrap()
        .args([r#"NAME if AGE = "23" and ID = "1""#])
        .write_stdin(indoc! {r#"
            ID  NAME          AGE
            1   Ivan Ivanov   18
            2   Peter Petrov  23
            3   John Johnson  50
        "#})
        .assert()
        .success()
        .stdout("");
}

#[test]
fn complex_condition() {
    Command::cargo_bin("tx")
        .unwrap()
        .args([r#"A B C if (A = "0" | A = "1") & (B = "1" | C = "2")"#])
        .write_stdin(indoc! {r#"
            A B C
            0 0 0
            0 0 2
            0 1 1
            1 2 0
            2 0 0
            2 1 2
        "#})
        .assert()
        .success()
        .stdout(indoc! {r#"
            0 0 2
            0 1 1
        "#});
}
