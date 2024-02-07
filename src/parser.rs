use crate::result::Result;
use roxmltree::Node;

pub trait Parser {
  fn parser(node: &Node) -> Result<Self>
  where
    Self: Sized;
}
