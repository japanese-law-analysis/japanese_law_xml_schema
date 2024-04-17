//! XML形式を法律構造に変換するトレイト

use crate::result::Result;
use xmltree::Element;

pub(crate) trait Parser {
  fn parser(node: &Element) -> Result<Self>
  where
    Self: Sized;
}
