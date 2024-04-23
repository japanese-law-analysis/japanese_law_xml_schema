//! 条文とそれの階層構造
//!
use crate::article_number::*;
use crate::class::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::result::Error;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

/// 編
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Part {
  pub part_title: Text,
  pub children: Vec<PartContents>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Part {
  fn new(
    title: Text,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
    children: Vec<PartContents>,
  ) -> Self {
    Part {
      part_title: title,
      children,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Part {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Part" {
      let num_str = get_attribute(element, "Num")?;
      let num = ArticleNumber::from_num_str(&num_str)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut children = element.children.iter();
      let title_element = children
        .next()
        .and_then(|n| {
          if let XMLNode::Element(e) = n {
            Some(e)
          } else {
            None
          }
        })
        .ok_or(Error::missing_required_tag("PartTitle"))?;
      let title_element_name = title_element.name.clone();
      let title = if "PartTitle" == title_element_name.as_str() {
        Text::from_children(&title_element.children)
      } else {
        return Err(Error::UnexpectedTag {
          wrong_name: title_element_name,
          tag: "PartTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Chapter" => {
              let v = Chapter::parser(e)?;
              children_list.push(PartContents::Chapter(v))
            }
            "Article" => {
              let v = Article::parser(e)?;
              children_list.push(PartContents::Article(v))
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(Part::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(element, "Part"))
    }
  }
}

impl ToXmlElement for Part {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Part");
    e.children.push(XMLNode::Element(
      self.part_title.to_xml_element_with_name("PartTitle"),
    ));
    for v in self.children.iter() {
      match v {
        PartContents::Article(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        PartContents::Chapter(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
      }
    }
    e.attributes
      .insert("Num".to_string(), self.num.num_str().clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    if self.hide {
      e.attributes
        .insert("Hide".to_string(), self.hide.to_string());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum PartContents {
  Article(Article),
  Chapter(Chapter),
}

/// 章
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chapter {
  pub chapter_title: Text,
  pub children: Vec<ChapterContents>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Chapter {
  fn new(
    title: Text,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
    children: Vec<ChapterContents>,
  ) -> Self {
    Chapter {
      chapter_title: title,
      children,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Chapter {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Chapter" {
      let num_str = get_attribute(element, "Num")?;
      let num = ArticleNumber::from_num_str(&num_str)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut children = element.children.iter();
      let title_element = children
        .next()
        .and_then(|n| {
          if let XMLNode::Element(e) = n {
            Some(e)
          } else {
            None
          }
        })
        .ok_or(Error::missing_required_tag("ChapterTitle"))?;
      let title_element_name = title_element.name.clone();
      let title = if "ChapterTitle" == title_element_name.as_str() {
        Text::from_children(&title_element.children)
      } else {
        return Err(Error::UnexpectedTag {
          wrong_name: title_element_name,
          tag: "ChapterTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Section" => {
              let v = Section::parser(e)?;
              children_list.push(ChapterContents::Section(v))
            }
            "Article" => {
              let v = Article::parser(e)?;
              children_list.push(ChapterContents::Article(v))
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(Chapter::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(element, "Chapter"))
    }
  }
}

impl ToXmlElement for Chapter {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Chapter");
    e.children.push(XMLNode::Element(
      self.chapter_title.to_xml_element_with_name("ChapterTitle"),
    ));
    for v in self.children.iter() {
      match v {
        ChapterContents::Article(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        ChapterContents::Section(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
      }
    }
    e.attributes
      .insert("Num".to_string(), self.num.num_str().clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    if self.hide {
      e.attributes
        .insert("Hide".to_string(), self.hide.to_string());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChapterContents {
  Article(Article),
  Section(Section),
}

/// 節
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Section {
  pub section_title: Text,
  pub children: Vec<SectionContents>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Section {
  fn new(
    title: Text,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
    children: Vec<SectionContents>,
  ) -> Self {
    Section {
      section_title: title,
      children,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Section {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Section" {
      let num_str = get_attribute(element, "Num")?;
      let num = ArticleNumber::from_num_str(&num_str)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut children = element.children.iter();
      let title_element = children
        .next()
        .and_then(|n| {
          if let XMLNode::Element(e) = n {
            Some(e)
          } else {
            None
          }
        })
        .ok_or(Error::missing_required_tag("SectionTitle"))?;
      let title_element_name = title_element.name.clone();
      let title = if "SectionTitle" == title_element_name.as_str() {
        Text::from_children(&title_element.children)
      } else {
        return Err(Error::UnexpectedTag {
          wrong_name: title_element_name,
          tag: "SectionTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subsection" => {
              let v = Subsection::parser(e)?;
              children_list.push(SectionContents::Subsection(v))
            }
            "Article" => {
              let v = Article::parser(e)?;
              children_list.push(SectionContents::Article(v))
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(Section::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(element, "Section"))
    }
  }
}

impl ToXmlElement for Section {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Section");
    e.children.push(XMLNode::Element(
      self.section_title.to_xml_element_with_name("SectionTitle"),
    ));
    for v in self.children.iter() {
      match v {
        SectionContents::Article(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        SectionContents::Subsection(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
      }
    }
    e.attributes
      .insert("Num".to_string(), self.num.num_str().clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    if self.hide {
      e.attributes
        .insert("Hide".to_string(), self.hide.to_string());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SectionContents {
  Article(Article),
  Subsection(Subsection),
}

/// 款
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subsection {
  pub subsection_title: Text,
  pub children: Vec<SubsectionContents>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subsection {
  fn new(
    title: Text,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
    children: Vec<SubsectionContents>,
  ) -> Self {
    Subsection {
      subsection_title: title,
      children,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subsection {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subsection" {
      let num_str = get_attribute(element, "Num")?;
      let num = ArticleNumber::from_num_str(&num_str)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut children = element.children.iter();
      let title_element = children
        .next()
        .and_then(|n| {
          if let XMLNode::Element(e) = n {
            Some(e)
          } else {
            None
          }
        })
        .ok_or(Error::missing_required_tag("SubsectionTitle"))?;
      let title_element_name = title_element.name.clone();
      let title = if "SubsectionTitle" == title_element_name.as_str() {
        Text::from_children(&title_element.children)
      } else {
        return Err(Error::UnexpectedTag {
          wrong_name: title_element_name,
          tag: "SubsectionTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Division" => {
              let v = Division::parser(e)?;
              children_list.push(SubsectionContents::Division(v))
            }
            "Article" => {
              let v = Article::parser(e)?;
              children_list.push(SubsectionContents::Article(v))
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(Subsection::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(element, "Subsection"))
    }
  }
}

impl ToXmlElement for Subsection {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subsection");
    e.children.push(XMLNode::Element(
      self
        .subsection_title
        .to_xml_element_with_name("SubsectionTitle"),
    ));
    for v in self.children.iter() {
      match v {
        SubsectionContents::Article(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        SubsectionContents::Division(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
      }
    }
    e.attributes
      .insert("Num".to_string(), self.num.num_str().clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    if self.hide {
      e.attributes
        .insert("Hide".to_string(), self.hide.to_string());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubsectionContents {
  Article(Article),
  Division(Division),
}

/// 目
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Division {
  pub division_title: Text,
  pub children: Vec<Article>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Division {
  fn new(
    title: Text,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
    children: Vec<Article>,
  ) -> Self {
    Division {
      division_title: title,
      children,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Division {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Division" {
      let num_str = get_attribute(element, "Num")?;
      let num = ArticleNumber::from_num_str(&num_str)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut children = element.children.iter();
      let title_element = children
        .next()
        .and_then(|n| {
          if let XMLNode::Element(e) = n {
            Some(e)
          } else {
            None
          }
        })
        .ok_or(Error::missing_required_tag("DivisionTitle"))?;
      let title_element_name = title_element.name.clone();
      let title = if "DivisionTitle" == title_element_name.as_str() {
        Text::from_children(&title_element.children)
      } else {
        return Err(Error::UnexpectedTag {
          wrong_name: title_element_name,
          tag: "DivisionTitle".to_string(),
        });
      };
      let mut children_list = Vec::new();
      for node in children {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Article" => {
              let v = Article::parser(e)?;
              children_list.push(v)
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(Division::new(title, num, delete, hide, children_list))
    } else {
      Err(Error::wrong_tag_name(element, "Division"))
    }
  }
}

impl ToXmlElement for Division {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Division");
    e.children.push(XMLNode::Element(
      self
        .division_title
        .to_xml_element_with_name("DivisionTitle"),
    ));
    for v in self.children.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    e.attributes
      .insert("Num".to_string(), self.num.num_str().clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    if self.hide {
      e.attributes
        .insert("Hide".to_string(), self.hide.to_string());
    }
    e
  }
}

/// 条
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Article {
  pub caption: Option<Caption>,
  pub title: Text,
  pub paragraph: Vec<Paragraph>,
  pub suppl_note: Option<Text>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Article {
  fn new(
    title: Text,
    num: ArticleNumber,
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
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Article {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Article" {
      let num_str = get_attribute(element, "Num")?;
      let num = ArticleNumber::from_num_str(&num_str)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut children = element.children.iter().peekable();
      let mut caption = None;
      let mut title = Text::new();
      loop {
        if let Some(XMLNode::Element(e)) = children.peek() {
          let tag_name = e.name.as_str();
          if tag_name != "ArticleCaption" && tag_name != "ArticleTitle" {
            break;
          }
        }
        let next = children.next().unwrap();
        if let XMLNode::Element(e) = next {
          match e.name.as_str() {
            "ArticleCaption" => {
              let v = Caption::parser(e)?;
              caption = Some(v);
            }
            "ArticleTitle" => {
              title = Text::from_children(&e.children);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      let mut paragraph = Vec::new();
      let mut suppl_note = None;
      for node in children {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Paragraph" => {
              let v = Paragraph::parser(e)?;
              paragraph.push(v);
            }
            "SupplNote" => suppl_note = Some(Text::from_children(&e.children)),

            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(Article::new(
        title, num, delete, hide, caption, paragraph, suppl_note,
      ))
    } else {
      Err(Error::wrong_tag_name(element, "Article"))
    }
  }
}

impl ToXmlElement for Article {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Article");
    if let Some(caption) = &self.caption {
      e.children.push(XMLNode::Element(
        caption.to_xml_element_with_name("ArticleCaption"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.title.to_xml_element_with_name("ArticleTitle"),
    ));
    for v in self.paragraph.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    if let Some(suppl_note) = &self.suppl_note {
      e.children.push(XMLNode::Element(
        suppl_note.to_xml_element_with_name("SupplNote"),
      ));
    }
    e.attributes
      .insert("Num".to_string(), self.num.num_str().clone());
    if self.delete {
      e.attributes
        .insert("Delete".to_string(), self.delete.to_string());
    }
    if self.hide {
      e.attributes
        .insert("Hide".to_string(), self.hide.to_string());
    }
    e
  }
}
