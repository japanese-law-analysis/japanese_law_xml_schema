use crate::paragraph::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Remarks {
  label: RemarksLabel,
  children: Vec<RemarksContents>,
}

impl Parser for Remarks {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Remarks" {
      let mut label_opt = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "RemarksLabel" => {
            if let Ok(l) = RemarksLabel::parser(&node) {
              label_opt = Some(l);
            }
          }
          "Item" => {
            if let Ok(item) = Item::parser(&node) {
              children.push(RemarksContents::Item(item));
            }
          }
          "Sentence" => {
            if let Ok(sentence) = Sentence::parser(&node) {
              children.push(RemarksContents::Sentence(sentence));
            }
          }
          _ => {}
        }
      }
      if let Some(label) = label_opt {
        Ok(Remarks { label, children })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct RemarksLabel {
  text: Text,
  line_break: bool,
}

impl Parser for RemarksLabel {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "RemarksLabel" {
      let line_break = node
        .attribute("LineBreak")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      let text = Text::from_children(node.children());
      Ok(RemarksLabel { text, line_break })
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum RemarksContents {
  Item(Item),
  Sentence(Sentence),
}
