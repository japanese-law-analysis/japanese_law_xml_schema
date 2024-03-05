//! エラーハンドリング

use std::ops::Range;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone)]
pub enum Error {
  //#[error("")]
  //Tag,
  //#[error("")]
  //Attribute,
  #[error("Missing required attribute of "{attribute_name}" at {tag_name} tag({} - {})", .range.start, .range.end)]
  MissingRequiredAttribute {
    range: Range<usize>,
    tag_name: String,
    attribute_name: String,
  },
  #[error("Missing required tag of "{tag_name}" tag({} - {})", .range.start, .range.end)]
  MissingRequiredTag {
    range: Range<usize>,
    tag_name: String,
  },
  #[error("Attribure parse error of "{attribute_name}" at {tag_name} tag({} - {})", .range.start, .range.end)]
  AttributeParseError {
    range: Range<usize>,
    tag_name: String,
    attribute_name: String,
  },
  #[error("Tag name is {wrong_name} but expected {expected_name} ({} - {})", .range.start, .range.end)]
  WrongTagName {
    range: Range<usize>,
    wrong_name: String,
    expected_name: String,
  },
  #[error("Unexpected {wrong_name} tag at {tag} ({} - {})", .range.start, .range.end)]
  UnexpectedTag {
    range: Range<usize>,
    wrong_name: String,
    tag: String,
  },
  #[error("parsing error({0})")]
  XMLParing(roxmltree::Error),
}

impl Error {
  pub(crate) fn missing_required_tag(range: &Range<usize>, tag_name: &str) -> Self {
    Error::MissingRequiredTag {
      range: range.clone(),
      tag_name: tag_name.to_string(),
    }
  }

  pub(crate) fn wrong_tag_name(node: &roxmltree::Node, expected: &str) -> Self {
    Error::WrongTagName {
      range: node.range().clone(),
      wrong_name: node.tag_name().name().to_string(),
      expected_name: expected.to_string(),
    }
  }

  pub(crate) fn unexpected_tag(node: &roxmltree::Node, wrong: &str) -> Self {
    Error::UnexpectedTag {
      range: node.range().clone(),
      wrong_name: wrong.to_string(),
      tag: node.tag_name().name().to_string(),
    }
  }
}
