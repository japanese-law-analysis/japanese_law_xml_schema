use crate::result::Result;
use xmltree::Element;

pub(crate) trait ToXmlElement {
  fn to_xml_element(&self) -> Result<Element>
  where
    Self: Sized;
}

pub(crate) trait ToXmlElementWithName {
  fn to_xml_element_with_name(&self, name: &str) -> Result<Element>
  where
    Self: Sized;
}
