use crate::result::Result;
use roxmltree::Node;

pub(crate) trait Parser {
  fn parser(node: &Node) -> Result<Self>
  where
    Self: Sized;
}
