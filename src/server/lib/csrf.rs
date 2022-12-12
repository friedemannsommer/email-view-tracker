#[derive(thiserror::Error, Debug)]
pub enum CsrfError {
    #[error(transparent)]
    Random(#[from] getrandom::Error),
}

pub fn generate_token() -> Result<String, CsrfError> {
    let mut buffer = [0u8; 32];

    getrandom::getrandom(&mut buffer)?;

    Ok(hex::encode(buffer))
}
