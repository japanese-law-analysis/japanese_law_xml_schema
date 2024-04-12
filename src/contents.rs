use crate::fig::*;
use crate::line::*;
use crate::list::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::sentence::*;
use crate::structs::*;
use crate::table::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

/// AritchFormulaやQuoteStructの中身
/// `any`とあるが、現実的にありえるパターンを列挙する
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contents {
  pub contents: Vec<ContentsElement>,
}

impl parser::Parser for Contents {
  fn parser(node: &roxmltree::Node) -> result::Result<Self> {
    let mut lst = Vec::new();
    for node in node.children() {
      match node.tag_name().name() {
        "Table" => {
          let v = Table::parser(&node)?;
          lst.push(ContentsElement::Table(v));
        }
        "TableStruct" => {
          let v = TableStruct::parser(&node)?;
          lst.push(ContentsElement::TableStruct(v));
        }
        "Fig" => {
          let v = Fig::parser(&node)?;
          lst.push(ContentsElement::Fig(v));
        }
        "FigStruct" => {
          let v = FigStruct::parser(&node)?;
          lst.push(ContentsElement::FigStruct(v));
        }
        "Ruby" => {
          let v = Ruby::parser(&node)?;
          lst.push(ContentsElement::Ruby(v));
        }
        "Line" => {
          let v = Line::parser(&node)?;
          lst.push(ContentsElement::Line(v));
        }
        "Sup" => {
          let v = Sup::parser(&node)?;
          lst.push(ContentsElement::Sup(v));
        }
        "Sub" => {
          let v = Sub::parser(&node)?;
          lst.push(ContentsElement::Sub(v));
        }
        "Paragraph" => {
          let v = Paragraph::parser(&node)?;
          lst.push(ContentsElement::Paragraph(v));
        }
        "List" => {
          let v = List::parser(&node)?;
          lst.push(ContentsElement::List(v));
        }
        "Sentence" => {
          let v = Sentence::parser(&node)?;
          lst.push(ContentsElement::Sentence(v));
        }
        "" => {
          if let Some(text) = node.text() {
            lst.push(ContentsElement::String(text.to_string()));
          }
        }
        s => return Err(Error::unexpected_tag(&node, s)),
      }
    }
    Ok(Contents { contents: lst })
  }
}

/// Contentsの中身
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Style {
  pub contentes: Contents,
}

impl Parser for Style {
  fn parser(node: &Node) -> result::Result<Self> {
    Contents::parser(node).map(|c| Style { contentes: c })
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Note {
  pub contentes: Contents,
}

impl Parser for Note {
  fn parser(node: &Node) -> result::Result<Self> {
    Contents::parser(node).map(|c| Note { contentes: c })
  }
}

/// 様式
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Format {
  pub contentes: Contents,
}

impl Parser for Format {
  fn parser(node: &Node) -> result::Result<Self> {
    Contents::parser(node).map(|c| Format { contentes: c })
  }
}

/// 数式
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArithFormula {
  pub num: Option<usize>,
  pub contentes: Contents,
}

impl Parser for ArithFormula {
  fn parser(node: &Node) -> result::Result<Self> {
    let num = get_attribute_opt_with_parse(node, "Num")?;
    Contents::parser(node).map(|c| ArithFormula { num, contentes: c })
  }
}
