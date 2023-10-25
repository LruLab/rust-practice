use assert_cmd::Command;
use indoc::indoc;

const BINARY_NAME: &str = "fizz-buzz";
const BINARY_LOAD_FAILED: &str = "Failed to load binary... Are you changed the binary name?";

// 入力値について, 「数値」としか言及していないが型をu64としているため, 負の数は受け付けない.
// 0の場合は「1から0まで」のFizzBuzzを出力することになるが, そのような範囲は存在しないため空集合 = 何も出力しなければ可
#[test]
fn test_fizz_buzz_0() {
    let mut cmd = Command::cargo_bin(BINARY_NAME).expect(BINARY_LOAD_FAILED);

    cmd.args(&["0"]);
    cmd.assert().stdout("").success();
}

// 10までのFizzBuzzが出力されれば可 (可変対応確認)
#[test]
fn test_fizz_buzz_10() {
    let mut cmd = Command::cargo_bin(BINARY_NAME).expect(BINARY_LOAD_FAILED);

    let expected = indoc! {r#"
        1
        2
        Fizz
        4
        Buzz
        Fizz
        7
        8
        Fizz
        Buzz
    "#};
    cmd.args(&["10"]);
    cmd.assert().stdout(expected).success();
}

// 15までのFizzBuzzが出力されれば可 (15, 5, 3の処理制御確認)
#[test]
fn test_fizz_buzz_15() {
    let mut cmd = Command::cargo_bin(BINARY_NAME).expect(BINARY_LOAD_FAILED);

    let expected = indoc! {r#"
        1
        2
        Fizz
        4
        Buzz
        Fizz
        7
        8
        Fizz
        Buzz
        11
        Fizz
        13
        14
        FizzBuzz
    "#};
    cmd.args(&["15"]);
    cmd.assert().stdout(expected).success();
}
