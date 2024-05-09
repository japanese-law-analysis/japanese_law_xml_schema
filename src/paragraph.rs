//! 段落
//!
use crate::article_number::*;
use crate::class::*;
use crate::law::*;
use crate::list::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
use crate::structs::*;
use crate::text::*;
use crate::to_xml::*;
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
  pub num: ArticleNumber,
  pub old_style: bool,
  pub old_num: bool,
  pub hide: bool,
}

impl Parser for Paragraph {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Paragraph" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
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
            "List" => {
              let v = List::parser(e)?;
              struct_list.push(Struct::List(v));
            }
            "Item" => {
              let v = Item::parser(e)?;
              children.push(v);
            }
            s => return Err(Error::unexpected_tag(element, s)),
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

impl ToXmlElement for Paragraph {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Paragraph");
    if let Some(caption) = &self.caption {
      e.children.push(XMLNode::Element(
        caption.to_xml_element_with_name("ParagraphCaption"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.paragraph_num.to_xml_element_with_name("ParagraphNum"),
    ));
    let mut se = Element::new("ParagraphSentence");
    se.children = self
      .sentence
      .iter()
      .map(|v| XMLNode::Element(v.to_xml_element()))
      .collect();
    e.children.push(XMLNode::Element(se));
    for v in self.amend_provision.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    for v in self.class.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    for v in self.children.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()));
    }
    e.attributes
      .insert("Num".to_string(), self.num.num_str().clone());
    if self.old_style {
      e.attributes
        .insert("OldStyle".to_string(), self.old_style.to_string());
    }
    if self.old_num {
      e.attributes
        .insert("OldNum".to_string(), self.old_num.to_string());
    }
    if self.hide {
      e.attributes
        .insert("Hide".to_string(), self.hide.to_string());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
  pub title: Option<Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem1>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Item {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem1>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Item {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Item {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Item" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Item"))
    }
  }
}

impl ToXmlElement for Item {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Item");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("ItemTitle"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("ItemSentence"),
    ));
    for n in &self.children {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem1 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem2>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem1 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem2>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem1 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem1 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem1" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem1Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem1"))
    }
  }
}

impl ToXmlElement for Subitem1 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem1");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem1Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem1Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem2 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem3>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem2 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem3>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem2 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem2 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem2" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem2Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem2"))
    }
  }
}

impl ToXmlElement for Subitem2 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem2");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem2Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem2Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem3 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem4>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem3 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem4>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem3 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem3 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem3" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem3Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem3"))
    }
  }
}

impl ToXmlElement for Subitem3 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem3");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem3Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem3Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem4 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem5>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem4 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem5>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem4 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem4 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem4" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem4Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem4"))
    }
  }
}

impl ToXmlElement for Subitem4 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem4");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem4Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem4Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem5 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem6>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem5 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem6>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem5 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem5 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem5" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem5Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem5"))
    }
  }
}

impl ToXmlElement for Subitem5 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem5");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem5Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem5Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem6 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem7>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem6 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem7>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem6 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem6 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem6" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem6Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem6"))
    }
  }
}

impl ToXmlElement for Subitem6 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem6");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem6Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem6Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem7 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem8>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem7 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem8>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem7 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem7 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem7" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem7Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem7"))
    }
  }
}

impl ToXmlElement for Subitem7 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem7");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem7Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem7Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem8 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem9>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem8 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem9>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem8 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem8 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem8" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem8Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem8"))
    }
  }
}

impl ToXmlElement for Subitem8 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem8");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem8Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem8Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem9 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub children: Vec<Subitem10>,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem9 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    children: Vec<Subitem10>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem9 {
      title,
      sentence,
      children,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem9 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem9" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem9Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem9"))
    }
  }
}

impl ToXmlElement for Subitem9 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem9");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem9Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem9Sentence"),
    ));
    for n in self.children.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()));
    }
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
pub struct Subitem10 {
  pub title: Option<text::Text>,
  pub sentence: SentenceOrColumnOrTable,
  pub struct_list: Vec<Struct>,
  pub num: ArticleNumber,
  pub delete: bool,
  pub hide: bool,
}

impl Subitem10 {
  fn new(
    title: Option<Text>,
    sentence: SentenceOrColumnOrTable,
    struct_list: Vec<Struct>,
    num: ArticleNumber,
    delete: bool,
    hide: bool,
  ) -> Self {
    Subitem10 {
      title,
      sentence,
      struct_list,
      num,
      delete,
      hide,
    }
  }
}

impl Parser for Subitem10 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Subitem10" {
      let num = ArticleNumber::from_num_str(&get_attribute_with_parse::<String>(element, "Num")?)?;
      let delete = get_attribute_opt_with_parse(element, "Delete")?.unwrap_or(false);
      let hide = get_attribute_opt_with_parse(element, "Hide")?.unwrap_or(false);
      let mut title = None;
      let mut sentence_opt = None;
      let mut struct_list = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Subitem10Title" => {
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
            s => return Err(Error::unexpected_tag(element, s)),
          }
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
        Err(Error::MissingRequiredTag {
          tag_name: "Sentence or Column or Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "Subitem10"))
    }
  }
}

impl ToXmlElement for Subitem10 {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Subitem10");
    if let Some(title) = &self.title {
      e.children.push(XMLNode::Element(
        title.to_xml_element_with_name("Subitem10Title"),
      ));
    }
    e.children.push(XMLNode::Element(
      self.sentence.to_xml_element_with_name("Subitem10Sentence"),
    ));
    for v in self.struct_list.iter() {
      match v {
        Struct::FigStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::TableStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::StyleStruct(v) => {
          e.children.push(XMLNode::Element(v.to_xml_element()));
        }
        Struct::List(v) => {
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
