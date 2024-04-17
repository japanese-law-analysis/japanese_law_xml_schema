//! [法令標準XMLスキーマ](https://elaws.e-gov.go.jp/file/XMLSchemaForJapaneseLaw_v3.xsd)で定義された規格に基づいてXMLとの相互変換を行う
//!
//!

#![recursion_limit = "256"]

pub mod appdx;
pub mod article;
pub mod article_number;
pub mod class;
pub mod contents;
pub mod fig;
pub mod law;
pub mod line;
pub mod list;
pub mod paragraph;
mod parse_from_text;
pub mod parser;
pub mod remarks;
pub mod result;
pub mod sentence;
pub mod structs;
pub mod suppl_provision;
pub mod table;
pub mod table_of_contents;
mod tests;
pub mod text;
pub mod to_xml;

use koyomi::{era, Date};
use result::*;
use std::io::{Read, Write};
use to_xml::ToXmlElement;
use xmltree::Element;

use crate::parser::Parser;

/// HTMLを生成できる構造であることを保証するトレイト
pub trait ToHtml {
  /// HTML文字列を生成する関数
  fn to_html(&self) -> String;
}

/// 意味のあるテキストに変換できることを保証するトレイト
pub trait ToText {
  /// 変換する関数
  fn to_text(&self) -> String;
}

/// XML文字列を法律の構造体に変換する
pub fn parse_xml(xml: &[u8]) -> Result<law::Law> {
  let element = Element::parse(xml).map_err(|_| Error::XMLParsing)?;
  let law = law::Law::parser(&element)?;
  Ok(law)
}

pub fn parse_file(path: &str) -> Result<law::Law> {
  let mut file = std::fs::File::open(path).map_err(|_| result::Error::Io)?;
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).map_err(|_| result::Error::Io)?;
  let law = parse_xml(&buf)?;
  Ok(law)
}

/// テキスト情報を法律の構造体に変換する
#[allow(clippy::too_many_arguments)]
pub fn parse_text(
  text: &str,
  year: usize,
  month: Option<usize>,
  day: Option<usize>,
  law_type: law::LawType,
  lang: law::Lang,
  num: usize,
  law_num: String,
  title: &str,
) -> Result<law::Law> {
  let date = Date::from_ymd(
    year as i32,
    month.unwrap_or(1) as u32,
    day.unwrap_or(1) as u32,
  )
  .unwrap();
  let e = era(&date).unwrap();
  let era = match e.name().as_str() {
    "明治" => law::Era::Meiji,
    "大正" => law::Era::Taisho,
    "昭和" => law::Era::Showa,
    "平成" => law::Era::Heisei,
    "令和" => law::Era::Reiwa,
    _ => unreachable!(),
  };
  let body = parse_from_text::parse_body(title, text)?;
  Ok(law::Law {
    era,
    year,
    num,
    promulgate_month: month,
    promulgate_day: day,
    law_type,
    lang,
    law_num,
    law_body: body,
  })
}

struct WritableString(String);

impl WritableString {
  pub fn new() -> Self {
    Self(String::new())
  }

  pub fn string(self) -> String {
    self.0
  }
}

impl Write for WritableString {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let s = std::str::from_utf8(buf)
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    self.0.push_str(s);
    Ok(s.len())
  }

  fn flush(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

fn gen_config() -> xmltree::EmitterConfig {
  xmltree::EmitterConfig::new().perform_indent(true)
}

/// XML文字列に変換する
pub fn to_xml(law: &law::Law) -> result::Result<String> {
  let mut s = WritableString::new();
  let config = gen_config();
  law
    .to_xml_element()
    .write_with_config(&mut s, config)
    .map_err(|_| result::Error::Io)?;
  Ok(s.string())
}

/// XML文字列を書き出す
pub fn write_file<W: Write>(law: &law::Law, w: &mut W) -> result::Result<()> {
  let config = gen_config();
  law
    .to_xml_element()
    .write_with_config(w, config)
    .map_err(|_| result::Error::Io)
}

pub(crate) fn get_attribute(element: &Element, name: &str) -> Result<String> {
  let att_opt = element.attributes.get(name);
  match att_opt {
    Some(s) => Ok(s.to_string()),
    None => Err(Error::MissingRequiredAttribute {
      tag_name: element.name.to_string(),
      attribute_name: name.to_string(),
    }),
  }
}

pub(crate) fn get_attribute_with_parse<T>(element: &Element, name: &str) -> Result<T>
where
  T: std::str::FromStr,
{
  let att_opt = element.attributes.get(name);
  match att_opt {
    Some(s) => {
      if let Ok(t) = s.parse::<T>() {
        Ok(t)
      } else {
        Err(Error::AttributeParseError {
          tag_name: element.name.to_string(),
          attribute_name: name.to_string(),
        })
      }
    }
    None => Err(Error::MissingRequiredAttribute {
      tag_name: element.name.to_string(),
      attribute_name: name.to_string(),
    }),
  }
}

pub(crate) fn get_attribute_opt_with_parse<T>(element: &Element, name: &str) -> Result<Option<T>>
where
  T: std::str::FromStr,
{
  let att_opt = element.attributes.get(name);
  match att_opt {
    Some(s) => {
      if let Ok(t) = s.parse::<T>() {
        Ok(Some(t))
      } else {
        Err(Error::AttributeParseError {
          tag_name: element.name.to_string(),
          attribute_name: name.to_string(),
        })
      }
    }
    None => Ok(None),
  }
}
