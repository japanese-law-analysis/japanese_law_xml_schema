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
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

/// 附則
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupplProvision {
  pub label: Text,
  pub children: Vec<SupplProvisionChildrenElement>,
  pub suppl_provision_type: Option<SupplProvisionType>,
  pub amend_law_num: Option<String>,
  pub extract: Option<bool>,
}

impl Parser for SupplProvision {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "SupplProvision" {
      let suppl_provision_type = match element.attributes.get("Type").map(|s| s.as_str()) {
        Some("New") => Some(SupplProvisionType::New),
        Some("Amend") => Some(SupplProvisionType::Amend),
        None => None,
        _ => {
          return Err(Error::AttributeParseError {
            tag_name: "SupplProbision".to_string(),
            attribute_name: "Type".to_string(),
          })
        }
      };
      let amend_law_num = element.attributes.get("AmendLawNum").cloned();
      let extract = get_attribute_opt_with_parse(element, "Extract")?;
      let mut label = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "SupplProvisionLabel" => {
              label = Some(Text::from_children(&e.children));
            }
            "Chapter" => {
              let v = Chapter::parser(e)?;
              children.push(SupplProvisionChildrenElement::Chapter(v))
            }
            "Article" => {
              let v = Article::parser(e)?;
              children.push(SupplProvisionChildrenElement::Article(v))
            }
            "Paragraph" => {
              let v = Paragraph::parser(e)?;
              children.push(SupplProvisionChildrenElement::Paragraph(v))
            }
            "SupplProvisionAppdxTable" => {
              let v = SupplProvisionAppdxTable::parser(e)?;
              children.push(SupplProvisionChildrenElement::SupplProvisionAppdxTable(v))
            }
            "SupplProvisionAppdxStyle" => {
              let v = SupplProvisionAppdxStyle::parser(e)?;
              children.push(SupplProvisionChildrenElement::SupplProvisionAppdxStyle(v))
            }
            "SupplProvisionAppdx" => {
              let v = SupplProvisionAppdx::parser(e)?;
              children.push(SupplProvisionChildrenElement::SupplProvisionAppdx(v))
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
          tag_name: "SupplProbisionLabel".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "SupplProbision"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupplProvisionChildrenElement {
  Chapter(article::Chapter),
  Article(article::Article),
  Paragraph(paragraph::Paragraph),
  SupplProvisionAppdxTable(SupplProvisionAppdxTable),
  SupplProvisionAppdxStyle(SupplProvisionAppdxStyle),
  SupplProvisionAppdx(SupplProvisionAppdx),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupplProvisionType {
  New,
  Amend,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupplProvisionAppdxTable {
  pub title: TextWithWritingMode,
  pub related_article_num: Option<Text>,
  pub table_struct: Vec<TableStruct>,
  pub num: Option<usize>,
}

impl Parser for SupplProvisionAppdxTable {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "SupplProvisionAppdxTable" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut table_struct = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "SupplProvisionAppdxTableTitle" => {
              let v = TextWithWritingMode::parser(e)?;
              title = Some(v);
            }
            "RelatedArticleNum" => {
              related_article_num = Some(Text::from_children(&e.children));
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              table_struct.push(v)
            }
            s => return Err(Error::wrong_tag_name(e, s)),
          }
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
          tag_name: "SupplProbisionAddxTableTitle".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "SupplProbisionAppdxTable"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupplProvisionAppdxStyle {
  pub title: text::TextWithWritingMode,
  pub related_article_num: Option<Text>,
  pub style_struct: Vec<StyleStruct>,
  pub num: Option<usize>,
}

impl Parser for SupplProvisionAppdxStyle {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "SupplProvisionAppdxStyle" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let mut title = None;
      let mut related_article_num = None;
      let mut style_struct = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "SupplProvisionAppdxStyleTitle" => {
              let v = TextWithWritingMode::parser(e)?;
              title = Some(v);
            }
            "RelatedArticleNum" => {
              related_article_num = Some(Text::from_children(&e.children));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              style_struct.push(v)
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
          tag_name: "SupplProbivionAppdxStyle".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "SupplProbisionAppdxStyle"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupplProvisionAppdx {
  pub arith_formula_num: Option<Text>,
  pub related_article_num: Option<Text>,
  pub arith_formula: Vec<ArithFormula>,
  pub num: Option<usize>,
}

impl Parser for SupplProvisionAppdx {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "SupplProvisionAppdx" {
      let num = get_attribute_opt_with_parse(element, "Num")?;
      let mut arith_formula_num = None;
      let mut related_article_num = None;
      let mut arith_formula = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ArithFormulaNum" => {
              arith_formula_num = Some(Text::from_children(&e.children));
            }
            "RelatedArticleNum" => {
              related_article_num = Some(Text::from_children(&e.children));
            }
            "ArithFormula" => {
              let v = ArithFormula::parser(e)?;
              arith_formula.push(v)
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(SupplProvisionAppdx {
        arith_formula_num,
        related_article_num,
        arith_formula,
        num,
      })
    } else {
      Err(Error::wrong_tag_name(element, "SupplProbisionAppdx"))
    }
  }
}
