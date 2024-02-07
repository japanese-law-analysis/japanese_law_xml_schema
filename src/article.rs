//! 条文とそれの階層構造
//!
use crate::parser::*;
use crate::result::Error;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

/// 編
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Part {
  part_title: Text,
  children: Vec<PartContents>,
  num: String,
  delete: bool,
  hide: bool,
}

impl Part {
  pub fn new(
    title: Text,
    num: &str,
    delete: bool,
    hide: bool,
    children: Vec<PartContents>,
  ) -> Self {
    Part {
      part_title: title,
      children,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Part {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Part" {
      let num = node.attribute("Num").ok_or(Error::Attribute)?;
      let delete = node
        .attribute("Delete")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let hide = node
        .attribute("Hide")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let mut children = node.children();
      let title_node = children.next();
      let mut title = Text::new();
      if let Some("PartTitle") = title_node.map(|node| node.tag_name().name()) {
        title = Text::from_children(title_node.unwrap().children());
      } else {
        return Err(Error::Tag);
      }
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Chapter" => {
            if let Ok(c) = Chapter::parser(&node) {
              children_list.push(PartContents::Chapter(c))
            }
          }
          "Article" => {
            if let Ok(a) = Article::parser(&node) {
              children_list.push(PartContents::Article(a))
            }
          }
          _ => return Err(Error::Tag),
        }
      }
      Ok(Part::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum PartContents {
  Article(Article),
  Chapter(Chapter),
}

/// 章
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Chapter {
  chapter_title: Text,
  children: Vec<ChapterContents>,
  num: String,
  delete: bool,
  hide: bool,
}

impl Chapter {
  pub fn new(
    title: Text,
    num: &str,
    delete: bool,
    hide: bool,
    children: Vec<ChapterContents>,
  ) -> Self {
    Chapter {
      chapter_title: title,
      children,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Chapter {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Chapter" {
      let num = node.attribute("Num").ok_or(Error::Attribute)?;
      let delete = node
        .attribute("Delete")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let hide = node
        .attribute("Hide")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let mut children = node.children();
      let title_node = children.next();
      let mut title = Text::new();
      if let Some("ChapterTitle") = title_node.map(|node| node.tag_name().name()) {
        title = Text::from_children(title_node.unwrap().children());
      } else {
        return Err(Error::Tag);
      }
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Section" => {
            if let Ok(s) = Section::parser(&node) {
              children_list.push(ChapterContents::Section(s))
            }
          }
          "Article" => {
            if let Ok(a) = Article::parser(&node) {
              children_list.push(ChapterContents::Article(a))
            }
          }
          _ => return Err(Error::Tag),
        }
      }
      Ok(Chapter::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum ChapterContents {
  Article(Article),
  Section(Section),
}

/// 節
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Section {
  section_title: Text,
  children: Vec<SectionContents>,
  num: String,
  delete: bool,
  hide: bool,
}

impl Section {
  pub fn new(
    title: Text,
    num: &str,
    delete: bool,
    hide: bool,
    children: Vec<SectionContents>,
  ) -> Self {
    Section {
      section_title: title,
      children,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Section {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Section" {
      let num = node.attribute("Num").ok_or(Error::Attribute)?;
      let delete = node
        .attribute("Delete")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let hide = node
        .attribute("Hide")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let mut children = node.children();
      let title_node = children.next();
      let mut title = Text::new();
      if let Some("SectionTitle") = title_node.map(|node| node.tag_name().name()) {
        title = Text::from_children(title_node.unwrap().children());
      } else {
        return Err(Error::Tag);
      }
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Subsection" => {
            if let Ok(s) = Subsection::parser(&node) {
              children_list.push(SectionContents::Subsection(s))
            }
          }
          "Article" => {
            if let Ok(a) = Article::parser(&node) {
              children_list.push(SectionContents::Article(a))
            }
          }
          _ => return Err(Error::Tag),
        }
      }
      Ok(Section::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SectionContents {
  Article(Article),
  Subsection(Subsection),
}

/// 款
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subsection {
  subsection_title: Text,
  children: Vec<SubsectionContents>,
  num: String,
  delete: bool,
  hide: bool,
}

impl Subsection {
  pub fn new(
    title: Text,
    num: &str,
    delete: bool,
    hide: bool,
    children: Vec<SubsectionContents>,
  ) -> Self {
    Subsection {
      subsection_title: title,
      children,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subsection {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subsection" {
      let num = node.attribute("Num").ok_or(Error::Attribute)?;
      let delete = node
        .attribute("Delete")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let hide = node
        .attribute("Hide")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let mut children = node.children();
      let title_node = children.next();
      let mut title = Text::new();
      if let Some("SubsectionTitle") = title_node.map(|node| node.tag_name().name()) {
        title = Text::from_children(title_node.unwrap().children());
      } else {
        return Err(Error::Tag);
      }
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Division" => {
            if let Ok(d) = Division::parser(&node) {
              children_list.push(SubsectionContents::Division(d))
            }
          }
          "Article" => {
            if let Ok(a) = Article::parser(&node) {
              children_list.push(SubsectionContents::Article(a))
            }
          }
          _ => return Err(Error::Tag),
        }
      }
      Ok(Subsection::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SubsectionContents {
  Article(Article),
  Division(Division),
}

/// 目
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Division {
  division_title: Text,
  children: Vec<Article>,
  num: String,
  delete: bool,
  hide: bool,
}

impl Division {
  pub fn new(title: Text, num: &str, delete: bool, hide: bool, children: Vec<Article>) -> Self {
    Division {
      division_title: title,
      children,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Division {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Division" {
      let num = node.attribute("Num").ok_or(Error::Attribute)?;
      let delete = node
        .attribute("Delete")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let hide = node
        .attribute("Hide")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let mut children = node.children();
      let title_node = children.next();
      let mut title = Text::new();
      if let Some("DivisionTitle") = title_node.map(|node| node.tag_name().name()) {
        title = Text::from_children(title_node.unwrap().children());
      } else {
        return Err(Error::Tag);
      }
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Article" => {
            if let Ok(a) = Article::parser(&node) {
              children_list.push(a)
            }
          }
          _ => return Err(Error::Tag),
        }
      }
      Ok(Division::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::Tag)
    }
  }
}

/// 条
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Article {
  caption: Option<ArticleCaption>,
  title: Text,
  paragraph: Vec<Paragraph>,
  suppl_note: Option<Text>,
  num: String,
  delete: bool,
  hide: bool,
}

impl Article {
  pub fn new(
    title: Text,
    num: &str,
    delete: bool,
    hide: bool,
    caption: Option<ArticleCaption>,
    paragraph: Vec<Paragraph>,
    suppl_note: Option<Text>,
  ) -> Self {
    Article {
      caption,
      title,
      paragraph,
      suppl_note,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Article {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Part" {
      let num = node.attribute("Num").ok_or(Error::Attribute)?;
      let delete = node
        .attribute("Delete")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let hide = node
        .attribute("Hide")
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false);
      let mut children = node.children().peekable();
      let mut caption = None;
      let mut title = Text::new();
      loop {
        if let Some(node) = children.peek() {
          let tag_name = node.tag_name().name();
          if tag_name != "ArticleCaption" && tag_name != "ArticleTitle" {
            break;
          }
        }
        let next = children.next().unwrap();
        match next.tag_name().name() {
          "ArticleCaption" => {
            let text = Text::from_children(next.children());
            let common_caption = next
              .attribute("CommonCaption")
              .and_then(|s| s.parse::<bool>().ok());
            caption = Some(ArticleCaption {
              text,
              common_caption,
            });
          }
          "ArticleTitle" => {
            title = Text::from_children(next.children());
          }
          _ => return Err(Error::Tag),
        }
      }
      let mut paragraph = Vec::new();
      let mut suppl_note = None;
      for node in children {
        match node.tag_name().name() {
          "Paragraph" => {
            if let Ok(p) = Paragraph::parser(&node) {
              paragraph.push(p);
            }
          }
          "SupplNote" => suppl_note = Some(Text::from_children(node.children())),
          _ => return Err(Error::Tag),
        }
      }
      Ok(Article::new(
        title, num, delete, hide, caption, paragraph, suppl_note,
      ))
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ArticleCaption {
  pub text: text::Text,
  pub common_caption: Option<bool>,
}
