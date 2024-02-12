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
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Paragraph {
  caption: Option<Caption>,
  paragraph_num: Text,
  amend_provision: Vec<law::AmendProvision>,
  class: Vec<Class>,
  sentence: Vec<Sentence>,
  struct_list: Vec<Struct>,
  children: Vec<Item>,
  num: usize,
  old_style: bool,
  old_num: bool,
  hide: bool,
}

impl Parser for Paragraph {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Paragraph" {
      let num = get_attribute_with_parse(node, "Num")?;
      let old_style = get_attribute_opt_with_parse(node, "OldStyle")?.unwrap_or(false);
      let old_num = get_attribute_opt_with_parse(node, "OldNum")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut caption = None;
      let mut paragraph_num = Text::new();
      let mut sentence = Vec::new();
      let mut amend_provision = Vec::new();
      let mut class = Vec::new();
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ParagraphCaption" => {
            let v = Caption::parser(&node)?;
            caption = Some(v);
          }
          "ParagraphNum" => {
            paragraph_num = Text::from_children(node.children());
          }
          "ParagraphSentence" => {
            for node in node.children() {
              let v = Sentence::parser(&node)?;
              sentence.push(v);
            }
          }
          "AmendProvision" => {
            let v = AmendProvision::parser(&node)?;
            amend_provision.push(v);
          }
          "Class" => {
            let v = Class::parser(&node)?;
            class.push(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "Item" => {
            let v = Item::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
      Err(Error::wrong_tag_name(node, "Paragraph"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Item {
  title: Option<Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem1>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Item" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "ItemSentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem1" => {
            let v = Subitem1::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Item"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem1 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem2>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem1" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem1Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem2" => {
            let v = Subitem2::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem1"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem2 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem3>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem2" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem2Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem3" => {
            let v = Subitem3::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem2"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem3 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem4>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem3" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem3Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem4" => {
            let v = Subitem4::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem3"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem4 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem5>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem4" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem4Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem5" => {
            let v = Subitem5::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem4"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem5 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem6>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem5" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem5Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem6" => {
            let v = Subitem6::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem5"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem6 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem7>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem6" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem6Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem7" => {
            let v = Subitem7::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem6"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem7 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem8>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem7" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem7Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem8" => {
            let v = Subitem8::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem7"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem8 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem9>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem8" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem8Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem9" => {
            let v = Subitem9::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem8"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem9 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem10>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem9" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem9Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          "Subitem10" => {
            let v = Subitem10::parser(&node)?;
            children.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem9"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem10 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Subitem10" {
      let num = get_attribute(node, "Num")?;
      let delete = get_attribute_opt_with_parse(node, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(node, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "ItemTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Subitem10Sentence" => {
            let v = SentenceOrColumnOrTable::from_node(&node)?;
            sentence_opt = Some(v);
          }
          "TableStruct" => {
            let v = TableStruct::parser(&node)?;
            struct_list.push(Struct::TableStruct(v));
          }
          "FigStruct" => {
            let v = FigStruct::parser(&node)?;
            struct_list.push(Struct::FigStruct(v));
          }
          "StyleStruct" => {
            let v = StyleStruct::parser(&node)?;
            struct_list.push(Struct::StyleStruct(v));
          }
          "List" => {
            let v = List::parser(&node)?;
            struct_list.push(Struct::List(v));
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
          range: node.range(),
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "Subitem10"))
    }
  }
}
