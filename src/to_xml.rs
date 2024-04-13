use crate::result::Result;
use roxmltree::Node;

pub(crate) trait ToXml<T> {
  fn to_xml(t: &T) -> Result<Node>
  where
    Self: Sized;
}
