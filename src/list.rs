//! 列挙

use crate::class::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

/// 列挙
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct List {
  pub sentence: Vec<ListSentence>,
  pub children: Vec<Sublist1>,
}

impl List {
  fn new(sentence: Vec<ListSentence>, children: Vec<Sublist1>) -> Self {
    List { sentence, children }
  }
}

impl Parser for List {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "List" {
      let mut children = element.children.iter();
      let list_sentence = children.next();
      let mut sentence = Vec::new();
      if let Some("ListSentence") = list_sentence.and_then(|node| {
        if let XMLNode::Element(e) = node {
          Some(e.name.as_str())
        } else {
          None
        }
      }) {
        if let XMLNode::Element(e) = list_sentence.unwrap() {
          for node in e.children.iter() {
            if let XMLNode::Element(e) = node {
              match e.name.as_str() {
                "Sentence" => {
                  let v = Sentence::parser(e)?;
                  sentence.push(ListSentence::Sentence(v))
                }
                "Column" => {
                  let v = Column::parser(e)?;
                  sentence.push(ListSentence::Column(v))
                }
                s => return Err(Error::unexpected_tag(e, s)),
              }
            }
          }
        }
      } else {
        return Err(Error::wrong_tag_name(element, "LiseSentence"));
      }
      let mut sublist = Vec::new();
      for node in children {
        if let XMLNode::Element(e) = node {
          let v = Sublist1::parser(e)?;
          sublist.push(v)
        }
      }
      Ok(List::new(sentence, sublist))
    } else {
      Err(Error::wrong_tag_name(element, "List"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListSentence {
  Sentence(Sentence),
  Column(Column),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sublist1 {
  pub sentence: Vec<ListSentence>,
  pub children: Vec<Sublist2>,
}

impl Sublist1 {
  fn new(sentence: Vec<ListSentence>, children: Vec<Sublist2>) -> Self {
    Sublist1 { sentence, children }
  }
}

impl Parser for Sublist1 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Sublist1" {
      let mut children = element.children.iter();
      let list_sentence = children.next();
      let mut sentence = Vec::new();
      if let Some("Sublist1Sentence") = list_sentence.and_then(|node| {
        if let XMLNode::Element(e) = node {
          Some(e.name.as_str())
        } else {
          None
        }
      }) {
        if let XMLNode::Element(e) = list_sentence.unwrap() {
          for node in e.children.iter() {
            if let XMLNode::Element(e) = node {
              match e.name.as_str() {
                "Sentence" => {
                  let v = Sentence::parser(e)?;
                  sentence.push(ListSentence::Sentence(v))
                }
                "Column" => {
                  let v = Column::parser(e)?;
                  sentence.push(ListSentence::Column(v))
                }
                s => return Err(Error::unexpected_tag(e, s)),
              }
            }
          }
        }
      } else {
        return Err(Error::wrong_tag_name(element, "Sublist1Sentence"));
      }
      let mut sublist = Vec::new();
      for node in children {
        if let XMLNode::Element(e) = node {
          let v = Sublist2::parser(e)?;
          sublist.push(v)
        }
      }
      Ok(Sublist1::new(sentence, sublist))
    } else {
      Err(Error::wrong_tag_name(element, "Sublist1"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sublist2 {
  pub sentence: Vec<ListSentence>,
  pub children: Vec<Sublist3>,
}

impl Sublist2 {
  fn new(sentence: Vec<ListSentence>, children: Vec<Sublist3>) -> Self {
    Sublist2 { sentence, children }
  }
}

impl Parser for Sublist2 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Sublist2" {
      let mut children = element.children.iter();
      let list_sentence = children.next();
      let mut sentence = Vec::new();
      if let Some("Sublist2Sentence") = list_sentence.and_then(|node| {
        if let XMLNode::Element(e) = node {
          Some(e.name.as_str())
        } else {
          None
        }
      }) {
        if let XMLNode::Element(e) = list_sentence.unwrap() {
          for node in e.children.iter() {
            if let XMLNode::Element(e) = node {
              match e.name.as_str() {
                "Sentence" => {
                  let v = Sentence::parser(e)?;
                  sentence.push(ListSentence::Sentence(v))
                }
                "Column" => {
                  let v = Column::parser(e)?;
                  sentence.push(ListSentence::Column(v))
                }
                s => return Err(Error::unexpected_tag(e, s)),
              }
            }
          }
        }
      } else {
        return Err(Error::wrong_tag_name(element, "Sublist2Sentence"));
      }
      let mut sublist = Vec::new();
      for node in children {
        if let XMLNode::Element(e) = node {
          let v = Sublist3::parser(e)?;
          sublist.push(v)
        }
      }
      Ok(Sublist2::new(sentence, sublist))
    } else {
      Err(Error::wrong_tag_name(element, "Sublist2"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sublist3 {
  pub sentence: Vec<ListSentence>,
}

impl Sublist3 {
  fn new(sentence: Vec<ListSentence>) -> Self {
    Sublist3 { sentence }
  }
}

impl Parser for Sublist3 {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Sublist3" {
      let mut children = element.children.iter();
      let list_sentence = children.next();
      let mut sentence = Vec::new();
      if let Some("Sublist3Sentence") = list_sentence.and_then(|node| {
        if let XMLNode::Element(e) = node {
          Some(e.name.as_str())
        } else {
          None
        }
      }) {
        if let XMLNode::Element(e) = list_sentence.unwrap() {
          for node in e.children.iter() {
            if let XMLNode::Element(e) = node {
              match e.name.as_str() {
                "Sentence" => {
                  let v = Sentence::parser(e)?;
                  sentence.push(ListSentence::Sentence(v))
                }
                "Column" => {
                  let v = Column::parser(e)?;
                  sentence.push(ListSentence::Column(v))
                }
                s => return Err(Error::unexpected_tag(e, s)),
              }
            }
          }
        }
      } else {
        return Err(Error::wrong_tag_name(element, "Sublist3Sentence"));
      }
      Ok(Sublist3::new(sentence))
    } else {
      Err(Error::wrong_tag_name(element, "Sublist3"))
    }
  }
}
