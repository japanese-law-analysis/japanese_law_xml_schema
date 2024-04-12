//! 画像

use crate::parser::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Fig {
  pub src: String,
}

impl Parser for Fig {
  fn parser(node: &Node) -> result::Result<Self> {
    let src = get_attribute(node, "src")?;
    Ok(Fig { src })
  }
}
