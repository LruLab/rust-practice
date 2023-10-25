#[derive(Debug)]
struct MyError;

impl From<std::env::VarError> for MyError {
    fn from(_: std::env::VarError) -> Self {
        MyError
    }
}

fn main() -> Result<(), MyError> {
    let path = std::env::var("CREDENTIALS_PATH")?;
    println!("{}", path);

    Ok(())
}
