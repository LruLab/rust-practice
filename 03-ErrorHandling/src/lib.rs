//! # エラーハンドリング
//!
//! 以下の関数群は次のような処理を行うための関数群です.
//!
//! - 環境変数からSecretの記述されたファイルのパスを取得する
//!   - ファイルにはユーザIDとパスワードのハッシュ値の組が記述されている
//!   - フォーマットはJSON形式であり, `user`, `hash`をkey, 文字列を値とするオブジェクトの配列が記述されている
//! - ファイルの内容を読み込まれ, 静的な変数として格納される
//! - verify関数は, 引数に与えられた文字列が, ユーザIDとパスワードのハッシュ値の組のいづれかと一致するかを検証する
//!
//! 下記関数群からなる処理には, 途中でエラーとなり処理が即時異常終了する可能性が多分に含まれています.
//! 適切にエラー処理を実行できるようこの関数たちを完成させ, 異常終了する可能性を排除してください.

use base64::engine::general_purpose::STANDARD_NO_PAD as coder;
use base64::Engine;
use bcrypt;
use serde::{Deserialize, Serialize};
use std::{env::VarError, ops::Range, sync::OnceLock};

const KEY: &str = "CREDENTIALS_PATH";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub password_hash: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VerifyError {
    InvalidHeaderFormat,
    LoadFailed,
    UserNotFound,
    PasswordMissMatch,
}

pub fn verify<S: Into<String>>(basic_auth_value: S) -> Result<(), VerifyError> {
    let value = basic_auth_value.into();
    let (id, password) = parse_basic_auth(value);

    let credentials = load_credentials();
    let user = credentials.iter().find(|user| user.id == id).unwrap();

    if bcrypt::verify(password, user.password_hash.as_str()).unwrap() {
        Ok(())
    } else {
        Err(VerifyError::PasswordMissMatch)
    }
}

// Basic認証のAuthorizationヘッダの値を分解する
// "Basic <base64 encoded id:password>"
fn parse_basic_auth(value: String) -> (String, String) {
    let value = value.strip_prefix("Basic ").unwrap();
    let decoded_bytes = coder.decode(value.as_bytes()).unwrap();
    let decoded_value = String::from_utf8_lossy(decoded_bytes.as_slice());
    let (id, password) = decoded_value.split_once(":").unwrap();

    (id.to_string(), password.to_string())
}

// 認証情報を記述したファイルを読み込む
fn load_credentials() -> Vec<User> {
    let path = std::env::var(KEY).unwrap();
    let json = std::fs::read_to_string(path).unwrap();
    let users = serde_json::from_str(json.as_str()).unwrap();

    users
}
