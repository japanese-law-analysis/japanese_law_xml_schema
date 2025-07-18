//! 文字列一般

use self::to_xml::*;
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

  pub fn to_children(&self) -> Vec<XMLNode> {
    let mut v = Vec::new();
    for c in self.contents.iter() {
      match c {
        TextElement::Text(s) => v.push(XMLNode::Text(s.clone())),
        TextElement::Ruby(ruby) => v.push(XMLNode::Element(ruby.to_xml_element())),
        TextElement::Line(line) => v.push(XMLNode::Element(line.to_xml_element())),
        TextElement::Sub(sub) => v.push(XMLNode::Element(sub.to_xml_element())),
        TextElement::Sup(sup) => v.push(XMLNode::Element(sup.to_xml_element())),
      }
    }
    v
  }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Text {
  fn to_string(&self) -> String {
    let mut s = String::new();
    for v in self.contents.iter() {
      match v {
        TextElement::Sub(sub) => s.push_str(&sub.text),
        TextElement::Sup(sup) => s.push_str(&sup.text),
        TextElement::Ruby(ruby) => s.push_str(&ruby.text.to_string()),
        TextElement::Line(_) => (),
        TextElement::Text(str) => s.push_str(str),
      }
    }
    s
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

impl ToXmlElementWithName for Text {
  fn to_xml_element_with_name(&self, name: &str) -> Element {
    let mut e = Element::new(name);
    for c in self.contents.iter() {
      match c {
        TextElement::Text(s) => e.children.push(XMLNode::Text(s.clone())),
        TextElement::Ruby(ruby) => e.children.push(XMLNode::Element(ruby.to_xml_element())),
        TextElement::Line(line) => e.children.push(XMLNode::Element(line.to_xml_element())),
        TextElement::Sub(sub) => e.children.push(XMLNode::Element(sub.to_xml_element())),
        TextElement::Sup(sup) => e.children.push(XMLNode::Element(sup.to_xml_element())),
      }
    }
    e
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

impl ToXmlElementWithName for TextWithWritingMode {
  fn to_xml_element_with_name(&self, name: &str) -> Element {
    let mut e = Element::new(name);
    for c in self.contents.iter() {
      match c {
        TextElement::Text(s) => e.children.push(XMLNode::Text(s.clone())),
        TextElement::Ruby(ruby) => e.children.push(XMLNode::Element(ruby.to_xml_element())),
        TextElement::Line(line) => e.children.push(XMLNode::Element(line.to_xml_element())),
        TextElement::Sub(sub) => e.children.push(XMLNode::Element(sub.to_xml_element())),
        TextElement::Sup(sup) => e.children.push(XMLNode::Element(sup.to_xml_element())),
      }
    }
    if let WritingMode::Horizontal = self.writing_mode {
      e.attributes
        .insert("WritingMode".to_string(), "horizontal".to_string());
    };
    e
  }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for TextWithWritingMode {
  fn to_string(&self) -> String {
    let mut s = String::new();
    for v in self.contents.iter() {
      match v {
        TextElement::Sub(sub) => s.push_str(&sub.text),
        TextElement::Sup(sup) => s.push_str(&sup.text),
        TextElement::Ruby(ruby) => s.push_str(&ruby.text.to_string()),
        TextElement::Line(_) => (),
        TextElement::Text(str) => s.push_str(str),
      }
    }
    s
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

impl ToXmlElement for Ruby {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Ruby");
    let mut rt = Element::new("Rt");
    rt.children.push(XMLNode::Text(self.ruby.clone()));
    for n in self.text.to_children().iter() {
      e.children.push(n.clone());
    }
    e.children.push(XMLNode::Element(rt));
    e
  }
}

/// 上付き文字
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sup {
  pub text: String,
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

impl ToXmlElement for Sup {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Sup");
    e.children.push(XMLNode::Text(self.text.clone()));
    e
  }
}

/// 下付き文字
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Sub {
  pub text: String,
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

impl ToXmlElement for Sub {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Sub");
    e.children.push(XMLNode::Text(self.text.clone()));
    e
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_text() {
    let data = r#"<Sentence Num="1" WritingMode="vertical">激<Ruby>甚<Rt>じん</Rt></Ruby>災害に対処するための特別の財政援助等に関する法律（以下「法」という。）第三条第一項の政令で定める基準に該当する都道府県又は市町村は、その年に発生した激甚災害（法第二条第一項の規定により激甚災害として指定され、かつ、同条第二項の規定により当該事項に係る法の規定の適用が指定された災害をいう。以下同じ。）に係る法第三条第一項各号に掲げる事業ごとの当該都道府県又は市町村の負担額を合算した額の当該激甚災害が発生した年の四月一日の属する会計年度における当該都道府県又は市町村の標準税収入（法第四条第一項第一号の標準税収入をいう。以下同じ。）に対する割合が都道府県にあつては百分の十、市町村にあつては百分の五を超えるものとする。</Sentence>"#;
    let node = xmltree::Element::parse(data.as_bytes()).unwrap();
    let text = Text::from_children(&node.children);
    assert_eq!(text.to_string(), "激甚災害に対処するための特別の財政援助等に関する法律（以下「法」という。）第三条第一項の政令で定める基準に該当する都道府県又は市町村は、その年に発生した激甚災害（法第二条第一項の規定により激甚災害として指定され、かつ、同条第二項の規定により当該事項に係る法の規定の適用が指定された災害をいう。以下同じ。）に係る法第三条第一項各号に掲げる事業ごとの当該都道府県又は市町村の負担額を合算した額の当該激甚災害が発生した年の四月一日の属する会計年度における当該都道府県又は市町村の標準税収入（法第四条第一項第一号の標準税収入をいう。以下同じ。）に対する割合が都道府県にあつては百分の十、市町村にあつては百分の五を超えるものとする。");
  }

  #[test]
  fn check_ruby() {
    let data = r#"<Ruby>甚<Rt>じん</Rt></Ruby>"#;
    let node = xmltree::Element::parse(data.as_bytes()).unwrap();
    let ruby = Ruby::parser(&node).unwrap();
    assert_eq!(ruby, Ruby::new(&Text::from_value("甚"), "じん"));
    assert_eq!(ruby.text.to_string(), "甚".to_string())
  }
}
