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
  pub label: Text,
  pub children: Vec<SupplProvisionChildrenElement>,
  pub suppl_provision_type: Option<SupplProvisionType>,
  pub amend_law_num: Option<String>,
  pub extract: Option<bool>,
}

impl Parser for SupplProvision {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "SupplProvision" {
      let suppl_provision_type = match node.attribute("Type") {
        Some("New") => Some(SupplProvisionType::New),
        Some("Amend") => Some(SupplProvisionType::Amend),
        None => None,
        _ => {
          return Err(Error::AttributeParseError {
            range: node.range(),
            tag_name: "SupplProbision".to_string(),
            attribute_name: "Type".to_string(),
          })
        }
      };
      let amend_law_num = node.attribute("AmendLawNum").map(|s| s.to_string());
      let extract = get_attribute_opt_with_parse(node, "Extract")?;
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
          s => return Err(Error::unexpected_tag(&node, s)),
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
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "SupplProbisionLabel".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "SupplProbision"))
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
  pub title: TextWithWritingMode,
  pub related_article_num: Option<Text>,
  pub table_struct: Vec<TableStruct>,
  pub num: Option<usize>,
}

impl Parser for SupplProvisionAppdxTable {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "SupplProvisionAppdxTable" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut table_struct = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "SupplProvisionAppdxTableTitle" => {
            let v = TextWithWritingMode::parser(&node)?;
            title = Some(v);
          }
          "RelatedArticleNum" => {
            related_article_num = Some(Text::from_children(node.children()));
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            table_struct.push(v)
          }
          s => return Err(Error::wrong_tag_name(&node, s)),
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
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "SupplProbisionAddxTableTitle".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "SupplProbisionAppdxTable"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvisionAppdxStyle {
  pub title: text::TextWithWritingMode,
  pub related_article_num: Option<Text>,
  pub style_struct: Vec<StyleStruct>,
  pub num: Option<usize>,
}

impl Parser for SupplProvisionAppdxStyle {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "SupplProvisionAppdxStyle" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut style_struct = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "SupplProvisionAppdxStyleTitle" => {
            let v = TextWithWritingMode::parser(&node)?;
            title = Some(v);
          }
          "RelatedArticleNum" => {
            related_article_num = Some(Text::from_children(node.children()));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            style_struct.push(v)
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "SupplProbivionAppdxStyle".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "SupplProbisionAppdxStyle"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvisionAppdx {
  pub arith_formula_num: Option<Text>,
  pub related_article_num: Option<Text>,
  pub arith_formula: Vec<ArithFormula>,
  pub num: Option<usize>,
}

impl Parser for SupplProvisionAppdx {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "SupplProvisionAppdx" {
      let num = get_attribute_opt_with_parse(node, "Num")?;
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
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(SupplProvisionAppdx {
        arith_formula_num,
        related_article_num,
        arith_formula,
        num,
      })
    } else {
      Err(Error::wrong_tag_name(node, "SupplProbisionAppdx"))
    }
  }
}
