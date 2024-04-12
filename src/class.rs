use crate::paragraph::*;
use crate::parser::*;
use crate::result::*;
use crate::sentence::*;
use crate::table::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Class {
  pub class_title: Option<Text>,
  pub class_sentence: SentenceOrColumnOrTable,
  pub children: Vec<Item>,
  pub num: String,
}

impl Parser for Class {
  fn parser(node: &Node) -> Result<Self> {
    if node.tag_name().name() == "Class" {
      let num = get_attribute(node, "Num")?;
      let mut class_title = None;
      let mut class_sentence_opt = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ClassTitle" => {
            class_title = Some(Text::from_children(node.children()));
          }
          "ClassSentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            class_sentence_opt = Some(v);
          }
          "Item" => {
            let v = Item::parser(&node)?;
            children.push(v)
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      if let Some(class_sentence) = class_sentence_opt {
        Ok(Class {
          class_title,
          class_sentence,
          children,
          num,
        })
      } else {
        Err(Error::missing_required_tag(
          &node.range(),
          "Sentence or Column",
        ))
      }
    } else {
      Err(Error::wrong_tag_name(node, "Class"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SentenceOrColumnOrTable {
  Sentence(Vec<Sentence>),
  Column(Vec<Column>),
  Table(Table),
}

impl SentenceOrColumnOrTable {
  pub(crate) fn from_node(node: &Node) -> Result<Self> {
    let mut sentence_opt = None;
    let mut sentence_list = Vec::new();
    let mut column_list = Vec::new();
    for node in node.children() {
      match node.tag_name().name() {
        "Sentence" => {
          let v = Sentence::parser(&node)?;
          sentence_list.push(v);
        }
        "Column" => {
          let v = Column::parser(&node)?;
          column_list.push(v);
        }
        "Table" => {
          let v = Table::parser(&node)?;
          sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
        }
        s => return Err(Error::unexpected_tag(&node, s)),
      }
    }
    if !sentence_list.is_empty() {
      sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
    }
    if !column_list.is_empty() {
      sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
    }
    if let Some(sentence) = sentence_opt {
      Ok(sentence)
    } else {
      Err(Error::MissingRequiredTag {
        range: node.range(),
        tag_name: "Sentence or Column or Table".to_string(),
      })
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Caption {
  pub text: Text,
  pub common_caption: Option<bool>,
}

impl Caption {
  fn new(text: text::Text, common_caption: Option<bool>) -> Self {
    Caption {
      text,
      common_caption,
    }
  }
}

impl Parser for Caption {
  fn parser(node: &Node) -> Result<Self> {
    let text = text::Text::from_children(node.children());
    let common_caption = get_attribute_opt_with_parse::<bool>(node, "CommonCaption")?;
    Ok(Caption::new(text, common_caption))
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Column {
  pub sentence: Vec<Sentence>,
  pub num: Option<usize>,
  pub line_break: bool,
  pub align: Option<Align>,
}

impl Parser for Column {
  fn parser(node: &Node) -> Result<Self> {
    if node.tag_name().name() == "Column" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
      let line_break = get_attribute_opt_with_parse(node, "LineBreak")?.unwrap_or(false);
      let align = Align::from_attribute(node.attribute("Align"));
      let mut sentence = Vec::new();
      for node in node.children() {
        let v = Sentence::parser(&node)?;
        sentence.push(v);
      }
      Ok(Column {
        sentence,
        num,
        line_break,
        align,
      })
    } else {
      Err(Error::wrong_tag_name(node, "Column"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum Align {
  Left,
  Center,
  Right,
  Justify,
}

impl Align {
  pub fn from_attribute(att: Option<&str>) -> Option<Align> {
    match att {
      Some("left") => Some(Align::Left),
      Some("center") => Some(Align::Center),
      Some("right") => Some(Align::Right),
      Some("justify") => Some(Align::Justify),
      _ => None,
    }
  }
}
