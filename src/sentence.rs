//! 節

use crate::parser::*;
use crate::result::Error;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

use self::text::WritingMode;

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sentence {
  pub contents: Vec<SentenceElement>,
  pub num: Option<usize>,
  pub function: Option<SentenceFunction>,
  pub indent: Option<SentenceIndent>,
  pub writing_mode: text::WritingMode,
}

impl Parser for Sentence {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Sentence" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
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

impl ToXmlElement for Sentence {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Sentence");
    if let Some(num) = self.num {
      e.attributes.insert("Num".to_string(), num.to_string());
    }
    match self.function {
      Some(SentenceFunction::Main) => {
        e.attributes
          .insert("Function".to_string(), "main".to_string());
      }
      Some(SentenceFunction::Proviso) => {
        e.attributes
          .insert("Function".to_string(), "proviso".to_string());
      }
      None => (),
    }
    match self.indent {
      Some(SentenceIndent::Paragraph) => {
        e.attributes
          .insert("Indent".to_string(), "Paragraph".to_string());
      }
      Some(SentenceIndent::Item) => {
        e.attributes
          .insert("Indent".to_string(), "Item".to_string());
      }
      Some(SentenceIndent::Subitem1) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem1".to_string());
      }
      Some(SentenceIndent::Subitem2) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem2".to_string());
      }
      Some(SentenceIndent::Subitem3) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem3".to_string());
      }
      Some(SentenceIndent::Subitem4) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem4".to_string());
      }
      Some(SentenceIndent::Subitem5) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem5".to_string());
      }
      Some(SentenceIndent::Subitem6) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem6".to_string());
      }
      Some(SentenceIndent::Subitem7) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem7".to_string());
      }
      Some(SentenceIndent::Subitem8) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem8".to_string());
      }
      Some(SentenceIndent::Subitem9) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem9".to_string());
      }
      Some(SentenceIndent::Subitem10) => {
        e.attributes
          .insert("Indent".to_string(), "Subitem10".to_string());
      }
      None => (),
    }
    match self.writing_mode {
      WritingMode::Horizontal => {
        e.attributes
          .insert("WritingMode".to_string(), "horizontal".to_string());
      }
      WritingMode::Vertical => (),
    }
    for n in self.contents.iter() {
      match n {
        SentenceElement::ArithFormula(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        SentenceElement::QuoteStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        SentenceElement::Ruby(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        SentenceElement::Sup(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        SentenceElement::Sub(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        SentenceElement::Line(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        SentenceElement::String(s) => e.children.push(XMLNode::Text(s.clone())),
      }
    }
    e
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
