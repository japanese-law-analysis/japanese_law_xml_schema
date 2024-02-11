//! 段落
//!
use crate::law::*;
use crate::list::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
use crate::structs::*;
use crate::table::*;
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
    let num = node
      .attribute("Num")
      .and_then(|s| s.parse::<usize>().ok())
      .ok_or(Error::Attribute)?;
    let old_style = node
      .attribute("OldStyle")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let old_num = node
      .attribute("OldNum")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          if let Ok(v) = Caption::parser(&node) {
            caption = Some(v);
          }
        }
        "ParagraphNum" => {
          paragraph_num = Text::from_children(node.children());
        }
        "ParagraphSentence" => {
          for node in node.children() {
            if node.tag_name().name() == "Sentence" {
              if let Ok(v) = Sentence::parser(&node) {
                sentence.push(v);
              }
            }
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
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "Item" => {
          if let Ok(v) = Item::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem1" => {
          if let Ok(v) = Subitem1::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Item::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem2" => {
          if let Ok(v) = Subitem2::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem1::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem3" => {
          if let Ok(v) = Subitem3::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem2::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem4" => {
          if let Ok(v) = Subitem4::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem3::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem5" => {
          if let Ok(v) = Subitem5::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem4::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem6" => {
          if let Ok(v) = Subitem6::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem5::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem7" => {
          if let Ok(v) = Subitem7::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem6::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem8" => {
          if let Ok(v) = Subitem8::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem7::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem9" => {
          if let Ok(v) = Subitem9::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem8::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
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
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        "Subitem10" => {
          if let Ok(v) = Subitem10::parser(&node) {
            children.push(v);
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem9::new(
        title,
        sentence,
        struct_list,
        children,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
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
  pub fn new(
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
    let num = node.attribute("Num").ok_or(Error::Attribute)?;
    let delete = node
      .attribute("Delete")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let hide = node
      .attribute("Hide")
      .and_then(|s| s.parse::<bool>().ok())
      .unwrap_or(false);
    let mut title = None;
    let mut sentence_opt = None;
    let mut struct_list = Vec::new();
    for node in node.children() {
      match node.tag_name().name() {
        "ItemTitle" => {
          title = Some(Text::from_children(node.children()));
        }
        "Subitem10Sentence" => {
          let mut sentence_list = Vec::new();
          let mut column_list = Vec::new();
          for node in node.children() {
            match node.tag_name().name() {
              "Sentence" => {
                if let Ok(v) = Sentence::parser(&node) {
                  sentence_list.push(v);
                }
              }
              "Column" => {
                if let Ok(v) = Column::parser(&node) {
                  column_list.push(v);
                }
              }
              "Table" => {
                if let Ok(v) = Table::parser(&node) {
                  sentence_opt = Some(SentenceOrColumnOrTable::Table(v));
                }
              }
              _ => {}
            }
            if !sentence_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Sentence(sentence_list.clone()));
            }
            if !column_list.is_empty() {
              sentence_opt = Some(SentenceOrColumnOrTable::Column(column_list.clone()));
            }
          }
        }
        "TableStruct" => {
          if let Ok(v) = TableStruct::parser(&node) {
            struct_list.push(Struct::TableStruct(v));
          }
        }
        "FigStruct" => {
          if let Ok(v) = FigStruct::parser(&node) {
            struct_list.push(Struct::FigStruct(v));
          }
        }
        "StyleStruct" => {
          if let Ok(v) = StyleStruct::parser(&node) {
            struct_list.push(Struct::StyleStruct(v));
          }
        }
        "List" => {
          if let Ok(v) = List::parser(&node) {
            struct_list.push(Struct::List(v));
          }
        }
        _ => {}
      }
    }
    if let Some(sentence) = sentence_opt {
      Ok(Subitem10::new(
        title,
        sentence,
        struct_list,
        num,
        delete,
        hide,
      ))
    } else {
      Err(Error::Tag)
    }
  }
}
