//! 附則
//!
use crate::article::*;
use crate::contents::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::result::Error;
use crate::structs::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

/// 附則
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvision {
  label: Text,
  children: Vec<SupplProvisionChildrenElement>,
  suppl_provision_type: Option<SupplProvisionType>,
  amend_law_num: Option<String>,
  extract: Option<bool>,
}

impl Parser for SupplProvision {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "SupplProvision" {
      let suppl_provision_type = match node.attribute("Type") {
        Some("New") => Some(SupplProvisionType::New),
        Some("Amend") => Some(SupplProvisionType::Amend),
        None => None,
        _ => return Err(Error::Attribute),
      };
      let amend_law_num = node.attribute("AmendLawNum").map(|s| s.to_string());
      let extract = node
        .attribute("Extract")
        .and_then(|s| s.parse::<bool>().ok());
      let mut label = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "SupplProvisionLabel" => {
            label = Some(Text::from_children(node.children()));
          }
          "Chapter" => {
            let v = Chapter::parser(&node)?;
            children.push(SupplProvisionChildrenElement::Chapter(v))
          }
          "Article" => {
            let v = Article::parser(&node)?;
            children.push(SupplProvisionChildrenElement::Article(v))
          }
          "Paragraph" => {
            let v = Paragraph::parser(&node)?;
            children.push(SupplProvisionChildrenElement::Paragraph(v))
          }
          "SupplProvisionAppdxTable" => {
            let v = SupplProvisionAppdxTable::parser(&node)?;
            children.push(SupplProvisionChildrenElement::SupplProvisionAppdxTable(v))
          }
          "SupplProvisionAppdxStyle" => {
            let v = SupplProvisionAppdxStyle::parser(&node)?;
            children.push(SupplProvisionChildrenElement::SupplProvisionAppdxStyle(v))
          }
          "SupplProvisionAppdx" => {
            let v = SupplProvisionAppdx::parser(&node)?;
            children.push(SupplProvisionChildrenElement::SupplProvisionAppdx(v))
          }
          _ => {}
        }
      }
      if let Some(label) = label {
        Ok(SupplProvision {
          label,
          children,
          suppl_provision_type,
          amend_law_num,
          extract,
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SupplProvisionChildrenElement {
  Chapter(article::Chapter),
  Article(article::Article),
  Paragraph(paragraph::Paragraph),
  SupplProvisionAppdxTable(SupplProvisionAppdxTable),
  SupplProvisionAppdxStyle(SupplProvisionAppdxStyle),
  SupplProvisionAppdx(SupplProvisionAppdx),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SupplProvisionType {
  New,
  Amend,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvisionAppdxTable {
  title: TextWithWritingMode,
  related_article_num: Option<Text>,
  table_struct: Vec<TableStruct>,
  num: Option<usize>,
}

impl Parser for SupplProvisionAppdxTable {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "SupplProvisionAppdxTable" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let mut title = None;
      let mut related_article_num = None;
      let mut table_struct = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "SupplProvisionAppdxTableTitle" => {
            if let Ok(v) = TextWithWritingMode::parser(&node) {
              title = Some(v);
            }
          }
          "RelatedArticleNum" => {
            related_article_num = Some(Text::from_children(node.children()));
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            table_struct.push(v)
          }
          _ => {}
        }
      }
      if let Some(title) = title {
        Ok(SupplProvisionAppdxTable {
          title,
          related_article_num,
          table_struct,
          num,
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvisionAppdxStyle {
  title: text::TextWithWritingMode,
  related_article_num: Option<Text>,
  style_struct: Vec<StyleStruct>,
  num: Option<usize>,
}

impl Parser for SupplProvisionAppdxStyle {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "SupplProvisionAppdxStyle" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let mut title = None;
      let mut related_article_num = None;
      let mut style_struct = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "SupplProvisionAppdxStyleTitle" => {
            if let Ok(v) = TextWithWritingMode::parser(&node) {
              title = Some(v);
            }
          }
          "RelatedArticleNum" => {
            related_article_num = Some(Text::from_children(node.children()));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            style_struct.push(v)
          }
          _ => {}
        }
      }
      if let Some(title) = title {
        Ok(SupplProvisionAppdxStyle {
          title,
          related_article_num,
          style_struct,
          num,
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvisionAppdx {
  arith_formula_num: Option<Text>,
  related_article_num: Option<Text>,
  arith_formula: Vec<ArithFormula>,
  num: Option<usize>,
}

impl Parser for SupplProvisionAppdx {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "SupplProvisionAppdx" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let mut arith_formula_num = None;
      let mut related_article_num = None;
      let mut arith_formula = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ArithFormulaNum" => {
            arith_formula_num = Some(Text::from_children(node.children()));
          }
          "RelatedArticleNum" => {
            related_article_num = Some(Text::from_children(node.children()));
          }
          "ArithFormula" => {
            let v = ArithFormula::parser(&node)?;
            arith_formula.push(v)
          }
          _ => {}
        }
      }
      Ok(SupplProvisionAppdx {
        arith_formula_num,
        related_article_num,
        arith_formula,
        num,
      })
    } else {
      Err(Error::Tag)
    }
  }
}
