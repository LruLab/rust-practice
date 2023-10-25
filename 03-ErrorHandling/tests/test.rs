use anyhow::{ensure, Result};
use base64::{engine::general_purpose::STANDARD_NO_PAD as coder, Engine};
use bcrypt::DEFAULT_COST;
use std::{
    fs,
    path::{self, Path, PathBuf},
    error::Error,
};

use error_handling::{verify, User, VerifyError};
use serial_test::serial;

// VALIDなユーザIDとパスワードの中身
const USER_ID: &str = "sample_user";
const PASSWORD: &str = "sample_password";

// 環境変数キー
const KEY: &str = "CREDENTIALS_PATH";

#[test]
#[serial]
// 正常ケース
fn test_verify() {
    set_credentials("credentials.json");

    assert!(verify(basic_value(USER_ID, PASSWORD)).is_ok());

    clear_credentials();
}

#[test]
#[serial]
// Basic Authorizationヘッダではない形式の場合
fn test_not_basic() {
    set_credentials("credentials.json");

    let result = verify("invalid header format");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), VerifyError::InvalidHeaderFormat);

    clear_credentials();
}

#[test]
#[serial]
// Basic Authorizationヘッダの値がBase64でない場合
fn test_invalid_header_encoding() {
    set_credentials("credentials.json");

    let result = verify("Basic invalid encoding");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), VerifyError::InvalidHeaderFormat);

    clear_credentials();
}

#[test]
#[serial]
// Basic Authorizationヘッダの値が期待された形式でない場合
fn test_invalid_header_format() {
    set_credentials("credentials.json");

    let value = coder.encode(format!("Basic {}~{}", USER_ID, PASSWORD));
    let result = verify(value);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), VerifyError::InvalidHeaderFormat);

    clear_credentials();
}

#[test]
#[serial]
// 認証情報ファイルが読み込めない場合
fn test_missing_environment() {
    let result = verify(basic_value(USER_ID, PASSWORD));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), VerifyError::LoadFailed);

    clear_credentials();
}

#[test]
#[serial]
// 認証情報ファイルが期待された形式でない場合
fn test_invalid_format() {
    set_credentials("invalid_credentials.json");

    let result = verify(basic_value(USER_ID, PASSWORD));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), VerifyError::LoadFailed);

    clear_credentials();
}

#[test]
#[serial]
// 指定されたユーザIDが存在しない場合
fn test_user_not_found() {
    set_credentials("credentials.json");

    let result = verify(basic_value("invalid_user", PASSWORD));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), VerifyError::UserNotFound);

    clear_credentials();
}

#[test]
#[serial]
// 指定されたパスワードが間違っている場合
fn test_password_miss_match() {
    set_credentials("credentials.json");

    let result = verify(basic_value(USER_ID, "invalid_password"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), VerifyError::PasswordMissMatch);

    clear_credentials();
}

// -- Utilities

// テスト用の認証情報を記述したディレクトリ
fn fixtures_path() -> PathBuf {
    let manifest_path = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_path).join("fixtures")
}

fn set_credentials<S: Into<String>>(path: S) {
    std::env::set_var(KEY, fixtures_path().join(path.into()).as_os_str());
}

fn clear_credentials() {
    std::env::remove_var(KEY);
}

// テスト用の認証情報をエンコードした文字列に変換する
fn basic_value<S1: Into<String>, S2: Into<String>>(id: S1, pass: S2) -> String {
    format!(
        "Basic {}",
        coder.encode(format!("{}:{}", id.into(), pass.into()))
    )
}
