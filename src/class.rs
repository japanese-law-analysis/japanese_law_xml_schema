//! classやcaptionなど
//!

use crate::paragraph::*;
use crate::parser::*;
use crate::result::*;
use crate::sentence::*;
use crate::table::*;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Class {
  pub class_title: Option<Text>,
  pub class_sentence: SentenceOrColumnOrTable,
  pub children: Vec<Item>,
  pub num: String,
}

impl Parser for Class {
  fn parser(element: &Element) -> Result<Self> {
    if element.name.as_str() == "Class" {
      let num = get_attribute(element, "Num")?;
      let mut class_title = None;
      let mut class_sentence_opt = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ClassTitle" => {
              class_title = Some(Text::from_children(&e.children));
            }
            "ClassSentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              class_sentence_opt = Some(v);
            }
            "Item" => {
              let v = Item::parser(e)?;
              children.push(v)
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
        Err(Error::missing_required_tag("Sentence or Column"))
      }
    } else {
      Err(Error::wrong_tag_name(element, "Class"))
    }
  }
}

impl ToXmlElement for Class {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Class");
    if let Some(title) = &self.class_title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("ClassTitle"),
      ))
    }
    e.children.push(XMLNode::Element(
      self
        .class_sentence
        .to_xml_element_with_name("ClassSentence"),
    ));
    for item in self.children.iter() {
      e.children.push(XMLNode::Element(item.to_xml_element()));
    }
    e.attributes.insert("Num".to_string(), self.num.to_string());
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SentenceOrColumnOrTable {
  Sentence(Vec<Sentence>),
  Column(Vec<Column>),
  Table(Table),
}

impl ToXmlElementWithName for SentenceOrColumnOrTable {
  fn to_xml_element_with_name(&self, name: &str) -> Element {
    let mut e = Element::new(name);
    match self {
      SentenceOrColumnOrTable::Column(columns) => {
        for c in columns.iter() {
          e.children.push(XMLNode::Element(c.to_xml_element()))
        }
      }
      SentenceOrColumnOrTable::Sentence(sentence) => {
        for s in sentence.iter() {
          e.children.push(XMLNode::Element(s.to_xml_element()))
        }
      }
      SentenceOrColumnOrTable::Table(table) => {
        e.children.push(XMLNode::Element(table.to_xml_element()))
      }
    }
    e
  }
}

impl SentenceOrColumnOrTable {
  pub(crate) fn from_element(element: &Element) -> Result<Self> {
    let mut sentence_opt = None;
    let mut sentence_list = Vec::new();
    let mut column_list = Vec::new();
    for node in element.children.iter() {
      if let XMLNode::Element(e) = node {
        match e.name.as_str() {
          "Sentence" => {
            let v = Sentence::parser(e)?;
            sentence_list.push(v);
          }
          "Column" => {
            let v = Column::parser(e)?;
            column_list.push(v);
          }
          "Table" => {
            let v = Table::parser(e)?;
            sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
          }
          s => return Err(Error::unexpected_tag(e, s)),
        }
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
  fn parser(element: &Element) -> Result<Self> {
    let text = text::Text::from_children(&element.children);
    let common_caption = get_attribute_opt_with_parse::<bool>(element, "CommonCaption")?;
    Ok(Caption::new(text, common_caption))
  }
}

impl ToXmlElementWithName for Caption {
  fn to_xml_element_with_name(&self, name: &str) -> Element {
    let mut e = Element::new(name);
    e.children = self.text.to_children();
    if let Some(b) = self.common_caption {
      e.attributes
        .insert("CommonCaption".to_string(), b.to_string());
    }
    e
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
  fn parser(element: &Element) -> Result<Self> {
    if element.name.as_str() == "Column" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let line_break = get_attribute_opt_with_parse(element, "LineBreak")?.unwrap_or(false);
      let align = Align::from_attribute(element.attributes.get("Align"));
      let mut sentence = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          let v = Sentence::parser(e)?;
          sentence.push(v);
        }
      }
      Ok(Column {
        sentence,
        num,
        line_break,
        align,
      })
    } else {
      Err(Error::wrong_tag_name(element, "Column"))
    }
  }
}

impl ToXmlElement for Column {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Column");
    for n in self.sentence.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    if let Some(n) = self.num {
      e.attributes.insert("Num".to_string(), n.to_string());
    }
    e.attributes
      .insert("LineBreak".to_string(), self.line_break.to_string());
    if let Some(a) = &self.align {
      e.attributes.insert("Align".to_string(), a.to_attribute());
    }
    e
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
  pub fn from_attribute(att: Option<&String>) -> Option<Align> {
    match att.map(|s| s.as_str()) {
      Some("left") => Some(Align::Left),
      Some("center") => Some(Align::Center),
      Some("right") => Some(Align::Right),
      Some("justify") => Some(Align::Justify),
      _ => None,
    }
  }

  pub fn to_attribute(&self) -> String {
    match self {
      Self::Center => "center".to_string(),
      Self::Right => "right".to_string(),
      Self::Left => "left".to_string(),
      Self::Justify => "justify".to_string(),
    }
  }
}
