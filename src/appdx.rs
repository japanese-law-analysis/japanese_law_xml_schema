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
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<AppdxTableContents>,
  pub remarks: Option<Remarks>,
  pub num: Option<usize>,
}

impl Parser for AppdxTable {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxTable" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxTableTitle" => {
            let v = TextWithWritingMode::parser(&node)?;
            title = Some(v)
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            children.push(AppdxTableContents::TableStruct(v))
          }
          "Item" => {
            let v = Item::parser(&node)?;
            children.push(AppdxTableContents::Item(v))
          }
          "Remarks" => {
            let v = Remarks::parser(&node)?;
            remarks = Some(v)
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
      Err(Error::wrong_tag_name(node, "AppdxTable"))
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
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<AppdxNoteContents>,
  pub remarks: Option<Remarks>,
  pub num: Option<usize>,
}

impl Parser for AppdxNote {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxNote" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxNoteTitle" => {
            let v = TextWithWritingMode::parser(&node)?;
            title = Some(v)
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            children.push(AppdxNoteContents::TableStruct(v))
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            children.push(AppdxNoteContents::FigStruct(v))
          }
          "NoteStruct" => {
            let v = NoteStruct::parser(&node)?;
            children.push(AppdxNoteContents::NoteStruct(v))
          }
          "Remarks" => {
            let v = Remarks::parser(&node)?;
            remarks = Some(v)
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
      Err(Error::wrong_tag_name(node, "AppdxNote"))
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
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<StyleStruct>,
  pub remarks: Option<Remarks>,
  pub num: Option<usize>,
}

impl Parser for AppdxStyle {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxStyle" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxStyleTitle" => {
            let v = TextWithWritingMode::parser(&node)?;
            title = Some(v)
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            children.push(v)
          }
          "Remarks" => {
            let v = Remarks::parser(&node)?;
            remarks = Some(v)
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
      Err(Error::wrong_tag_name(node, "AppdxStyle"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxFormat {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<FormatStruct>,
  pub remarks: Option<Remarks>,
  pub num: Option<usize>,
}

impl Parser for AppdxFormat {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxFormat" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut remarks = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxFormatTitle" => {
            let v = TextWithWritingMode::parser(&node)?;
            title = Some(v)
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "FormatStruct" => {
            let v = FormatStruct::parser(&node)?;
            children.push(v)
          }
          "Remarks" => {
            let v = Remarks::parser(&node)?;
            remarks = Some(v)
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
      Err(Error::wrong_tag_name(node, "AppdxFormat"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Appdx {
  pub arith_formula_num: Option<Text>,
  pub related_article_num: Option<Text>,
  pub arith_formula: Vec<ArithFormula>,
  pub remarks: Option<Remarks>,
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
            let v = ArithFormula::parser(&node)?;
            arith_formula.push(v)
          }
          "Remarks" => {
            let v = Remarks::parser(&node)?;
            remarks = Some(v)
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(Appdx {
        arith_formula_num,
        related_article_num,
        arith_formula,
        remarks,
      })
    } else {
      Err(Error::wrong_tag_name(node, "Appdx"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxFig {
  pub title: Option<TextWithWritingMode>,
  pub related_article_num: Option<Text>,
  pub children: Vec<AppdxFigContents>,
  pub num: Option<usize>,
}

impl Parser for AppdxFig {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AppdxFig" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AppdxFigTitle" => {
            let v = TextWithWritingMode::parser(&node)?;
            title = Some(v)
          }
          "RelatedArticleNum" => related_article_num = Some(Text::from_children(node.children())),
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            children.push(AppdxFigContents::TableStruct(v))
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            children.push(AppdxFigContents::FigStruct(v))
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(AppdxFig {
        title,
        related_article_num,
        children,
        num,
      })
    } else {
      Err(Error::wrong_tag_name(node, "AppdxFig"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum AppdxFigContents {
  FigStruct(FigStruct),
  TableStruct(TableStruct),
}
