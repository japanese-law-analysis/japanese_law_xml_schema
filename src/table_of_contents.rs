//! 目次
use crate::class::*;
use crate::parser::*;
use crate::result::Error;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

/// 目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TOC {
  /// 目次タイトル（概ね「目次」）
  pub toc_label: Option<Text>,
  /// 前文タイトル（概ね「前文」）
  pub toc_preamble_label: Option<Text>,
  /// 本文の目次
  pub toc_main_contents: Vec<TOCMainContents>,
  /// 附則の目次
  pub toc_suppl_provision: Option<TOCSupplProvision>,
  /// 付録表のタイトル
  pub toc_appdx_table_label: Vec<Text>,
}

impl Parser for TOC {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOC" {
      let mut label = None;
      let mut preamble_label = None;
      let mut main_contents = Vec::new();
      let mut suppl_provision = None;
      let mut appdx_table_lable = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "TOCLabel" => {
            label = Some(Text::from_children(node.children()));
          }
          "TOCPreambleLabel" => {
            preamble_label = Some(Text::from_children(node.children()));
          }
          "TOCPart" => {
            let v = TOCPart::parser(&node)?;
            main_contents.push(TOCMainContents::TOCPart(v));
          }
          "TOCChapter" => {
            let v = TOCChapter::parser(&node)?;
            main_contents.push(TOCMainContents::TOCChapter(v));
          }
          "TOCSection" => {
            let v = TOCSection::parser(&node)?;
            main_contents.push(TOCMainContents::TOCSection(v));
          }
          "TOCArticle" => {
            let v = TOCArticle::parser(&node)?;
            main_contents.push(TOCMainContents::TOCArticle(v));
          }
          "TOCSupplProvision" => {
            let v = TOCSupplProvision::parser(&node)?;
            suppl_provision = Some(v);
          }
          "TOCAppdxTableLabel" => {
            appdx_table_lable.push(Text::from_children(node.children()));
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(TOC {
        toc_label: label,
        toc_preamble_label: preamble_label,
        toc_main_contents: main_contents,
        toc_suppl_provision: suppl_provision,
        toc_appdx_table_label: appdx_table_lable,
      })
    } else {
      Err(Error::wrong_tag_name(node, "TOC"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum TOCMainContents {
  /// 編の目次
  TOCPart(TOCPart),
  /// 章の目次
  TOCChapter(TOCChapter),
  /// 節の目次
  TOCSection(TOCSection),
  /// 条の目次
  TOCArticle(TOCArticle),
}

/// 編の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TOCPart {
  /// 編のタイトル
  pub part_title: Text,
  /// 条の範囲
  pub article_range: Option<Text>,
  /// 編の番号
  pub num: String,
  /// 削除された編かどうか
  pub delete: bool,
  /// 子要素
  pub children: Vec<TOCChapter>,
}

impl Parser for TOCPart {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCPart" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "PartTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          "TOCChapter" => {
            let v = TOCChapter::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(TOCPart {
        part_title: title,
        article_range,
        num,
        delete,
        children,
      })
    } else {
      Err(Error::wrong_tag_name(node, "TOCPart"))
    }
  }
}

/// 章の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TOCChapter {
  /// 章のタイトル
  pub chapter_title: Text,
  /// 条の範囲
  pub article_range: Option<Text>,
  /// 章の番号
  pub num: String,
  /// 削除された章かどうか
  pub delete: bool,
  /// 子要素
  pub children: Vec<TOCSection>,
}

impl Parser for TOCChapter {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCChapter" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "ChapterTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          "TOCSection" => {
            let v = TOCSection::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(TOCChapter {
        chapter_title: title,
        article_range,
        num,
        delete,
        children,
      })
    } else {
      Err(Error::wrong_tag_name(node, "TOCChapter"))
    }
  }
}

/// 節の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TOCSection {
  /// 節のタイトル
  pub section_title: Text,
  /// 条の範囲
  pub article_range: Option<Text>,
  /// 節の番号
  pub num: String,
  /// 削除された節かどうか
  pub delete: bool,
  /// 子要素
  pub children: Vec<TOCSectionContents>,
}

impl Parser for TOCSection {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCSection" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "SectionTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          "TOCSubsection" => {
            let v = TOCSubsection::parser(&node)?;
            children.push(TOCSectionContents::TOCSubsection(v));
          }
          "TOCDivision" => {
            let v = TOCDivision::parser(&node)?;
            children.push(TOCSectionContents::TOCDivision(v));
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(TOCSection {
        section_title: title,
        article_range,
        num,
        delete,
        children,
      })
    } else {
      Err(Error::wrong_tag_name(node, "TOCSection"))
    }
  }
}

/// 節目次の子要素
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum TOCSectionContents {
  /// 款
  TOCSubsection(TOCSubsection),
  /// 目
  TOCDivision(TOCDivision),
}

/// 款の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TOCSubsection {
  /// 款のタイトル
  pub subsection_title: Text,
  /// 条の範囲
  pub article_range: Option<Text>,
  /// 款の番号
  pub num: String,
  /// 削除された款かどうか
  pub delete: bool,
  /// 子要素
  pub children: Vec<TOCDivision>,
}

impl Parser for TOCSubsection {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCSubsection" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "SubsectionTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          "TOCDivision" => {
            let v = TOCDivision::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(TOCSubsection {
        subsection_title: title,
        article_range,
        num,
        delete,
        children,
      })
    } else {
      Err(Error::wrong_tag_name(node, "TOCSubsection"))
    }
  }
}

/// 目の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TOCDivision {
  /// 目のタイトル
  pub division_title: Text,
  /// 条の範囲
  pub article_range: Option<Text>,
  /// 目の番号
  pub num: String,
  /// 削除された目かどうか
  pub delete: bool,
}

impl Parser for TOCDivision {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCDivision" {
      let mut title = Text::new();
      let mut article_range = None;
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "DivisionTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(TOCDivision {
        division_title: title,
        article_range,
        num,
        delete,
      })
    } else {
      Err(Error::wrong_tag_name(node, "TOCDivision"))
    }
  }
}

/// 条の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TOCArticle {
  /// 条のタイトル
  pub article_title: Text,
  /// 見出し
  pub article_caption: Caption,
  /// 条番号
  pub num: String,
  /// 削除された条かどうか
  pub delete: bool,
}

impl Parser for TOCArticle {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCArticle" {
      let mut title = Text::new();
      let mut caption = None;
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "ArticleTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleCaption" => {
            let v = Caption::parser(&node)?;
            caption = Some(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      if let Some(caption) = caption {
        Ok(TOCArticle {
          article_title: title,
          article_caption: caption,
          num: num.to_string(),
          delete,
        })
      } else {
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "Caption".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "TOCArticle"))
    }
  }
}

/// 附則の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TOCSupplProvision {
  /// 見出し（概ね「附則」）
  pub suppl_provision_label: Text,
  /// 範囲
  pub article_range: Option<Text>,
  /// 子要素
  pub children: Vec<TOCSupplProvisionContents>,
}

impl Parser for TOCSupplProvision {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCSupplProvision" {
      let mut label = Text::new();
      let mut range = None;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "SupplProvisionLabel" => {
            label = Text::from_children(node.children());
          }
          "ArticleRange" => {
            range = Some(Text::from_children(node.children()));
          }
          "TOCArticle" => {
            let v = TOCArticle::parser(&node)?;
            children.push(TOCSupplProvisionContents::TOCArticle(v));
          }
          "TOCChapter" => {
            let v = TOCChapter::parser(&node)?;
            children.push(TOCSupplProvisionContents::TOCChapter(v));
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(TOCSupplProvision {
        suppl_provision_label: label,
        article_range: range,
        children,
      })
    } else {
      Err(Error::wrong_tag_name(node, "TOCSupplProbision"))
    }
  }
}

/// 附則の目次の中身
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum TOCSupplProvisionContents {
  /// 条
  TOCArticle(TOCArticle),
  /// 章
  TOCChapter(TOCChapter),
}
