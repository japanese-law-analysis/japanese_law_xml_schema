//! エラーハンドリング

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
  //#[error("")]
  //Tag,
  //#[error("")]
  //Attribute,
  #[error("Missing required attribute of {attribute_name} at {tag_name} tag")]
  MissingRequiredAttribute {
    tag_name: String,
    attribute_name: String,
  },
  #[error("Missing required tag of {tag_name} tag")]
  MissingRequiredTag { tag_name: String },
  #[error("Attribure parse error of {attribute_name} at {tag_name} tag")]
  AttributeParseError {
    tag_name: String,
    attribute_name: String,
  },
  #[error("Tag name is {wrong_name} but expected {expected_name}")]
  WrongTagName {
    wrong_name: String,
    expected_name: String,
  },
  #[error("Unexpected {wrong_name} tag at {tag}")]
  UnexpectedTag { wrong_name: String, tag: String },
  #[error("xml parsing error")]
  XMLParsing,
  #[error("{0} parsing error: {1}")]
  ParsingError(String, String),
  #[error("write error")]
  Io,
}

impl Error {
  pub(crate) fn missing_required_tag(tag_name: &str) -> Self {
    Error::MissingRequiredTag {
      tag_name: tag_name.to_string(),
    }
  }

  pub(crate) fn wrong_tag_name(element: &xmltree::Element, expected: &str) -> Self {
    Error::WrongTagName {
      wrong_name: element.name.to_string(),
      expected_name: expected.to_string(),
    }
  }

  pub(crate) fn unexpected_tag(element: &xmltree::Element, wrong: &str) -> Self {
    Error::UnexpectedTag {
      wrong_name: wrong.to_string(),
      tag: element.name.to_string(),
    }
  }
}
