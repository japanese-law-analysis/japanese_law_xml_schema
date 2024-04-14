//! 段落
//!
use crate::class::*;
use crate::law::*;
use crate::list::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
use crate::structs::*;
use crate::text::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Paragraph {
  pub caption: Option<Caption>,
  pub paragraph_num: Text,
  pub amend_provision: Vec<law::AmendProvision>,
  pub class: Vec<Class>,
  pub sentence: Vec<Sentence>,
  pub struct_list: Vec<Struct>,
  pub children: Vec<Item>,
  pub num: usize,
  pub old_style: bool,
  pub old_num: bool,
  pub hide: bool,
}

impl Parser for Paragraph {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Paragraph" {
      let num = get_attribute_with_parse(element, "Num")?;
      let old_style = get_attribute_opt_with_parse(element, "OldStyle")?.unwrap_or(false);
      let old_num = get_attribute_opt_with_parse(element, "OldNum")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut caption = None;
      let mut paragraph_num = Text::new();
      let mut sentence = Vec::new();
      let mut amend_provision = Vec::new();
      let mut class = Vec::new();
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ParagraphCaption" => {
              let v = Caption::parser(e)?;
              caption = Some(v);
            }
            "ParagraphNum" => {
              paragraph_num = Text::from_children(&e.children);
            }
            "ParagraphSentence" => {
              for node in e.children.iter() {
                if let XMLNode::Element(e) = node {
                  let v = Sentence::parser(e)?;
                  sentence.push(v);
                }
              }
            }
            "AmendProvision" => {
              let v = AmendProvision::parser(e)?;
              amend_provision.push(v);
            }
            "Class" => {
              let v = Class::parser(e)?;
              class.push(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "Item" => {
              let v = Item::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(Paragraph {
        caption,
        paragraph_num,
        amend_provision,
        class,
        sentence,
        struct_list,
        children,
        num,
        old_style,
        old_num,
        hide,
      })
    } else {
      Err(Error::wrong_tag_name(element, "Paragraph"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
  pub title: Option<Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem1>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Item {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem1>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Item {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Item {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Item" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "ItemSentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem1" => {
              let v = Subitem1::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Item::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Item"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem1 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem2>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem1 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem2>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem1 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem1 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem1" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem1Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem2" => {
              let v = Subitem2::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem1::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem1"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem2 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem3>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem2 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem3>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem2 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem2 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem2" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem2Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem3" => {
              let v = Subitem3::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem2::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem2"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem3 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem4>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem3 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem4>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem3 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem3 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem3" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem3Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem4" => {
              let v = Subitem4::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem3::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem3"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem4 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem5>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem4 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem5>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem4 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem4 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem4" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem4Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem5" => {
              let v = Subitem5::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem4::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem4"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem5 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem6>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem5 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem6>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem5 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem5 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem5" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem5Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem6" => {
              let v = Subitem6::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem5::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem5"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem6 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem7>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem6 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem7>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem6 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem6 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem6" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem6Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem7" => {
              let v = Subitem7::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem6::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem6"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem7 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem8>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem7 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem8>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem7 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem7 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem7" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem7Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem8" => {
              let v = Subitem8::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem7::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem7"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem8 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem9>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem8 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem9>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem8 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem8 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem8" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem8Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem9" => {
              let v = Subitem9::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem8::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem8"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem9 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem10>,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem9 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem10>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem9 {
      title,
      sentence,
      children,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem9 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem9" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem9Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Subitem10" => {
              let v = Subitem10::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem9::new(
          title,
          sentence,
          struct_list,
          children,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem9"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subitem10 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub struct_list: Vec<Struct>,
  pub num: String,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem10 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    num: &str,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem10 {
      title,
      sentence,
      struct_list,
      num: num.to_string(),
      delete,
      hide,
    }
  }
}

impl Parser for Subitem10 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem10" {
      let num = get_attribute(element, "Num")?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "ItemTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Subitem10Sentence" => {
              let v = SentenceOrColumnOrTable::from_element(e)?;
              sentence_opt = Some(v);
            }
            "TableStruct" => {
              let v = TableStruct::parser(e)?;
              struct_list.push(Struct::TableStruct(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              struct_list.push(Struct::FigStruct(v));
            }
            "StyleStruct" => {
              let v = StyleStruct::parser(e)?;
              struct_list.push(Struct::StyleStruct(v));
            }
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(sentence) = sentence_opt {
        Ok(Subitem10::new(
          title,
          sentence,
          struct_list,
          &num,
          delete,
          hide,
        ))
      } else {
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem10"))
    }
  }
}
