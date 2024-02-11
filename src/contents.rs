use crate::fig::*;
use crate::parser::*;
use crate::structs::*;
use crate::table::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

/// AritchFormulaやQuoteStructの中身
/// `any`とあるが、現実的にありえるパターンを列挙する
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Contents {
  contents: Vec<ContentsElement>,
}

impl parser::Parser for Contents {
  fn parser(node: &roxmltree::Node) -> result::Result<Self> {
    let mut lst = Vec::new();
    for node in node.children() {
      match node.tag_name().name() {
        "Table" => {
          if let Ok(v) = Table::parser(&node) {
            lst.push(ContentsElement::Table(v));
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            lst.push(ContentsElement::TableStruct(v));
          }
        }
        "Fig" => {
          if let Ok(v) = Fig::parser(&node) {
            lst.push(ContentsElement::Fig(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            lst.push(ContentsElement::FigStruct(v));
          }
        }
        "Ruby" => {
          if let Ok(v) = text::Ruby::parser(&node) {
            lst.push(ContentsElement::Ruby(v));
          }
        }
        "Line" => {
          if let Ok(v) = line::Line::parser(&node) {
            lst.push(ContentsElement::Line(v));
          }
        }
        "Sup" => {
          if let Ok(v) = text::Sup::parser(&node) {
            lst.push(ContentsElement::Sup(v));
          }
        }
        "Sub" => {
          if let Ok(v) = text::Sub::parser(&node) {
            lst.push(ContentsElement::Sub(v));
          }
        }
        "Paragraph" => {
          if let Ok(v) = paragraph::Paragraph::parser(&node) {
            lst.push(ContentsElement::Paragraph(v));
          }
        }
        "List" => {
          if let Ok(v) = list::List::parser(&node) {
            lst.push(ContentsElement::List(v));
          }
        }
        "Sentence" => {
          if let Ok(v) = sentence::Sentence::parser(&node) {
            lst.push(ContentsElement::Sentence(v));
          }
        }
        "" => {
          if let Some(text) = node.text() {
            lst.push(ContentsElement::String(text.to_string()));
          }
        }
        _ => return Err(result::Error::Tag),
      }
    }
    Ok(Contents { contents: lst })
  }
}

/// Contentsの中身
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum ContentsElement {
  Table(Table),
  TableStruct(TableStruct),
  Fig(Fig),
  FigStruct(FigStruct),
  Ruby(text::Ruby),
  Line(line::Line),
  Sup(text::Sup),
  Sub(text::Sub),
  String(String),
  Paragraph(paragraph::Paragraph),
  List(list::List),
  Sentence(sentence::Sentence),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Style {
  contentes: Contents,
}

impl Parser for Style {
  fn parser(node: &Node) -> result::Result<Self> {
    Contents::parser(node).map(|c| Style { contentes: c })
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Note {
  contentes: Contents,
}

impl Parser for Note {
  fn parser(node: &Node) -> result::Result<Self> {
    Contents::parser(node).map(|c| Note { contentes: c })
  }
}

/// 様式
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Format {
  contentes: Contents,
}

impl Parser for Format {
  fn parser(node: &Node) -> result::Result<Self> {
    Contents::parser(node).map(|c| Format { contentes: c })
  }
}

/// 数式
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ArithFormula {
  num: Option<usize>,
  contentes: Contents,
}

impl Parser for ArithFormula {
  fn parser(node: &Node) -> result::Result<Self> {
    let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
    Contents::parser(node).map(|c| ArithFormula { num, contentes: c })
  }
}
