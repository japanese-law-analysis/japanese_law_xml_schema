use crate::line::*;
use crate::parser::*;
use crate::result::Error;
use crate::*;
use roxmltree::{Children, Node};
use serde::{Deserialize, Serialize};

/// テキスト
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Text {
  contents: Vec<TextElement>,
}

#[allow(clippy::new_without_default)]
impl Text {
  /// テキストの初期値を生成する
  pub fn new() -> Self {
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
  contents: Vec<TextElement>,
  writing_mode: WritingMode,
}

/// テキストの要素
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
enum TextElement {
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
  text: Text,
  /// ルビ
  ruby: String,
}

impl Ruby {
  pub fn new(text: &Text, ruby: &str) -> Self {
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
      if let Some(r) = ruby {
        Ok(Ruby::new(&text, r))
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

/// 上付き文字
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sup {
  text: String,
}

impl Sup {
  pub fn new(text: &str) -> Self {
    Sup {
      text: text.to_string(),
    }
  }
}

impl ToHtml for Sup {
  fn to_html(&self) -> String {
    format!("<sup>{}</sup>", self.text)
  }
}

impl Parser for Sup {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Sup" {
      if let Some(text) = node.children().next().and_then(|n| n.text()) {
        Ok(Sup {
          text: text.to_string(),
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

/// 下付き文字
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sub {
  text: String,
}

impl Sub {
  pub fn new(text: &str) -> Self {
    Sub {
      text: text.to_string(),
    }
  }
}

impl ToHtml for Sub {
  fn to_html(&self) -> String {
    format!("<sub>{}</sub>", self.text)
  }
}

impl Parser for Sub {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Sub" {
      if let Some(text) = node.children().next().and_then(|n| n.text()) {
        Ok(Sub {
          text: text.to_string(),
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}
