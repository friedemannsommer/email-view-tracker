pub fn generate_token() -> Result<String, getrandom::Error> {
    let mut buffer = [0u8; 32];

    getrandom::fill(&mut buffer)?;

    Ok(hex::encode(buffer))
}
