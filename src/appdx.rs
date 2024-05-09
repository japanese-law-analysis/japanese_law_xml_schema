//! 付録

use crate::article_number::ArticleNumber;
use crate::contents::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::remarks::*;
use crate::result::Error;
use crate::structs::*;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppdxTable {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<AppdxTableContents>,
  pub remarks: Option<Remarks>,
  pub num: Option<ArticleNumber>,
}

impl Parser for AppdxTable {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "AppdxTable" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "AppdxTableTitle" => {
              let v = TextWithWritingMode::parser(e)?;
              title = Some(v)
            }
            "RelatedArticleNum" => related_article_num = Some(Text::from_children(&e.children)),
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              children.push(AppdxTableContents::TableStruct(v))
            }
            "Item" => {
              let v = Item::parser(e)?;
              children.push(AppdxTableContents::Item(v))
            }
            "Remarks" => {
              let v = Remarks::parser(e)?;
              remarks = Some(v)
            }
            s => return Err(Error::unexpected_tag(element, s)),
          }
        }
      }
      Ok(AppdxTable {
        title,
        related_article_num,
        children,
        remarks,
        num,
      })
    } else {
      Err(Error::wrong_tag_name(element, "AppdxTable"))
    }
  }
}

impl ToXmlElement for AppdxTable {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("AppdxTable");
    if let Some(v) = &self.title {
      e.children.push(XMLNode::Element(
        v.to_xml_element_with_name("AppdxTableTitle"),
      ));
    }
    if let Some(n) = &self.related_article_num {
      e.children.push(XMLNode::Element(
        n.to_xml_element_with_name("RelatedArticleNum"),
      ));
    }
    for v in self.children.iter() {
      match v {
        AppdxTableContents::Item(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        AppdxTableContents::TableStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
      }
    }
    if let Some(v) = &self.remarks {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    if let Some(n) = &self.num {
      e.attributes.insert("Num".to_string(), n.num_str());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppdxTableContents {
  TableStruct(TableStruct),
  Item(Item),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppdxNote {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<AppdxNoteContents>,
  pub remarks: Option<Remarks>,
  pub num: Option<ArticleNumber>,
}

impl Parser for AppdxNote {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "AppdxNote" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "AppdxNoteTitle" => {
              let v = TextWithWritingMode::parser(e)?;
              title = Some(v)
            }
            "RelatedArticleNum" => related_article_num = Some(Text::from_children(&e.children)),
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              children.push(AppdxNoteContents::TableStruct(v))
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              children.push(AppdxNoteContents::FigStruct(v))
            }
            "NoteStruct" => {
              let v = NoteStruct::parser(e)?;
              children.push(AppdxNoteContents::NoteStruct(v))
            }
            "Remarks" => {
              let v = Remarks::parser(e)?;
              remarks = Some(v)
            }
            s => return Err(Error::unexpected_tag(element, s)),
          }
        }
      }
      Ok(AppdxNote {
        title,
        related_article_num,
        children,
        remarks,
        num,
      })
    } else {
      Err(Error::wrong_tag_name(element, "AppdxNote"))
    }
  }
}

impl ToXmlElement for AppdxNote {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("AppdxNote");
    if let Some(v) = &self.title {
      e.children.push(XMLNode::Element(
        v.to_xml_element_with_name("AppdxNoteTitle"),
      ));
    }
    if let Some(n) = &self.related_article_num {
      e.children.push(XMLNode::Element(
        n.to_xml_element_with_name("RelatedArticleNum"),
      ));
    }
    for v in self.children.iter() {
      match v {
        AppdxNoteContents::NoteStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        AppdxNoteContents::TableStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        AppdxNoteContents::FigStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
      }
    }
    if let Some(v) = &self.remarks {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    if let Some(n) = &self.num {
      e.attributes.insert("Num".to_string(), n.num_str());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppdxNoteContents {
  NoteStruct(NoteStruct),
  FigStruct(FigStruct),
  TableStruct(TableStruct),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppdxStyle {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<StyleStruct>,
  pub remarks: Option<Remarks>,
  pub num: Option<ArticleNumber>,
}

impl Parser for AppdxStyle {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "AppdxStyle" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "AppdxStyleTitle" => {
              let v = TextWithWritingMode::parser(e)?;
              title = Some(v)
            }
            "RelatedArticleNum" => related_article_num = Some(Text::from_children(&e.children)),
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              children.push(v)
            }
            "Remarks" => {
              let v = Remarks::parser(e)?;
              remarks = Some(v)
            }
            s => return Err(Error::unexpected_tag(element, s)),
          }
        }
      }
      Ok(AppdxStyle {
        title,
        related_article_num,
        children,
        remarks,
        num,
      })
    } else {
      Err(Error::wrong_tag_name(element, "AppdxStyle"))
    }
  }
}

impl ToXmlElement for AppdxStyle {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("AppdxStyle");
    if let Some(v) = &self.title {
      e.children.push(XMLNode::Element(
        v.to_xml_element_with_name("AppdxStyleTitle"),
      ));
    }
    if let Some(n) = &self.related_article_num {
      e.children.push(XMLNode::Element(
        n.to_xml_element_with_name("RelatedArticleNum"),
      ));
    }
    for v in self.children.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    if let Some(v) = &self.remarks {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    if let Some(n) = &self.num {
      e.attributes.insert("Num".to_string(), n.num_str());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppdxFormat {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<FormatStruct>,
  pub remarks: Option<Remarks>,
  pub num: Option<ArticleNumber>,
}

impl Parser for AppdxFormat {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "AppdxFormat" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "AppdxFormatTitle" => {
              let v = TextWithWritingMode::parser(e)?;
              title = Some(v)
            }
            "RelatedArticleNum" => related_article_num = Some(Text::from_children(&e.children)),
            "FormatStruct" => {
              let v = FormatStruct::parser(e)?;
              children.push(v)
            }
            "Remarks" => {
              let v = Remarks::parser(e)?;
              remarks = Some(v)
            }
            s => return Err(Error::unexpected_tag(element, s)),
          }
        }
      }
      Ok(AppdxFormat {
        title,
        related_article_num,
        children,
        remarks,
        num,
      })
    } else {
      Err(Error::wrong_tag_name(element, "AppdxFormat"))
    }
  }
}
impl ToXmlElement for AppdxFormat {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("AppdxFormat");
    if let Some(v) = &self.title {
      e.children.push(XMLNode::Element(
        v.to_xml_element_with_name("AppdxFormatTitle"),
      ));
    }
    if let Some(n) = &self.related_article_num {
      e.children.push(XMLNode::Element(
        n.to_xml_element_with_name("RelatedArticleNum"),
      ));
    }
    for v in self.children.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    if let Some(v) = &self.remarks {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    if let Some(n) = &self.num {
      e.attributes.insert("Num".to_string(), n.num_str());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Appdx {
  pub arith_formula_num: Option<Text>,
  pub related_article_num: Option<Text>,
  pub arith_formula: Vec<ArithFormula>,
  pub remarks: Option<Remarks>,
}

impl Parser for Appdx {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Appdx" {
      let mut arith_formula_num = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut arith_formula = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ArithFormulaNum" => arith_formula_num = Some(Text::from_children(&e.children)),
            "RelatedArticleNum" => related_article_num = Some(Text::from_children(&e.children)),
            "ArithFormula" => {
              let v = ArithFormula::parser(e)?;
              arith_formula.push(v)
            }
            "Remarks" => {
              let v = Remarks::parser(e)?;
              remarks = Some(v)
            }
            s => return Err(Error::unexpected_tag(element, s)),
          }
        }
      }
      Ok(Appdx {
        arith_formula_num,
        related_article_num,
        arith_formula,
        remarks,
      })
    } else {
      Err(Error::wrong_tag_name(element, "Appdx"))
    }
  }
}

impl ToXmlElement for Appdx {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Appdx");
    if let Some(v) = &self.arith_formula_num {
      e.children.push(XMLNode::Element(
        v.to_xml_element_with_name("ArithFormulaNum"),
      ))
    }
    if let Some(v) = &self.related_article_num {
      e.children.push(XMLNode::Element(
        v.to_xml_element_with_name("ArithFormulaNum"),
      ))
    }
    for v in self.arith_formula.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    if let Some(v) = &self.remarks {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppdxFig {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<AppdxFigContents>,
  pub num: Option<usize>,
}

impl Parser for AppdxFig {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "AppdxFig" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "AppdxFigTitle" => {
              let v = TextWithWritingMode::parser(e)?;
              title = Some(v)
            }
            "RelatedArticleNum" => related_article_num = Some(Text::from_children(&e.children)),
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              children.push(AppdxFigContents::TableStruct(v))
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              children.push(AppdxFigContents::FigStruct(v))
            }
            s => return Err(Error::unexpected_tag(element, s)),
          }
        }
      }
      Ok(AppdxFig {
        title,
        related_article_num,
        children,
        num,
      })
    } else {
      Err(Error::wrong_tag_name(element, "AppdxFig"))
    }
  }
}
impl ToXmlElement for AppdxFig {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("AppdxFig");
    if let Some(v) = &self.title {
      e.children.push(XMLNode::Element(
        v.to_xml_element_with_name("AppdxFigTitle"),
      ));
    }
    if let Some(n) = &self.related_article_num {
      e.children.push(XMLNode::Element(
        n.to_xml_element_with_name("RelatedArticleNum"),
      ));
    }
    for v in self.children.iter() {
      match v {
        AppdxFigContents::FigStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        AppdxFigContents::TableStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
      }
    }
    if let Some(n) = &self.num {
      e.attributes.insert("Num".to_string(), n.to_string());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppdxFigContents {
  FigStruct(FigStruct),
  TableStruct(TableStruct),
}
