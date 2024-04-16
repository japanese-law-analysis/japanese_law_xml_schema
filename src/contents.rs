use crate::fig::*;
use crate::line::*;
use crate::list::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::sentence::*;
use crate::structs::*;
use crate::table::*;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

/// AritchFormulaやQuoteStructの中身
/// `any`とあるが、現実的にありえるパターンを列挙する
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contents {
  pub contents: Vec<ContentsElement>,
}

impl parser::Parser for Contents {
  fn parser(element: &Element) -> result::Result<Self> {
    let mut lst = Vec::new();
    for node in element.children.iter() {
      if let XMLNode::Element(e) = node {
        match e.name.as_str() {
          "Table" => {
            let v = Table::parser(e)?;
            lst.push(ContentsElement::Table(v));
          }
          "TableStruct" => {
            let v = TableStruct::parser(e)?;
            lst.push(ContentsElement::TableStruct(v));
          }
          "Fig" => {
            let v = Fig::parser(e)?;
            lst.push(ContentsElement::Fig(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(e)?;
            lst.push(ContentsElement::FigStruct(v));
          }
          "Ruby" => {
            let v = Ruby::parser(e)?;
            lst.push(ContentsElement::Ruby(v));
          }
          "Line" => {
            let v = Line::parser(e)?;
            lst.push(ContentsElement::Line(v));
          }
          "Sup" => {
            let v = Sup::parser(e)?;
            lst.push(ContentsElement::Sup(v));
          }
          "Sub" => {
            let v = Sub::parser(e)?;
            lst.push(ContentsElement::Sub(v));
          }
          "Paragraph" => {
            let v = Paragraph::parser(e)?;
            lst.push(ContentsElement::Paragraph(v));
          }
          "List" => {
            let v = List::parser(e)?;
            lst.push(ContentsElement::List(v));
          }
          "Sentence" => {
            let v = Sentence::parser(e)?;
            lst.push(ContentsElement::Sentence(v));
          }
          s => return Err(Error::unexpected_tag(e, s)),
        }
      } else if let XMLNode::Text(s) = node {
        lst.push(ContentsElement::String(s.to_string()))
      }
    }
    Ok(Contents { contents: lst })
  }
}

impl ToXmlElementWithName for Contents {
  fn to_xml_element_with_name(&self, name: &str) -> Element {
    let mut e = Element::new(name);
    for n in self.contents.iter() {
      match n {
        ContentsElement::Table(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::TableStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Fig(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::FigStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Ruby(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Line(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Sub(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Sup(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Paragraph(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::List(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Sentence(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::String(s) => e.children.push(XMLNode::Text(s.clone())),
      }
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    Contents::parser(element).map(|c| Style { contentes: c })
  }
}

impl ToXmlElement for Style {
  fn to_xml_element(&self) -> Element {
    self.contentes.to_xml_element_with_name("Style")
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Note {
  pub contentes: Contents,
}

impl Parser for Note {
  fn parser(element: &Element) -> result::Result<Self> {
    Contents::parser(element).map(|c| Note { contentes: c })
  }
}

impl ToXmlElement for Note {
  fn to_xml_element(&self) -> Element {
    self.contentes.to_xml_element_with_name("Note")
  }
}

/// 様式
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Format {
  pub contentes: Contents,
}

impl Parser for Format {
  fn parser(element: &Element) -> result::Result<Self> {
    Contents::parser(element).map(|c| Format { contentes: c })
  }
}

impl ToXmlElement for Format {
  fn to_xml_element(&self) -> Element {
    self.contentes.to_xml_element_with_name("Format")
  }
}

/// 数式
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArithFormula {
  pub num: Option<usize>,
  pub contentes: Contents,
}

impl Parser for ArithFormula {
  fn parser(element: &Element) -> result::Result<Self> {
    let num = get_attribute_opt_with_parse(element, "Num")?;
    Contents::parser(element).map(|c| ArithFormula { num, contentes: c })
  }
}

impl ToXmlElement for ArithFormula {
  fn to_xml_element(&self) -> Element {
    self.contentes.to_xml_element_with_name("ArithFormula")
  }
}
