pub mod access;
pub mod connection;
pub mod migrate;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    Connection(#[from] connection::ConnectError),
    #[error(transparent)]
    Access(#[from] access::AccessError),
}
