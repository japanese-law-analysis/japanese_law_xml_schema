use xmltree::Element;

pub(crate) trait ToXmlElement {
  fn to_xml_element(&self) -> Element;
}

pub(crate) trait ToXmlElementWithName {
  fn to_xml_element_with_name(&self, name: &str) -> Element;
}
