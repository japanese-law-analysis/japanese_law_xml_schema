use crate::paragraph::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
use crate::text::*;
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

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum RemarksContents {
  Item(Item),
  Sentence(Sentence),
}
