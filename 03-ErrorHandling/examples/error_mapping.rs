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

// 今回は簡易化のためErrorはEnumのみで値を含まないが, 本来は値として伝搬元のエラーを含めるべき
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VerifyError {
    InvalidHeaderFormat,
    LoadFailed,
    UserNotFound,
    PasswordMissMatch,
}

// エラーの発生する可能性のある箇所をResult型でラップすることで, エラーを返すようにする.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    NotBasicHeader,
    InvalidEncoding,
    InvalidFormat,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoadError {
    EnvironmentMissing,
    NotReadable,
    InvalidFormat,
}

pub fn verify<S: Into<String>>(basic_auth_value: S) -> Result<(), VerifyError> {
    let value = basic_auth_value.into();
    let (id, password) = parse_basic_auth(value)?;

    let credentials = load_credentials()?;
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
    let json = std::fs::read_to_string(path)?;
    let users = serde_json::from_str(json.as_str())?;

    Ok(users)
}

// -- Error Handling in verify

impl From<ParseError> for VerifyError {
    fn from(error: ParseError) -> Self {
      Self::InvalidHeaderFormat
    }
}

impl From<LoadError> for VerifyError {
    fn from(error: LoadError) -> Self {
      Self::LoadFailed
    }
}

// -- Error Handling in Parsing Basic Auth Header

impl From<base64::DecodeError> for ParseError {
    fn from(_: base64::DecodeError) -> Self {
        Self::InvalidEncoding
    }
}

// -- Error Handling in Loading Credentials

impl From<std::env::VarError> for LoadError {
    fn from(error: VarError) -> Self {
        Self::EnvironmentMissing
    }
}

impl From<std::io::Error> for LoadError {
    fn from(error: std::io::Error) -> Self {
        Self::NotReadable
    }
}

impl From<serde_json::error::Error> for LoadError {
    fn from(_: serde_json::error::Error) -> Self {
        Self::InvalidFormat
    }
}

// -- Error Implementation

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::InvalidHeaderFormat => "Invalid Basic Auth Header format",
            Self::LoadFailed => "Failed to load credentials",
            Self::UserNotFound => "User not found",
            Self::PasswordMissMatch => "Password miss match",
        };

        write!(f, "{}", message)
    }
}
impl std::error::Error for VerifyError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::NotBasicHeader => "Value is not Basic header",
            Self::InvalidEncoding => "Encoding is not base64",
            Self::InvalidFormat => "Invalid format",
        };

        write!(f, "{}", message)
    }
}
impl std::error::Error for ParseError {}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EnvironmentMissing => "Environment variable is missing",
            Self::NotReadable => "File is not readable",
            Self::InvalidFormat => "Invalid format",
        };

        write!(f, "{}", message)
    }
}
impl std::error::Error for LoadError {}

fn main() {}
