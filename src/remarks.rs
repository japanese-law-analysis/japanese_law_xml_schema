use crate::paragraph::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Remarks {
  pub label: RemarksLabel,
  pub children: Vec<RemarksContents>,
}

impl Parser for Remarks {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Remarks" {
      let mut label_opt = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "RemarksLabel" => {
            let v = RemarksLabel::parser(&node)?;
            label_opt = Some(v);
          }
          "Item" => {
            let v = Item::parser(&node)?;
            children.push(RemarksContents::Item(v));
          }
          "Sentence" => {
            let v = Sentence::parser(&node)?;
            children.push(RemarksContents::Sentence(v));
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      if let Some(label) = label_opt {
        Ok(Remarks { label, children })
      } else {
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "RemarksLabel".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Remarks"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemarksLabel {
  pub text: Text,
  pub line_break: bool,
}

impl Parser for RemarksLabel {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "RemarksLabel" {
      let line_break = get_attribute_opt_with_parse(node, "LineBreak")?.unwrap_or(false);
      let text = Text::from_children(node.children());
      Ok(RemarksLabel { text, line_break })
    } else {
      Err(Error::wrong_tag_name(node, "RemarksLabel"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum RemarksContents {
  Item(Item),
  Sentence(Sentence),
}
