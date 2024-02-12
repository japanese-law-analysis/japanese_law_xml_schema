//! 条文とそれの階層構造
//!
use crate::class::*;
use crate::paragraph::*;
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
  fn new(title: Text, num: &str, delete: bool, hide: bool, children: Vec<PartContents>) -> Self {
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
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut children = node.children();
      let title_node = children
        .next()
        .ok_or(Error::missing_required_tag(&node.range(), "PartTitle"))?;
      let title_node_name = title_node.tag_name().name();
      let title = if "PartTitle" == title_node_name {
        Text::from_children(title_node.children())
      } else {
        return Err(Error::UnexpectedTag {
          range: title_node.range(),
          wrong_name: title_node_name.to_string(),
          tag: "PartTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Chapter" => {
            let v = Chapter::parser(&node)?;
            children_list.push(PartContents::Chapter(v))
          }
          "Article" => {
            let v = Article::parser(&node)?;
            children_list.push(PartContents::Article(v))
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(Part::new(title, &num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(node, "Part"))
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
  fn new(title: Text, num: &str, delete: bool, hide: bool, children: Vec<ChapterContents>) -> Self {
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
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut children = node.children();
      let title_node = children
        .next()
        .ok_or(Error::missing_required_tag(&node.range(), "ChapterTitle"))?;
      let title_node_name = title_node.tag_name().name();
      let title = if "ChapterTitle" == title_node_name {
        Text::from_children(title_node.children())
      } else {
        return Err(Error::UnexpectedTag {
          range: title_node.range(),
          wrong_name: title_node_name.to_string(),
          tag: "ChapterTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Section" => {
            let v = Section::parser(&node)?;
            children_list.push(ChapterContents::Section(v))
          }
          "Article" => {
            let v = Article::parser(&node)?;
            children_list.push(ChapterContents::Article(v))
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(Chapter::new(title, &num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(node, "Chapter"))
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
  fn new(title: Text, num: &str, delete: bool, hide: bool, children: Vec<SectionContents>) -> Self {
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
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut children = node.children();
      let title_node = children
        .next()
        .ok_or(Error::missing_required_tag(&node.range(), "SectionTitle"))?;
      let title_node_name = title_node.tag_name().name();
      let title = if "SectionTitle" == title_node_name {
        Text::from_children(title_node.children())
      } else {
        return Err(Error::UnexpectedTag {
          range: title_node.range(),
          wrong_name: title_node_name.to_string(),
          tag: "SectionTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Subsection" => {
            let v = Subsection::parser(&node)?;
            children_list.push(SectionContents::Subsection(v))
          }
          "Article" => {
            let v = Article::parser(&node)?;
            children_list.push(SectionContents::Article(v))
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(Section::new(title, &num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(node, "Section"))
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
  fn new(
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
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut children = node.children();
      let title_node = children.next().ok_or(Error::missing_required_tag(
        &node.range(),
        "SubsectionTitle",
      ))?;
      let title_node_name = title_node.tag_name().name();
      let title = if "SubsectionTitle" == title_node_name {
        Text::from_children(title_node.children())
      } else {
        return Err(Error::UnexpectedTag {
          range: title_node.range(),
          wrong_name: title_node_name.to_string(),
          tag: "SubsectionTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Division" => {
            let v = Division::parser(&node)?;
            children_list.push(SubsectionContents::Division(v))
          }
          "Article" => {
            let v = Article::parser(&node)?;
            children_list.push(SubsectionContents::Article(v))
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(Subsection::new(title, &num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(node, "Subsection"))
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
  fn new(title: Text, num: &str, delete: bool, hide: bool, children: Vec<Article>) -> Self {
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
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut children = node.children();
      let title_node = children
        .next()
        .ok_or(Error::missing_required_tag(&node.range(), "DivisionTitle"))?;
      let title_node_name = title_node.tag_name().name();
      let title = if "DivisionTitle" == title_node_name {
        Text::from_children(title_node.children())
      } else {
        return Err(Error::UnexpectedTag {
          range: title_node.range(),
          wrong_name: title_node_name.to_string(),
          tag: "DivisionTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        match node.tag_name().name() {
          "Article" => {
            let v = Article::parser(&node)?;
            children_list.push(v)
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(Division::new(title, &num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(node, "Division"))
    }
  }
}

/// 条
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Article {
  caption: Option<Caption>,
  title: Text,
  paragraph: Vec<Paragraph>,
  suppl_note: Option<Text>,
  num: String,
  delete: bool,
  hide: bool,
}

impl Article {
  fn new(
    title: Text,
    num: &str,
    delete: bool,
    hide: bool,
    caption: Option<Caption>,
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
    if node.tag_name().name() == "Article" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
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
            let v = Caption::parser(&next)?;
            caption = Some(v);
          }
          "ArticleTitle" => {
            title = Text::from_children(next.children());
          }
          s => return Err(Error::unexpected_tag(node, s)),
        }
      }
      let mut paragraph = Vec::new();
      let mut suppl_note = None;
      for node in children {
        match node.tag_name().name() {
          "Paragraph" => {
            let v = Paragraph::parser(&node)?;
            paragraph.push(v);
          }
          "SupplNote" => suppl_note = Some(Text::from_children(node.children())),

          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(Article::new(
        title, &num, delete, hide, caption, paragraph, suppl_note,
      ))
    } else {
      Err(Error::wrong_tag_name(node, "Article"))
    }
  }
}
