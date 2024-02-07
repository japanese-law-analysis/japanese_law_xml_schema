//! 線

use crate::parser::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Line {
  /// TODO
  /// contents: any,
  style: LineStyle,
}

impl ToHtml for Line {
  fn to_html(&self) -> String {
    format!("〔図略〕")
  }
}

impl Parser for Line {
  fn parser(node: &Node) -> result::Result<Self> {
    todo!()
  }
}

/// 線の引き方
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum LineStyle {
  Dotted,
  Double,
  Solid,
  None,
}
