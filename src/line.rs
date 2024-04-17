//! 線
use crate::contents::*;
use crate::parser::*;
use crate::result::*;
use crate::structs::*;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Line {
  pub contents: Vec<LineContents>,
  pub style: LineStyle,
}

impl Parser for Line {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Line" {
      let style = LineStyle::from_attribute(element.attributes.get("Style")).ok_or(
        Error::AttributeParseError {
          tag_name: "Line".to_string(),
          attribute_name: "Style".to_string(),
        },
      )?;
      let mut contents = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "QuoteStruct" => {
              let v = QuoteStruct::parser(e)?;
              contents.push(LineContents::QuoteStruct(v))
            }
            "ArithFormula" => {
              let v = ArithFormula::parser(e)?;
              contents.push(LineContents::ArithFormula(v))
            }
            "Ruby" => {
              let v = Ruby::parser(e)?;
              contents.push(LineContents::Ruby(v))
            }
            "Sup" => {
              let v = Sup::parser(e)?;
              contents.push(LineContents::Sup(v))
            }
            "Sub" => {
              let v = Sub::parser(e)?;
              contents.push(LineContents::Sub(v))
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        } else if let XMLNode::Text(s) = node {
          contents.push(LineContents::String(s.clone()))
        }
      }
      Ok(Line { contents, style })
    } else {
      Err(Error::wrong_tag_name(element, "Line"))
    }
  }
}

impl ToXmlElement for Line {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Line");
    e.attributes
      .insert("Style".to_string(), self.style.to_attribute());
    for n in self.contents.iter() {
      match n {
        LineContents::ArithFormula(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        LineContents::QuoteStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        LineContents::Ruby(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        LineContents::Sub(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        LineContents::Sup(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        LineContents::String(s) => e.children.push(XMLNode::Text(s.clone())),
      }
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum LineContents {
  QuoteStruct(QuoteStruct),
  ArithFormula(ArithFormula),
  Ruby(Ruby),
  Sup(Sup),
  Sub(Sub),
  String(String),
}

/// 線の引き方
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum LineStyle {
  Dotted,
  Double,
  Solid,
  None,
}

impl LineStyle {
  pub fn from_attribute(att: Option<&String>) -> Option<Self> {
    match att.map(|s| s.as_str()) {
      Some("solid") => Some(LineStyle::Solid),
      Some("none") => Some(LineStyle::None),
      Some("dotted") => Some(LineStyle::Dotted),
      Some("double") => Some(LineStyle::Dotted),
      _ => None,
    }
  }

  pub fn to_attribute(&self) -> String {
    match self {
      LineStyle::Dotted => "dotted".to_string(),
      LineStyle::Solid => "solid".to_string(),
      LineStyle::Double => "double".to_string(),
      LineStyle::None => "none".to_string(),
    }
  }
}
