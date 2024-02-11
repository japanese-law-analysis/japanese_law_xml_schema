//! 付録

use crate::contents::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::remarks::*;
use crate::result::Error;
use crate::structs::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxTable {
  title: Option<TextWithWritingMode>,
  related_article_num: Option<Text>,
  children: Vec<AppdxTableContents>,
  remarks: Option<Remarks>,
  num: Option<usize>,
}

impl Parser for AppdxTable {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxTable" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxTableTitle" => {
            if let Ok(v) = TextWithWritingMode::parser(&node) {
              title = Some(v)
            }
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "TableStruct" => {
            if let Ok(v) = TableStruct::parser(&node) {
              children.push(AppdxTableContents::TableStruct(v))
            }
          }
          "Item" => {
            if let Ok(v) = Item::parser(&node) {
              children.push(AppdxTableContents::Item(v))
            }
          }
          "Remarks" => {
            if let Ok(v) = Remarks::parser(&node) {
              remarks = Some(v)
            }
          }
          _ => {}
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
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum AppdxTableContents {
  TableStruct(TableStruct),
  Item(Item),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxNote {
  title: Option<TextWithWritingMode>,
  related_article_num: Option<Text>,
  children: Vec<AppdxNoteContents>,
  remarks: Option<Remarks>,
  num: Option<usize>,
}

impl Parser for AppdxNote {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxNote" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxNoteTitle" => {
            if let Ok(v) = TextWithWritingMode::parser(&node) {
              title = Some(v)
            }
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "TableStruct" => {
            if let Ok(v) = TableStruct::parser(&node) {
              children.push(AppdxNoteContents::TableStruct(v))
            }
          }
          "FigStruct" => {
            if let Ok(v) = FigStruct::parser(&node) {
              children.push(AppdxNoteContents::FigStruct(v))
            }
          }
          "NoteStruct" => {
            if let Ok(v) = NoteStruct::parser(&node) {
              children.push(AppdxNoteContents::NoteStruct(v))
            }
          }
          "Remarks" => {
            if let Ok(v) = Remarks::parser(&node) {
              remarks = Some(v)
            }
          }
          _ => {}
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
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum AppdxNoteContents {
  NoteStruct(NoteStruct),
  FigStruct(FigStruct),
  TableStruct(TableStruct),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxStyle {
  title: Option<TextWithWritingMode>,
  related_article_num: Option<Text>,
  children: Vec<StyleStruct>,
  remarks: Option<Remarks>,
  num: Option<usize>,
}

impl Parser for AppdxStyle {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxStyle" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxStyleTitle" => {
            if let Ok(v) = TextWithWritingMode::parser(&node) {
              title = Some(v)
            }
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "StyleStruct" => {
            if let Ok(v) = StyleStruct::parser(&node) {
              children.push(v)
            }
          }
          "Remarks" => {
            if let Ok(v) = Remarks::parser(&node) {
              remarks = Some(v)
            }
          }
          _ => {}
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
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxFormat {
  title: Option<TextWithWritingMode>,
  related_article_num: Option<Text>,
  children: Vec<FormatStruct>,
  remarks: Option<Remarks>,
  num: Option<usize>,
}

impl Parser for AppdxFormat {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxFormat" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxFormatTitle" => {
            if let Ok(v) = TextWithWritingMode::parser(&node) {
              title = Some(v)
            }
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "FormatStruct" => {
            if let Ok(v) = FormatStruct::parser(&node) {
              children.push(v)
            }
          }
          "Remarks" => {
            if let Ok(v) = Remarks::parser(&node) {
              remarks = Some(v)
            }
          }
          _ => {}
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
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Appdx {
  arith_formula_num: Option<Text>,
  related_article_num: Option<Text>,
  arith_formula: Vec<ArithFormula>,
  remarks: Option<Remarks>,
}

impl Parser for Appdx {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Appdx" {
      let mut arith_formula_num = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut arith_formula = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ArithFormulaNum" => arith_formula_num = Some(Text::from_children(node.children())),
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "ArithFormula" => {
            if let Ok(v) = ArithFormula::parser(&node) {
              arith_formula.push(v)
            }
          }
          "Remarks" => {
            if let Ok(v) = Remarks::parser(&node) {
              remarks = Some(v)
            }
          }
          _ => {}
        }
      }
      Ok(Appdx {
        arith_formula_num,
        related_article_num,
        arith_formula,
        remarks,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxFig {
  title: Option<TextWithWritingMode>,
  related_article_num: Option<Text>,
  children: Vec<AppdxFigContents>,
  num: Option<usize>,
}

impl Parser for AppdxFig {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxFig" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let mut title = None;
      let mut related_article_num = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxFigTitle" => {
            if let Ok(v) = TextWithWritingMode::parser(&node) {
              title = Some(v)
            }
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "TableStruct" => {
            if let Ok(v) = TableStruct::parser(&node) {
              children.push(AppdxFigContents::TableStruct(v))
            }
          }
          "FigStruct" => {
            if let Ok(v) = FigStruct::parser(&node) {
              children.push(AppdxFigContents::FigStruct(v))
            }
          }
          _ => {}
        }
      }
      Ok(AppdxFig {
        title,
        related_article_num,
        children,
        num,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum AppdxFigContents {
  FigStruct(FigStruct),
  TableStruct(TableStruct),
}
