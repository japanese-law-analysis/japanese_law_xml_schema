use crate::result::Result;
use xmltree::Element;

pub(crate) trait Parser {
  fn parser(node: &Element) -> Result<Self>
  where
    Self: Sized;
}
