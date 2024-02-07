//! 列挙に関する

use crate::parser::*;
use crate::result::Error;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

/// 列挙
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct List {
  sentence: Vec<ListSentence>,
  children: Vec<Sublist1>,
}

impl List {
  pub fn new(sentence: Vec<ListSentence>, children: Vec<Sublist1>) -> Self {
    List { sentence, children }
  }
}

impl Parser for List {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "List" {
      let mut children = node.children();
      let list_sentence = children.next();
      let mut sentence = Vec::new();
      if let Some("ListSentence") = list_sentence.map(|node| node.tag_name().name()) {
        for node in list_sentence.unwrap().children() {
          match node.tag_name().name() {
            "Sentence" => {
              if let Ok(s) = Sentence::parser(&node) {
                sentence.push(ListSentence::Sentence(s))
              }
            }
            "Column" => {
              if let Ok(c) = Column::parser(&node) {
                sentence.push(ListSentence::Column(c))
              }
            }
            _ => return Err(Error::Tag),
          }
        }
      } else {
        return Err(Error::Tag);
      }
      let mut sublist = Vec::new();
      for node in children {
        if let Ok(sub) = Sublist1::parser(&node) {
          sublist.push(sub)
        }
      }
      Ok(List::new(sentence, sublist))
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum ListSentence {
  Sentence(Sentence),
  Column(Column),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sublist1 {
  sentence: Vec<ListSentence>,
  children: Vec<Sublist2>,
}

impl Sublist1 {
  pub fn new(sentence: Vec<ListSentence>, children: Vec<Sublist2>) -> Self {
    Sublist1 { sentence, children }
  }
}

impl Parser for Sublist1 {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Sublist1" {
      let mut children = node.children();
      let list_sentence = children.next();
      let mut sentence = Vec::new();
      if let Some("Sublist1Sentence") = list_sentence.map(|node| node.tag_name().name()) {
        for node in list_sentence.unwrap().children() {
          match node.tag_name().name() {
            "Sentence" => {
              if let Ok(s) = Sentence::parser(&node) {
                sentence.push(ListSentence::Sentence(s))
              }
            }
            "Column" => {
              if let Ok(c) = Column::parser(&node) {
                sentence.push(ListSentence::Column(c))
              }
            }
            _ => return Err(Error::Tag),
          }
        }
      } else {
        return Err(Error::Tag);
      }
      let mut sublist = Vec::new();
      for node in children {
        if let Ok(sub) = Sublist2::parser(&node) {
          sublist.push(sub)
        }
      }
      Ok(Sublist1::new(sentence, sublist))
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sublist2 {
  sentence: Vec<ListSentence>,
  children: Vec<Sublist3>,
}

impl Sublist2 {
  pub fn new(sentence: Vec<ListSentence>, children: Vec<Sublist3>) -> Self {
    Sublist2 { sentence, children }
  }
}

impl Parser for Sublist2 {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Sublist2" {
      let mut children = node.children();
      let list_sentence = children.next();
      let mut sentence = Vec::new();
      if let Some("Sublist2Sentence") = list_sentence.map(|node| node.tag_name().name()) {
        for node in list_sentence.unwrap().children() {
          match node.tag_name().name() {
            "Sentence" => {
              if let Ok(s) = Sentence::parser(&node) {
                sentence.push(ListSentence::Sentence(s))
              }
            }
            "Column" => {
              if let Ok(c) = Column::parser(&node) {
                sentence.push(ListSentence::Column(c))
              }
            }
            _ => return Err(Error::Tag),
          }
        }
      } else {
        return Err(Error::Tag);
      }
      let mut sublist = Vec::new();
      for node in children {
        if let Ok(sub) = Sublist3::parser(&node) {
          sublist.push(sub)
        }
      }
      Ok(Sublist2::new(sentence, sublist))
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sublist3 {
  sentence: Vec<ListSentence>,
}

impl Sublist3 {
  pub fn new(sentence: Vec<ListSentence>) -> Self {
    Sublist3 { sentence }
  }
}

impl Parser for Sublist3 {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Sublist3" {
      let mut children = node.children();
      let list_sentence = children.next();
      let mut sentence = Vec::new();
      if let Some("Sublist3Sentence") = list_sentence.map(|node| node.tag_name().name()) {
        for node in list_sentence.unwrap().children() {
          match node.tag_name().name() {
            "Sentence" => {
              if let Ok(s) = Sentence::parser(&node) {
                sentence.push(ListSentence::Sentence(s))
              }
            }
            "Column" => {
              if let Ok(c) = Column::parser(&node) {
                sentence.push(ListSentence::Column(c))
              }
            }
            _ => return Err(Error::Tag),
          }
        }
      } else {
        return Err(Error::Tag);
      }
      Ok(Sublist3::new(sentence))
    } else {
      Err(Error::Tag)
    }
  }
}
