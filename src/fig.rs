use crate::parser::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Fig {
  src: String,
}

impl Parser for Fig {
  fn parser(node: &Node) -> result::Result<Self> {
    let src = node.attribute("src").ok_or(result::Error::Attribute)?;
    Ok(Fig {
      src: src.to_string(),
    })
  }
}
