//! 文字列一般

use crate::line::*;
use crate::parser::*;
use crate::result::Error;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

/// テキスト
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Text {
  pub contents: Vec<TextElement>,
}

#[allow(clippy::new_without_default)]
impl Text {
  /// テキストの初期値を生成する
  pub(crate) fn new() -> Self {
    Text {
      contents: Vec::new(),
    }
  }
  /// Node列からの生成
  pub fn from_children(children: &[XMLNode]) -> Self {
    let mut text = Text::new();
    for node in children {
      if let XMLNode::Element(element) = node {
        match element.name.as_str() {
          "Ruby" => {
            if let Ok(ruby) = Ruby::parser(element) {
              text.add_ruby(ruby)
            }
          }
          "Line" => {
            if let Ok(line) = Line::parser(element) {
              text.add_line(line)
            }
          }
          "Sup" => {
            if let Ok(sup) = Sup::parser(element) {
              text.add_sup(sup)
            }
          }
          "Sub" => {
            if let Ok(sub) = Sub::parser(element) {
              text.add_sub(sub)
            }
          }
          _ => (),
        }
      } else if let XMLNode::Text(s) = node {
        text.add_string(s)
      }
    }
    text
  }

  /// 値から
  pub fn from_value<T: ToString>(t: T) -> Self {
    Text {
      contents: vec![TextElement::Text(t.to_string())],
    }
  }

  /// テキストを追加する
  pub fn add_text(&mut self, text2: Text) {
    for t in text2.contents {
      self.contents.push(t)
    }
  }
  /// テキストに文字列を追加する
  pub fn add_string(&mut self, str: &str) {
    self.contents.push(TextElement::Text(str.to_string()))
  }
  /// テキストにルビを追加する
  pub fn add_ruby(&mut self, ruby: Ruby) {
    self.contents.push(TextElement::Ruby(ruby))
  }
  /// テキストに線を追加する
  pub fn add_line(&mut self, line: Line) {
    self.contents.push(TextElement::Line(line))
  }
  /// テキストに上付き文字を追加する
  pub fn add_sup(&mut self, sup: Sup) {
    self.contents.push(TextElement::Sup(sup))
  }
  /// テキストに下付き文字を追加する
  pub fn add_sub(&mut self, sub: Sub) {
    self.contents.push(TextElement::Sub(sub))
  }
  pub fn with_writing_mode(&self, writing_mode: WritingMode) -> TextWithWritingMode {
    TextWithWritingMode {
      contents: self.clone().contents,
      writing_mode,
    }
  }
}

impl ToHtml for Text {
  fn to_html(&self) -> String {
    use TextElement::*;
    format!(
      "<p>{}</p>",
      self
        .contents
        .iter()
        .map(|text_element| match text_element {
          Ruby(ruby) => ruby.to_html(),
          Line(line) => line.to_html(),
          Sup(sup) => sup.to_html(),
          Sub(sub) => sub.to_html(),
          Text(text) => text.to_string(),
        })
        .collect::<String>()
    )
  }
}

impl Parser for Text {
  fn parser(element: &Element) -> result::Result<Self> {
    let mut text = Text::new();
    match element.name.as_str() {
      "Ruby" => {
        if let Ok(ruby) = Ruby::parser(element) {
          text.add_ruby(ruby)
        }
      }
      "Line" => {
        if let Ok(line) = Line::parser(element) {
          text.add_line(line)
        }
      }
      "Sup" => {
        if let Ok(sup) = Sup::parser(element) {
          text.add_sup(sup)
        }
      }
      "Sub" => {
        if let Ok(sub) = Sub::parser(element) {
          text.add_sub(sub)
        }
      }
      _ => (),
    }
    Ok(text)
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum WritingMode {
  Vertical,
  Horizontal,
}

/// 段落方向の情報がついたテキスト
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextWithWritingMode {
  pub contents: Vec<TextElement>,
  pub writing_mode: WritingMode,
}

impl Parser for TextWithWritingMode {
  fn parser(element: &Element) -> result::Result<Self> {
    let writing_mode = element.attributes.get("WritingMode").map(|s| s.as_str());
    let text = Text::from_children(&element.children);
    match writing_mode {
      Some("vertical") => Ok(TextWithWritingMode {
        contents: text.contents,
        writing_mode: WritingMode::Vertical,
      }),
      Some("horizontal") => Ok(TextWithWritingMode {
        contents: text.contents,
        writing_mode: WritingMode::Horizontal,
      }),
      _ => Ok(TextWithWritingMode {
        contents: text.contents,
        writing_mode: WritingMode::Vertical,
      }),
    }
  }
}

/// テキストの要素
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextElement {
  Ruby(Ruby),
  Line(Line),
  Sup(Sup),
  Sub(Sub),
  Text(String),
}

/// ルビ
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ruby {
  /// 本文
  pub text: Text,
  /// ルビ
  pub ruby: String,
}

impl Ruby {
  fn new(text: &Text, ruby: &str) -> Self {
    Ruby {
      text: text.clone(),
      ruby: ruby.to_string(),
    }
  }
}

impl ToHtml for Ruby {
  fn to_html(&self) -> String {
    format!("<ruby>{}<rt>{}</rt></ruby>", self.text.to_html(), self.ruby)
  }
}

impl Parser for Ruby {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name == "Ruby" {
      let children = &element.children;
      let mut text = Text::new();
      let mut ruby = None;
      for children_node in children.iter() {
        if let XMLNode::Element(element) = children_node {
          if "Rt" == element.name.as_str() {
            let s = element
              .children
              .iter()
              .map(|n| {
                if let XMLNode::Text(s) = n {
                  s.clone()
                } else {
                  String::new()
                }
              })
              .collect::<String>();
            ruby = Some(s)
          }
        } else if let XMLNode::Text(s) = children_node {
          text.add_string(s)
        }
      }
      let r = ruby.unwrap_or_default();
      Ok(Ruby::new(&text, r.as_str()))
    } else {
      Err(Error::wrong_tag_name(element, "Ruby"))
    }
  }
}

/// 上付き文字
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sup {
  pub text: String,
}

impl ToHtml for Sup {
  fn to_html(&self) -> String {
    format!("<sup>{}</sup>", self.text)
  }
}

impl Parser for Sup {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Sup" {
      let text = element
        .children
        .first()
        .and_then(|n| {
          if let XMLNode::Text(s) = n {
            Some(s.clone())
          } else {
            None
          }
        })
        .unwrap_or_default();
      Ok(Sup { text })
    } else {
      Err(Error::wrong_tag_name(element, "Sup"))
    }
  }
}

/// 下付き文字
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sub {
  pub text: String,
}

impl ToHtml for Sub {
  fn to_html(&self) -> String {
    format!("<sub>{}</sub>", self.text)
  }
}

impl Parser for Sub {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Sub" {
      let text = element
        .children
        .first()
        .and_then(|n| {
          if let XMLNode::Text(s) = n {
            Some(s.clone())
          } else {
            None
          }
        })
        .unwrap_or_default();
      Ok(Sub { text })
    } else {
      Err(Error::wrong_tag_name(element, "Sub"))
    }
  }
}
