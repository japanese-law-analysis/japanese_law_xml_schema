use crate::parser::*;
use crate::result::Error;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sentence {
  contents: Vec<SentenceElement>,
  num: usize,
  function: SentenceFunction,
  indent: SentenceIndent,
  writing_mode: text::WritingMode,
}

impl Parser for Sentence {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Sentence" {
      let num = get_attribute_with_parse(node, "Num")?;
      let function = match node.attribute("Function") {
        Some("main") => SentenceFunction::Main,
        Some("proviso") => SentenceFunction::Proviso,
        _ => {
          return Err(Error::AttributeParseError {
            range: node.range(),
            tag_name: node.tag_name().name().to_string(),
            attribute_name: "Function".to_string(),
          })
        }
      };
      let indent = match node.attribute("Indent") {
        Some("Paragraph") => SentenceIndent::Paragraph,
        Some("Item") => SentenceIndent::Item,
        Some("Subitem1") => SentenceIndent::Subitem1,
        Some("Subitem2") => SentenceIndent::Subitem2,
        Some("Subitem3") => SentenceIndent::Subitem3,
        Some("Subitem4") => SentenceIndent::Subitem4,
        Some("Subitem5") => SentenceIndent::Subitem5,
        Some("Subitem6") => SentenceIndent::Subitem6,
        Some("Subitem7") => SentenceIndent::Subitem7,
        Some("Subitem8") => SentenceIndent::Subitem8,
        Some("Subitem9") => SentenceIndent::Subitem9,
        Some("Subitem10") => SentenceIndent::Subitem10,
        _ => {
          return Err(Error::AttributeParseError {
            range: node.range(),
            tag_name: node.tag_name().name().to_string(),
            attribute_name: "Indent".to_string(),
          })
        }
      };
      let writing_mode = match node.attribute("WritingMode") {
        Some("vertical") => text::WritingMode::Vertical,
        Some("horizontal") => text::WritingMode::Horizontal,
        _ => text::WritingMode::Vertical,
      };
      let mut contents = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "Line" => {
            let v = line::Line::parser(&node)?;
            contents.push(SentenceElement::Line(v));
          }
          "QuoteStruct" => {
            let v = structs::QuoteStruct::parser(&node)?;
            contents.push(SentenceElement::QuoteStruct(v));
          }
          "ArithFormula" => {
            let v = contents::ArithFormula::parser(&node)?;
            contents.push(SentenceElement::ArithFormula(v));
          }
          "Ruby" => {
            let v = text::Ruby::parser(&node)?;
            contents.push(SentenceElement::Ruby(v));
          }
          "Sup" => {
            let v = text::Sup::parser(&node)?;
            contents.push(SentenceElement::Sup(v));
          }
          "Sub" => {
            let v = text::Sub::parser(&node)?;
            contents.push(SentenceElement::Sub(v));
          }
          "" => {
            if let Some(t) = node.text() {
              contents.push(SentenceElement::String(t.to_string()))
            }
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
      Err(Error::wrong_tag_name(node, "Sentence"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SentenceElement {
  Line(line::Line),
  QuoteStruct(structs::QuoteStruct),
  ArithFormula(contents::ArithFormula),
  Ruby(text::Ruby),
  Sup(text::Sup),
  Sub(text::Sub),
  String(String),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SentenceFunction {
  Main,
  Proviso,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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
