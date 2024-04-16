use crate::paragraph::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Remarks {
  pub label: RemarksLabel,
  pub children: Vec<RemarksContents>,
}

impl Parser for Remarks {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Remarks" {
      let mut label_opt = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "RemarksLabel" => {
              let v = RemarksLabel::parser(e)?;
              label_opt = Some(v);
            }
            "Item" => {
              let v = Item::parser(e)?;
              children.push(RemarksContents::Item(v));
            }
            "Sentence" => {
              let v = Sentence::parser(e)?;
              children.push(RemarksContents::Sentence(v));
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(label) = label_opt {
        Ok(Remarks { label, children })
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "RemarksLabel".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Remarks"))
    }
  }
}

impl ToXmlElement for Remarks {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Remarks");
    for n in self.children.iter() {
      match n {
        RemarksContents::Item(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        RemarksContents::Sentence(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
      }
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemarksLabel {
  pub text: Text,
  pub line_break: bool,
}

impl Parser for RemarksLabel {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "RemarksLabel" {
      let line_break = get_attribute_opt_with_parse(element, "LineBreak")?.unwrap_or(false);
      let text = Text::from_children(&element.children);
      Ok(RemarksLabel { text, line_break })
    } else {
      Err(Error::wrong_tag_name(element, "RemarksLabel"))
    }
  }
}

impl ToXmlElement for RemarksLabel {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("RemarksLabel");
    if self.line_break {
      e.attributes
        .insert("LineBreak".to_string(), "true".to_string());
    }
    e.children = self.text.to_children();
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum RemarksContents {
  Item(Item),
  Sentence(Sentence),
}
