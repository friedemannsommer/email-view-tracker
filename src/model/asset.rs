#[derive(Debug)]
pub struct Asset {
    pub contents: &'static [u8],
    pub last_modified: time::OffsetDateTime,
}
