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

// thiserror crateを利用することで, Error Traitを自動実装した上で, Attribute情報でDisplay Traitを実装することが可能.
// 更に変換対象が明確かつ実装しているTraitが条件を満たしている場合は, From Attributeを利用することでFromの変換も自動実装可能.
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

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum ParseError {
    #[error("Value is not Basic Header")]
    NotBasicHeader,

    #[error("Encoding is not base64")]
    InvalidEncoding(#[from] base64::DecodeError),

    #[error("Invalid format")]
    InvalidFormat,
}

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum LoadError {
    #[error("Environment variable is missing")]
    EnvironmentMissing(#[from] std::env::VarError),

    #[error("File is not readable")]
    NotReadable,

    #[error("Invalid format")]
    InvalidFormat,
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
        Ok(false) => Err(VerifyError::PasswordMissMatch),
        Err(_) => Err(VerifyError::PasswordMissMatch),
    }
}

// Basic認証のAuthorizationヘッダの値を分解する
// "Basic <base64 encoded id:password>"
fn parse_basic_auth(value: String) -> Result<(String, String), ParseError> {
    let value = value
        .strip_prefix("Basic ")
        .ok_or(ParseError::NotBasicHeader)?;
    let decoded_bytes = coder.decode(value.as_bytes())?;
    let decoded_value = String::from_utf8_lossy(decoded_bytes.as_slice());
    let (id, password) = decoded_value
        .split_once(":")
        .ok_or(ParseError::InvalidFormat)?;

    Ok((id.to_string(), password.to_string()))
}

// 認証情報を記述したファイルを読み込む
fn load_credentials() -> Result<Vec<User>, LoadError> {
    let path = std::env::var(KEY)?;
    let json = std::fs::read_to_string(path).map_err(|_| LoadError::NotReadable)?;
    let users = serde_json::from_str(json.as_str()).map_err(|_| LoadError::InvalidFormat)?;

    Ok(users)
}

fn main() {}
