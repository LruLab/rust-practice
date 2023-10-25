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

// anyhow crateを利用することで, 「1つの関数で生じうるすべてのエラーを1つに集約する」ということが可能.
// anyhow::Resultは, std::error::Errorを実装する任意のエラーを受け取ることができるResult.
// また, Contextを利用することで, Option型のような型に対してもunwrapの代わりにResult型でラップすることが可能.
// この程度の浅い関数でのエラー伝搬程度であれば, anyhow::Resultを利用することで十分なエラーハンドリングできる.
use anyhow::{anyhow, Context};

// thiserror crateを利用することで, Error Traitを自動実装した上で, Attribute情報でDisplay Traitを実装することが可能.
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum VerifyError {
    #[error("Invalid Basic Auth Header format")]
    InvalidHeaderFormat,

    #[error("Failed to load credentials")]
    LoadFailed,

    #[error("User not found")]
    UserNotFound,

    #[error("Password miss match")]
    PasswordMissMatch,
}

pub fn verify<S: Into<String>>(basic_auth_value: S) -> Result<(), VerifyError> {
    let value = basic_auth_value.into();
    let (id, password) = parse_basic_auth(value).map_err(|_| VerifyError::InvalidHeaderFormat)?;

    let credentials = load_credentials().map_err(|_| VerifyError::LoadFailed)?;
    let user = credentials
        .iter()
        .find(|user| user.id == id)
        .ok_or(VerifyError::UserNotFound)?;

    match bcrypt::verify(password, user.password_hash.as_str()) {
        Ok(true) => Ok(()),
        _ => Err(VerifyError::PasswordMissMatch),
    }
}

// Basic認証のAuthorizationヘッダの値を分解する
// "Basic <base64 encoded id:password>"
fn parse_basic_auth(value: String) -> anyhow::Result<(String, String)> {
    let value = value
        .strip_prefix("Basic ")
        .context("Value is not Basic Header")?;
    let decoded_bytes = coder.decode(value.as_bytes())?;
    let decoded_value = String::from_utf8_lossy(decoded_bytes.as_slice());
    let (id, password) = decoded_value.split_once(":").context("Invalid Format")?;

    Ok((id.to_string(), password.to_string()))
}

// 認証情報を記述したファイルを読み込む
fn load_credentials() -> anyhow::Result<Vec<User>> {
    let path = std::env::var(KEY)?;
    let json = std::fs::read_to_string(path)?;
    let users = serde_json::from_str(json.as_str())?;

    Ok(users)
}

fn main() {}
