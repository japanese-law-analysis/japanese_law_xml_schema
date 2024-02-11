//! 目次
use crate::parser::*;
use crate::result::Error;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

/// 目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOC {
  /// 目次タイトル（概ね「目次」）
  toc_label: Option<Text>,
  /// 前文タイトル（概ね「前文」）
  toc_preamble_label: Option<Text>,
  /// 本文の目次
  toc_main_contents: Vec<TOCMainContents>,
  /// 附則の目次
  toc_suppl_provision: Option<TOCSupplProvision>,
  /// 付録表のタイトル
  toc_appdx_table_label: Vec<Text>,
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
            if let Ok(v) = TOCPart::parser(&node) {
              main_contents.push(TOCMainContents::TOCPart(v));
            }
          }
          "TOCChapter" => {
            if let Ok(v) = TOCChapter::parser(&node) {
              main_contents.push(TOCMainContents::TOCChapter(v));
            }
          }
          "TOCSection" => {
            if let Ok(v) = TOCSection::parser(&node) {
              main_contents.push(TOCMainContents::TOCSection(v));
            }
          }
          "TOCArticle" => {
            if let Ok(v) = TOCArticle::parser(&node) {
              main_contents.push(TOCMainContents::TOCArticle(v));
            }
          }
          "TOCSupplProvision" => {
            if let Ok(v) = TOCSupplProvision::parser(&node) {
              suppl_provision = Some(v);
            }
          }
          "TOCAppdxTableLabel" => {
            appdx_table_lable.push(Text::from_children(node.children()));
          }
          _ => {}
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
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCPart {
  /// 編のタイトル
  part_title: Text,
  /// 条の範囲
  article_range: Option<Text>,
  /// 編の番号
  num: String,
  /// 削除された編かどうか
  delete: bool,
  /// 子要素
  children: Vec<TOCChapter>,
}

impl Parser for TOCPart {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCPart" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = node.attribute("Num").unwrap_or_default();
      let delete = node
        .attribute("Delete")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "PartTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          "TOCChapter" => {
            if let Ok(v) = TOCChapter::parser(&node) {
              children.push(v);
            }
          }
          _ => {}
        }
      }
      Ok(TOCPart {
        part_title: title,
        article_range,
        num: num.to_string(),
        delete,
        children,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

/// 章の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCChapter {
  /// 章のタイトル
  chapter_title: Text,
  /// 条の範囲
  article_range: Option<Text>,
  /// 章の番号
  num: String,
  /// 削除された章かどうか
  delete: bool,
  /// 子要素
  children: Vec<TOCSection>,
}

impl Parser for TOCChapter {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCChapter" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = node.attribute("Num").unwrap_or_default();
      let delete = node
        .attribute("Delete")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "ChapterTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          "TOCSection" => {
            if let Ok(v) = TOCSection::parser(&node) {
              children.push(v);
            }
          }
          _ => {}
        }
      }
      Ok(TOCChapter {
        chapter_title: title,
        article_range,
        num: num.to_string(),
        delete,
        children,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

/// 節の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCSection {
  /// 節のタイトル
  section_title: Text,
  /// 条の範囲
  article_range: Option<Text>,
  /// 節の番号
  num: String,
  /// 削除された節かどうか
  delete: bool,
  /// 子要素
  children: Vec<TOCSectionContents>,
}

impl Parser for TOCSection {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCSection" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = node.attribute("Num").unwrap_or_default();
      let delete = node
        .attribute("Delete")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "SectionTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          "TOCSubsection" => {
            if let Ok(v) = TOCSubsection::parser(&node) {
              children.push(TOCSectionContents::TOCSubsection(v));
            }
          }
          "TOCDivision" => {
            if let Ok(v) = TOCDivision::parser(&node) {
              children.push(TOCSectionContents::TOCDivision(v));
            }
          }
          _ => {}
        }
      }
      Ok(TOCSection {
        section_title: title,
        article_range,
        num: num.to_string(),
        delete,
        children,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

/// 節目次の子要素
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum TOCSectionContents {
  /// 款
  TOCSubsection(TOCSubsection),
  /// 目
  TOCDivision(TOCDivision),
}

/// 款の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCSubsection {
  /// 款のタイトル
  subsection_title: Text,
  /// 条の範囲
  article_range: Option<Text>,
  /// 款の番号
  num: String,
  /// 削除された款かどうか
  delete: bool,
  /// 子要素
  children: Vec<TOCDivision>,
}

impl Parser for TOCSubsection {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCSubsection" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = node.attribute("Num").unwrap_or_default();
      let delete = node
        .attribute("Delete")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "SubsectionTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          "TOCDivision" => {
            if let Ok(v) = TOCDivision::parser(&node) {
              children.push(v);
            }
          }
          _ => {}
        }
      }
      Ok(TOCSubsection {
        subsection_title: title,
        article_range,
        num: num.to_string(),
        delete,
        children,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

/// 目の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCDivision {
  /// 目のタイトル
  division_title: Text,
  /// 条の範囲
  article_range: Option<Text>,
  /// 目の番号
  num: String,
  /// 削除された目かどうか
  delete: bool,
}

impl Parser for TOCDivision {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCDivision" {
      let mut title = Text::new();
      let mut article_range = None;
      let num = node.attribute("Num").unwrap_or_default();
      let delete = node
        .attribute("Delete")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "DivisionTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleRange" => {
            article_range = Some(Text::from_children(node.children()));
          }
          _ => {}
        }
      }
      Ok(TOCDivision {
        division_title: title,
        article_range,
        num: num.to_string(),
        delete,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

/// 条の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCArticle {
  /// 条のタイトル
  article_title: Text,
  /// 見出し
  article_caption: Caption,
  /// 条番号
  num: String,
  /// 削除された条かどうか
  delete: bool,
}

impl Parser for TOCArticle {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TOCArticle" {
      let mut title = Text::new();
      let mut caption = None;
      let num = node.attribute("Num").unwrap_or_default();
      let delete = node
        .attribute("Delete")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      for node in node.children() {
        match node.tag_name().name() {
          "ArticleTitle" => {
            title = Text::from_children(node.children());
          }
          "ArticleCaption" => {
            if let Ok(c) = Caption::parser(&node) {
              caption = Some(c);
            }
          }
          _ => {}
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
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

/// 附則の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCSupplProvision {
  /// 見出し（概ね「附則」）
  suppl_provision_label: Text,
  /// 範囲
  article_range: Option<Text>,
  /// 子要素
  children: Vec<TOCSupplProvisionContents>,
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
            if let Ok(v) = TOCArticle::parser(&node) {
              children.push(TOCSupplProvisionContents::TOCArticle(v));
            }
          }
          "TOCChapter" => {
            if let Ok(v) = TOCChapter::parser(&node) {
              children.push(TOCSupplProvisionContents::TOCChapter(v));
            }
          }
          _ => {}
        }
      }
      Ok(TOCSupplProvision {
        suppl_provision_label: label,
        article_range: range,
        children,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

/// 附則の目次の中身
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum TOCSupplProvisionContents {
  /// 条
  TOCArticle(TOCArticle),
  /// 章
  TOCChapter(TOCChapter),
}
