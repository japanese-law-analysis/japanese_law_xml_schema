//! anyとなるようなものをまとめるもの

use crate::appdx::*;
use crate::fig::*;
use crate::line::*;
use crate::list::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::remarks::*;
use crate::sentence::*;
use crate::structs::*;
use crate::table::*;
use crate::table_of_contents::{TOCSection, TOC};
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
          "QuoteStruct" => {
            let v = QuoteStruct::parser(e)?;
            lst.push(ContentsElement::QuoteStruct(v));
          }
          "NoteStruct" => {
            let v = NoteStruct::parser(e)?;
            lst.push(ContentsElement::NoteStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(e)?;
            lst.push(ContentsElement::StyleStruct(v));
          }
          "AppdxTable" => {
            let v = AppdxTable::parser(e)?;
            lst.push(ContentsElement::AppdxTable(v));
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
          "Item" => {
            let v = Item::parser(e)?;
            lst.push(ContentsElement::Item(v));
          }
          "Subitem1" => {
            let v = Subitem1::parser(e)?;
            lst.push(ContentsElement::Subitem1(v));
          }
          "Subitem2" => {
            let v = Subitem2::parser(e)?;
            lst.push(ContentsElement::Subitem2(v));
          }
          "Subitem3" => {
            let v = Subitem3::parser(e)?;
            lst.push(ContentsElement::Subitem3(v));
          }
          "Subitem4" => {
            let v = Subitem4::parser(e)?;
            lst.push(ContentsElement::Subitem4(v));
          }
          "Subitem5" => {
            let v = Subitem5::parser(e)?;
            lst.push(ContentsElement::Subitem5(v));
          }
          "Subitem6" => {
            let v = Subitem6::parser(e)?;
            lst.push(ContentsElement::Subitem6(v));
          }
          "Subitem7" => {
            let v = Subitem7::parser(e)?;
            lst.push(ContentsElement::Subitem7(v));
          }
          "Subitem8" => {
            let v = Subitem8::parser(e)?;
            lst.push(ContentsElement::Subitem8(v));
          }
          "Subitem9" => {
            let v = Subitem9::parser(e)?;
            lst.push(ContentsElement::Subitem9(v));
          }
          "Subitem10" => {
            let v = Subitem10::parser(e)?;
            lst.push(ContentsElement::Subitem10(v));
          }
          "List" => {
            let v = List::parser(e)?;
            lst.push(ContentsElement::List(v));
          }
          "Sentence" => {
            let v = Sentence::parser(e)?;
            lst.push(ContentsElement::Sentence(v));
          }
          "ArithFormula" => {
            let v = ArithFormula::parser(e)?;
            lst.push(ContentsElement::ArithFormula(v));
          }
          "TOC" => {
            let v = TOC::parser(e)?;
            lst.push(ContentsElement::TOC(v));
          }
          "TOCSection" => {
            let v = TOCSection::parser(e)?;
            lst.push(ContentsElement::TOCSection(v));
          }
          "Remarks" => {
            let v = Remarks::parser(e)?;
            lst.push(ContentsElement::Remarks(v));
          }
          "TableRow" => {
            let v = TableRow::parser(e)?;
            lst.push(ContentsElement::TableRow(v));
          }
          s => return Err(Error::unexpected_tag(element, s)),
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
        ContentsElement::QuoteStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::NoteStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::StyleStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::AppdxTable(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Ruby(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Line(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Sub(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Sup(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Paragraph(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Item(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem1(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem2(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem3(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem4(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem5(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem6(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem7(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem8(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem9(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Subitem10(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::List(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Sentence(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::ArithFormula(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::TOC(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::TOCSection(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::Remarks(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        ContentsElement::TableRow(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
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
  QuoteStruct(QuoteStruct),
  NoteStruct(NoteStruct),
  StyleStruct(StyleStruct),
  AppdxTable(AppdxTable),
  Ruby(Ruby),
  Line(Line),
  Sup(Sup),
  Sub(Sub),
  String(String),
  Paragraph(Paragraph),
  Item(Item),
  Subitem1(Subitem1),
  Subitem2(Subitem2),
  Subitem3(Subitem3),
  Subitem4(Subitem4),
  Subitem5(Subitem5),
  Subitem6(Subitem6),
  Subitem7(Subitem7),
  Subitem8(Subitem8),
  Subitem9(Subitem9),
  Subitem10(Subitem10),
  List(List),
  Sentence(Sentence),
  ArithFormula(ArithFormula),
  Remarks(Remarks),
  TOC(TOC),
  TOCSection(TOCSection),
  TableRow(TableRow),
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
