//! 線
use crate::contents::*;
use crate::parser::*;
use crate::result::*;
use crate::structs::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Line {
  contents: Vec<LineContents>,
  style: LineStyle,
}

impl ToHtml for Line {
  fn to_html(&self) -> String {
    "〔図略〕".to_string()
  }
}

impl Parser for Line {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Line" {
      let style = LineStyle::from_attribute(node.attribute("Style")).ok_or(Error::Attribute)?;
      let mut contents = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "QuoteStruct" => {
            if let Ok(v) = QuoteStruct::parser(&node) {
              contents.push(LineContents::QuoteStruct(v))
            }
          }
          "ArithFormula" => {
            if let Ok(v) = ArithFormula::parser(&node) {
              contents.push(LineContents::ArithFormula(v))
            }
          }
          "Ruby" => {
            if let Ok(v) = Ruby::parser(&node) {
              contents.push(LineContents::Ruby(v))
            }
          }
          "Sup" => {
            if let Ok(v) = Sup::parser(&node) {
              contents.push(LineContents::Sup(v))
            }
          }
          "Sub" => {
            if let Ok(v) = Sub::parser(&node) {
              contents.push(LineContents::Sub(v))
            }
          }
          "" => {
            if let Some(v) = node.text() {
              contents.push(LineContents::String(v.to_string()));
            }
          }
          _ => {}
        }
      }
      Ok(Line { contents, style })
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum LineContents {
  QuoteStruct(QuoteStruct),
  ArithFormula(ArithFormula),
  Ruby(Ruby),
  Sup(Sup),
  Sub(Sub),
  String(String),
}

/// 線の引き方
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum LineStyle {
  Dotted,
  Double,
  Solid,
  None,
}

impl LineStyle {
  pub fn from_attribute(att: Option<&str>) -> Option<Self> {
    match att {
      Some("solid") => Some(LineStyle::Solid),
      Some("none") => Some(LineStyle::None),
      Some("dotted") => Some(LineStyle::Dotted),
      Some("double") => Some(LineStyle::Dotted),
      _ => None,
    }
  }
}
