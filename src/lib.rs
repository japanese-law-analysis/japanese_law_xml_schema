//! [法令標準XMLスキーマ](https://elaws.e-gov.go.jp/file/XMLSchemaForJapaneseLaw_v3.xsd)で定義された規格に基づいてXMLとの相互変換を行う
//!
//!
#![recursion_limit = "256"]
use serde::{Deserialize, Serialize};

pub mod appdx;
pub mod article;
pub mod contents;
pub mod fig;
pub mod law;
pub mod line;
pub mod list;
pub mod paragraph;
pub mod parser;
pub mod remarks;
pub mod result;
pub mod sentence;
pub mod structs;
pub mod suppl_provision;
pub mod table;
pub mod table_of_contents;
pub mod text;

/// HTMLを生成できる構造であることを保証するトレイト
pub trait ToHtml {
  /// HTML文字列を生成する関数
  fn to_html(&self) -> String;
}

/// 意味のあるテキストに変換できることを保証するトレイト
pub trait ToText {
  /// 変換する関数
  fn to_text(&self) -> String;
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Class {
  class_title: Option<text::Text>,
  class_sentence: SentenceOrColumnOrTable,
  children: Vec<paragraph::Item>,
  num: String,
}

impl parser::Parser for Class {
  fn parser(node: &roxmltree::Node) -> result::Result<Self> {
    if node.tag_name().name() == "Class" {
      let num = node
        .attribute("Num")
        .ok_or(result::Error::Attribute)?
        .to_string();
      let mut class_title = None;
      let mut class_sentence_opt = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ClassTitle" => {
            class_title = Some(text::Text::from_children(node.children()));
          }
          "ClassSentence" => {
            let mut sentence_list = Vec::new();
            let mut column_list = Vec::new();
            for node in node.children() {
              match node.tag_name().name() {
                "Sentence" => {
                  let v = sentence::Sentence::parser(&node)?;
                  sentence_list.push(v);
                }
                "Column" => {
                  let v = Column::parser(&node)?;
                  column_list.push(v);
                }
                "Table" => {
                  let v = table::Table::parser(&node)?;
                  class_sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
                _ => {}
              }
              if !sentence_list.is_empty() {
                class_sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
              }
              if !column_list.is_empty() {
                class_sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
              }
            }
          }
          "Item" => {
            let v = paragraph::Item::parser(&node)?;
            children.push(v)
          }
          _ => {}
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
        Err(result::Error::Tag)
      }
    } else {
      Err(result::Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SentenceOrColumnOrTable {
  Sentence(Vec<sentence::Sentence>),
  Column(Vec<Column>),
  Table(table::Table),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Caption {
  text: text::Text,
  common_caption: Option<bool>,
}

impl Caption {
  pub fn new(text: text::Text, common_caption: Option<bool>) -> Self {
    Caption {
      text,
      common_caption,
    }
  }
}

impl parser::Parser for Caption {
  fn parser(node: &roxmltree::Node) -> result::Result<Self> {
    let text = text::Text::from_children(node.children());
    let common_caption = node
      .attribute("CommonCaption")
      .and_then(|s| s.parse::<bool>().ok());
    Ok(Caption::new(text, common_caption))
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Column {
  sentence: Vec<sentence::Sentence>,
  num: Option<usize>,
  line_break: bool,
  align: Option<Align>,
}

impl parser::Parser for Column {
  fn parser(node: &roxmltree::Node) -> result::Result<Self> {
    if node.tag_name().name() == "Column" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let line_break = node
        .attribute("LineBreak")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      let align = Align::from_attribute(node.attribute("Align"));
      let mut sentence = Vec::new();
      for node in node.children() {
        if let Ok(v) = sentence::Sentence::parser(&node) {
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
      Err(result::Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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
