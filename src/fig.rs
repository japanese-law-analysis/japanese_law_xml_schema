//! 画像

use std::collections::HashMap;

use crate::parser::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::Element;

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Fig {
  pub src: String,
}

impl Parser for Fig {
  fn parser(element: &Element) -> result::Result<Self> {
    let src = element
      .attributes
      .get("src")
      .ok_or(result::Error::MissingRequiredAttribute {
        tag_name: "Fig".to_string(),
        attribute_name: "src".to_string(),
      })?
      .clone();
    Ok(Fig { src })
  }
}

impl ToXmlElement for Fig {
  fn to_xml_element(&self) -> Result<Element> {
    let e = Element::new("Fig");
    let mut att = HashMap::new();
    att.insert("src".to_string(), self.src.clone());
    Ok(Element {
      attributes: att,
      ..e
    })
  }
}
