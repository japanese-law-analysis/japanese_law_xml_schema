//! 付録

use crate::contents::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::remarks::*;
use crate::result::Error;
use crate::structs::*;
use crate::text::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppdxTable {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<AppdxTableContents>,
  pub remarks: Option<Remarks>,
  pub num: Option<usize>,
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
            s => return Err(Error::unexpected_tag(e, s)),
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
  pub num: Option<usize>,
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
            s => return Err(Error::unexpected_tag(e, s)),
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
  pub num: Option<usize>,
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
            s => return Err(Error::unexpected_tag(e, s)),
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

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppdxFormat {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<FormatStruct>,
  pub remarks: Option<Remarks>,
  pub num: Option<usize>,
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
            s => return Err(Error::unexpected_tag(e, s)),
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
            s => return Err(Error::unexpected_tag(e, s)),
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
            s => return Err(Error::unexpected_tag(e, s)),
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

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppdxFigContents {
  FigStruct(FigStruct),
  TableStruct(TableStruct),
}
