//! 文字列一般

use crate::line::*;
use crate::parser::*;
use crate::result::Error;
use crate::*;
use roxmltree::{Children, Node};
use serde::{Deserialize, Serialize};

/// テキスト
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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
  pub fn from_children(children: Children) -> Self {
    let mut text = Text::new();
    for node in children {
      match node.tag_name().name() {
        "Ruby" => {
          if let Ok(ruby) = Ruby::parser(&node) {
            text.add_ruby(ruby)
          }
        }
        "Line" => {
          if let Ok(line) = Line::parser(&node) {
            text.add_line(line)
          }
        }
        "Sup" => {
          if let Ok(sup) = Sup::parser(&node) {
            text.add_sup(sup)
          }
        }
        "Sub" => {
          if let Ok(sub) = Sub::parser(&node) {
            text.add_sub(sub)
          }
        }
        "" => {
          if node.is_text() {
            if let Some(s) = node.text() {
              text.add_string(s)
            }
          }
        }
        _ => (),
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
  fn parser(node: &Node) -> result::Result<Self> {
    let mut text = Text::new();
    match node.tag_name().name() {
      "Ruby" => {
        if let Ok(ruby) = Ruby::parser(node) {
          text.add_ruby(ruby)
        }
      }
      "Line" => {
        if let Ok(line) = Line::parser(node) {
          text.add_line(line)
        }
      }
      "Sup" => {
        if let Ok(sup) = Sup::parser(node) {
          text.add_sup(sup)
        }
      }
      "Sub" => {
        if let Ok(sub) = Sub::parser(node) {
          text.add_sub(sub)
        }
      }
      "" => {
        if node.is_text() {
          if let Some(s) = node.text() {
            text.add_string(s)
          }
        }
      }
      _ => (),
    }
    Ok(text)
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum WritingMode {
  Vertical,
  Horizontal,
}

/// 段落方向の情報がついたテキスト
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TextWithWritingMode {
  pub contents: Vec<TextElement>,
  pub writing_mode: WritingMode,
}

impl Parser for TextWithWritingMode {
  fn parser(node: &Node) -> result::Result<Self> {
    let writing_mode = node.attribute("WritingMode");
    let text = Text::from_children(node.children());
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
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum TextElement {
  Ruby(Ruby),
  Line(Line),
  Sup(Sup),
  Sub(Sub),
  Text(String),
}

/// ルビ
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Ruby" {
      let children = node.children();
      let mut text = Text::new();
      let mut ruby = None;
      for children_node in children {
        if "Rt" == children_node.tag_name().name() {
          if let Some(t) = children_node.children().next().and_then(|n| n.text()) {
            ruby = Some(t)
          }
          break;
        } else if let Ok(t) = Text::parser(&children_node) {
          text.add_text(t);
        }
      }
      let r = ruby.unwrap_or_default();
      Ok(Ruby::new(&text, r))
    } else {
      Err(Error::wrong_tag_name(node, "Ruby"))
    }
  }
}

/// 上付き文字
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sup {
  pub text: String,
}

impl ToHtml for Sup {
  fn to_html(&self) -> String {
    format!("<sup>{}</sup>", self.text)
  }
}

impl Parser for Sup {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Sup" {
      let text = node
        .children()
        .next()
        .and_then(|n| n.text())
        .unwrap_or_default();
      Ok(Sup {
        text: text.to_string(),
      })
    } else {
      Err(Error::wrong_tag_name(node, "Sup"))
    }
  }
}

/// 下付き文字
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sub {
  pub text: String,
}

impl ToHtml for Sub {
  fn to_html(&self) -> String {
    format!("<sub>{}</sub>", self.text)
  }
}

impl Parser for Sub {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Sub" {
      let text = node
        .children()
        .next()
        .and_then(|n| n.text())
        .unwrap_or_default();
      Ok(Sub {
        text: text.to_string(),
      })
    } else {
      Err(Error::wrong_tag_name(node, "Sub"))
    }
  }
}
