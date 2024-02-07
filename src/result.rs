use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
  #[error("attribute")]
  Attribute,
  #[error("tag")]
  Tag,
}
