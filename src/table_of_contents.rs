//! 目次
use crate::class::*;
use crate::parser::*;
use crate::result::Error;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TOC" {
      let mut label = None;
      let mut preamble_label = None;
      let mut main_contents = Vec::new();
      let mut suppl_provision = None;
      let mut appdx_table_lable = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "TOCLabel" => {
              label = Some(Text::from_children(&e.children));
            }
            "TOCPreambleLabel" => {
              preamble_label = Some(Text::from_children(&e.children));
            }
            "TOCPart" => {
              let v = TOCPart::parser(e)?;
              main_contents.push(TOCMainContents::TOCPart(v));
            }
            "TOCChapter" => {
              let v = TOCChapter::parser(e)?;
              main_contents.push(TOCMainContents::TOCChapter(v));
            }
            "TOCSection" => {
              let v = TOCSection::parser(e)?;
              main_contents.push(TOCMainContents::TOCSection(v));
            }
            "TOCArticle" => {
              let v = TOCArticle::parser(e)?;
              main_contents.push(TOCMainContents::TOCArticle(v));
            }
            "TOCSupplProvision" => {
              let v = TOCSupplProvision::parser(e)?;
              suppl_provision = Some(v);
            }
            "TOCAppdxTableLabel" => {
              appdx_table_lable.push(Text::from_children(&e.children));
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
      Err(Error::wrong_tag_name(element, "TOC"))
    }
  }
}

impl ToXmlElement for TOC {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TOC");
    if let Some(label) = &self.toc_label {
      e.children
        .push(XMLNode::Element(label.to_xml_element_with_name("TOCLabel")));
    }
    if let Some(label) = &self.toc_preamble_label {
      e.children
        .push(XMLNode::Element(label.to_xml_element_with_name("TOCLabel")));
    }
    for v in self.toc_main_contents.iter() {
      match v {
        TOCMainContents::TOCPart(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TOCMainContents::TOCChapter(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TOCMainContents::TOCSection(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TOCMainContents::TOCArticle(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
      }
    }
    if let Some(v) = &self.toc_suppl_provision {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    for text in self.toc_appdx_table_label.iter() {
      e.children.push(XMLNode::Element(
        text.to_xml_element_with_name("TOCAppdxTableLabel"),
      ));
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TOCPart" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "PartTitle" => {
              title = Text::from_children(&e.children);
            }
            "ArticleRange" => {
              article_range = Some(Text::from_children(&e.children));
            }
            "TOCChapter" => {
              let v = TOCChapter::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
      Err(Error::wrong_tag_name(element, "TOCPart"))
    }
  }
}

impl ToXmlElement for TOCPart {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TOCPart");
    e.children.push(XMLNode::Element(
      self.part_title.to_xml_element_with_name("PartTitle"),
    ));
    if let Some(range) = &self.article_range {
      e.children.push(XMLNode::Element(
        range.to_xml_element_with_name("ArticleRange"),
      ));
    }
    for v in self.children.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    e.attributes.insert("Num".to_string(), self.num.clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TOCChapter" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ChapterTitle" => {
              title = Text::from_children(&e.children);
            }
            "ArticleRange" => {
              article_range = Some(Text::from_children(&e.children));
            }
            "TOCSection" => {
              let v = TOCSection::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
      Err(Error::wrong_tag_name(element, "TOCChapter"))
    }
  }
}

impl ToXmlElement for TOCChapter {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TOCChapter");
    e.children.push(XMLNode::Element(
      self.chapter_title.to_xml_element_with_name("ChapterTitle"),
    ));
    if let Some(range) = &self.article_range {
      e.children.push(XMLNode::Element(
        range.to_xml_element_with_name("ArticleRange"),
      ));
    }
    for v in self.children.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    e.attributes.insert("Num".to_string(), self.num.clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TOCSection" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "SectionTitle" => {
              title = Text::from_children(&e.children);
            }
            "ArticleRange" => {
              article_range = Some(Text::from_children(&e.children));
            }
            "TOCSubsection" => {
              let v = TOCSubsection::parser(e)?;
              children.push(TOCSectionContents::TOCSubsection(v));
            }
            "TOCDivision" => {
              let v = TOCDivision::parser(e)?;
              children.push(TOCSectionContents::TOCDivision(v));
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
      Err(Error::wrong_tag_name(element, "TOCSection"))
    }
  }
}

impl ToXmlElement for TOCSection {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TOCSection");
    e.children.push(XMLNode::Element(
      self.section_title.to_xml_element_with_name("SectionTitle"),
    ));
    if let Some(range) = &self.article_range {
      e.children.push(XMLNode::Element(
        range.to_xml_element_with_name("ArticleRange"),
      ));
    }
    for v in self.children.iter() {
      match v {
        TOCSectionContents::TOCDivision(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TOCSectionContents::TOCSubsection(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()))
        }
      }
    }
    e.attributes.insert("Num".to_string(), self.num.clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TOCSubsection" {
      let mut title = Text::new();
      let mut article_range = None;
      let mut children = Vec::new();
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "SubsectionTitle" => {
              title = Text::from_children(&e.children);
            }
            "ArticleRange" => {
              article_range = Some(Text::from_children(&e.children));
            }
            "TOCDivision" => {
              let v = TOCDivision::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
      Err(Error::wrong_tag_name(element, "TOCSubsection"))
    }
  }
}

impl ToXmlElement for TOCSubsection {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TOCSubsection");
    e.children.push(XMLNode::Element(
      self
        .subsection_title
        .to_xml_element_with_name("SubsectionTitle"),
    ));
    if let Some(range) = &self.article_range {
      e.children.push(XMLNode::Element(
        range.to_xml_element_with_name("ArticleRange"),
      ));
    }
    for v in self.children.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    e.attributes.insert("Num".to_string(), self.num.clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TOCDivision" {
      let mut title = Text::new();
      let mut article_range = None;
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "DivisionTitle" => {
              title = Text::from_children(&e.children);
            }
            "ArticleRange" => {
              article_range = Some(Text::from_children(&e.children));
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(TOCDivision {
        division_title: title,
        article_range,
        num,
        delete,
      })
    } else {
      Err(Error::wrong_tag_name(element, "TOCDivision"))
    }
  }
}

impl ToXmlElement for TOCDivision {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TOCDivision");
    e.children.push(XMLNode::Element(
      self
        .division_title
        .to_xml_element_with_name("DivisionTitle"),
    ));
    if let Some(range) = &self.article_range {
      e.children.push(XMLNode::Element(
        range.to_xml_element_with_name("ArticleRange"),
      ));
    }
    e.attributes.insert("Num".to_string(), self.num.clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TOCArticle" {
      let mut title = Text::new();
      let mut caption = None;
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ArticleTitle" => {
              title = Text::from_children(&e.children);
            }
            "ArticleCaption" => {
              let v = Caption::parser(e)?;
              caption = Some(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
          tag_name: "Caption".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "TOCArticle"))
    }
  }
}

impl ToXmlElement for TOCArticle {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TOCArticle");
    e.children.push(XMLNode::Element(
      self.article_title.to_xml_element_with_name("ArticleTitle"),
    ));
    e.children.push(XMLNode::Element(
      self
        .article_caption
        .to_xml_element_with_name("ArticleCaption"),
    ));
    e.attributes.insert("Num".to_string(), self.num.clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name == "TOCSupplProvision" {
      let mut label = Text::new();
      let mut range = None;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "SupplProvisionLabel" => {
              label = Text::from_children(&e.children);
            }
            "ArticleRange" => {
              range = Some(Text::from_children(&e.children));
            }
            "TOCArticle" => {
              let v = TOCArticle::parser(e)?;
              children.push(TOCSupplProvisionContents::TOCArticle(v));
            }
            "TOCChapter" => {
              let v = TOCChapter::parser(e)?;
              children.push(TOCSupplProvisionContents::TOCChapter(v));
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(TOCSupplProvision {
        suppl_provision_label: label,
        article_range: range,
        children,
      })
    } else {
      Err(Error::wrong_tag_name(element, "TOCSupplProbision"))
    }
  }
}

impl ToXmlElement for TOCSupplProvision {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TOCSupplProvision");
    e.children.push(XMLNode::Element(
      self
        .suppl_provision_label
        .to_xml_element_with_name("SupplProvisionLabel"),
    ));
    if let Some(range) = &self.article_range {
      e.children.push(XMLNode::Element(
        range.to_xml_element_with_name("ArticleRange"),
      ));
    }
    for v in self.children.iter() {
      match v {
        TOCSupplProvisionContents::TOCArticle(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        TOCSupplProvisionContents::TOCChapter(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
      }
    }
    e
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
