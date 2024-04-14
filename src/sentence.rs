//! 節

use crate::parser::*;
use crate::result::Error;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sentence {
  pub contents: Vec<SentenceElement>,
  pub num: usize,
  pub function: Option<SentenceFunction>,
  pub indent: Option<SentenceIndent>,
  pub writing_mode: text::WritingMode,
}

impl Parser for Sentence {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Sentence" {
      let num = get_attribute_with_parse(element, "Num")?;
      let function = match element.attributes.get("Function").map(|s| s.as_str()) {
        Some("main") => Some(SentenceFunction::Main),
        Some("proviso") => Some(SentenceFunction::Proviso),
        None => None,
        _ => {
          return Err(Error::AttributeParseError {
            tag_name: element.name.clone(),
            attribute_name: "Function".to_string(),
          })
        }
      };
      let indent = match element.attributes.get("Indent").map(|s| s.as_str()) {
        Some("Paragraph") => Some(SentenceIndent::Paragraph),
        Some("Item") => Some(SentenceIndent::Item),
        Some("Subitem1") => Some(SentenceIndent::Subitem1),
        Some("Subitem2") => Some(SentenceIndent::Subitem2),
        Some("Subitem3") => Some(SentenceIndent::Subitem3),
        Some("Subitem4") => Some(SentenceIndent::Subitem4),
        Some("Subitem5") => Some(SentenceIndent::Subitem5),
        Some("Subitem6") => Some(SentenceIndent::Subitem6),
        Some("Subitem7") => Some(SentenceIndent::Subitem7),
        Some("Subitem8") => Some(SentenceIndent::Subitem8),
        Some("Subitem9") => Some(SentenceIndent::Subitem9),
        Some("Subitem10") => Some(SentenceIndent::Subitem10),
        None => None,
        _ => {
          return Err(Error::AttributeParseError {
            tag_name: element.name.clone(),
            attribute_name: "Indent".to_string(),
          })
        }
      };
      let writing_mode = match element.attributes.get("WritingMode").map(|s| s.as_str()) {
        Some("vertical") => text::WritingMode::Vertical,
        Some("horizontal") => text::WritingMode::Horizontal,
        _ => text::WritingMode::Vertical,
      };
      let mut contents = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Line" => {
              let v = line::Line::parser(e)?;
              contents.push(SentenceElement::Line(v));
            }
            "QuoteStruct" => {
              let v = structs::QuoteStruct::parser(e)?;
              contents.push(SentenceElement::QuoteStruct(v));
            }
            "ArithFormula" => {
              let v = contents::ArithFormula::parser(e)?;
              contents.push(SentenceElement::ArithFormula(v));
            }
            "Ruby" => {
              let v = text::Ruby::parser(e)?;
              contents.push(SentenceElement::Ruby(v));
            }
            "Sup" => {
              let v = text::Sup::parser(e)?;
              contents.push(SentenceElement::Sup(v));
            }
            "Sub" => {
              let v = text::Sub::parser(e)?;
              contents.push(SentenceElement::Sub(v));
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        } else if let XMLNode::Text(s) = node {
          contents.push(SentenceElement::String(s.clone()))
        }
      }
      Ok(Sentence {
        contents,
        num,
        function,
        indent,
        writing_mode,
      })
    } else {
      Err(Error::wrong_tag_name(element, "Sentence"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SentenceElement {
  Line(line::Line),
  QuoteStruct(structs::QuoteStruct),
  ArithFormula(contents::ArithFormula),
  Ruby(text::Ruby),
  Sup(text::Sup),
  Sub(text::Sub),
  String(String),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SentenceFunction {
  Main,
  Proviso,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SentenceIndent {
  Paragraph,
  Item,
  Subitem1,
  Subitem2,
  Subitem3,
  Subitem4,
  Subitem5,
  Subitem6,
  Subitem7,
  Subitem8,
  Subitem9,
  Subitem10,
}
