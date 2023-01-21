#[derive(Debug)]
pub struct Asset {
    pub name: &'static str,
    pub contents: &'static [u8],
    pub last_modified: time::OffsetDateTime,
}
