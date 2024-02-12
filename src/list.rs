//! 列挙に関する

use crate::class::*;
use crate::parser::*;
use crate::result::Error;
use crate::sentence::*;
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
  fn new(sentence: Vec<ListSentence>, children: Vec<Sublist1>) -> Self {
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
              let v = Sentence::parser(&node)?;
              sentence.push(ListSentence::Sentence(v))
            }
            "Column" => {
              let v = Column::parser(&node)?;
              sentence.push(ListSentence::Column(v))
            }
            s => return Err(Error::unexpected_tag(&node, s)),
          }
        }
      } else {
        return Err(Error::wrong_tag_name(node, "LiseSentence"));
      }
      let mut sublist = Vec::new();
      for node in children {
        let v = Sublist1::parser(&node)?;
        sublist.push(v)
      }
      Ok(List::new(sentence, sublist))
    } else {
      Err(Error::wrong_tag_name(node, "List"))
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
  fn new(sentence: Vec<ListSentence>, children: Vec<Sublist2>) -> Self {
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
              let v = Sentence::parser(&node)?;
              sentence.push(ListSentence::Sentence(v))
            }
            "Column" => {
              let v = Column::parser(&node)?;
              sentence.push(ListSentence::Column(v))
            }
            s => return Err(Error::unexpected_tag(&node, s)),
          }
        }
      } else {
        return Err(Error::wrong_tag_name(node, "Sublist1Sentence"));
      }
      let mut sublist = Vec::new();
      for node in children {
        let v = Sublist2::parser(&node)?;
        sublist.push(v)
      }
      Ok(Sublist1::new(sentence, sublist))
    } else {
      Err(Error::wrong_tag_name(node, "Sublist1"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sublist2 {
  sentence: Vec<ListSentence>,
  children: Vec<Sublist3>,
}

impl Sublist2 {
  fn new(sentence: Vec<ListSentence>, children: Vec<Sublist3>) -> Self {
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
              let v = Sentence::parser(&node)?;
              sentence.push(ListSentence::Sentence(v))
            }
            "Column" => {
              let v = Column::parser(&node)?;
              sentence.push(ListSentence::Column(v))
            }
            s => return Err(Error::unexpected_tag(&node, s)),
          }
        }
      } else {
        return Err(Error::wrong_tag_name(node, "Sublist2Sentence"));
      }
      let mut sublist = Vec::new();
      for node in children {
        let v = Sublist3::parser(&node)?;
        sublist.push(v)
      }
      Ok(Sublist2::new(sentence, sublist))
    } else {
      Err(Error::wrong_tag_name(node, "Sublist2"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sublist3 {
  sentence: Vec<ListSentence>,
}

impl Sublist3 {
  fn new(sentence: Vec<ListSentence>) -> Self {
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
              let v = Sentence::parser(&node)?;
              sentence.push(ListSentence::Sentence(v))
            }
            "Column" => {
              let v = Column::parser(&node)?;
              sentence.push(ListSentence::Column(v))
            }
            s => return Err(Error::unexpected_tag(&node, s)),
          }
        }
      } else {
        return Err(Error::wrong_tag_name(node, "Sublist3Sentence"));
      }
      Ok(Sublist3::new(sentence))
    } else {
      Err(Error::wrong_tag_name(node, "Sublist3"))
    }
  }
}
