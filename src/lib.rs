//! [法令標準XMLスキーマ](https://elaws.e-gov.go.jp/file/XMLSchemaForJapaneseLaw_v3.xsd)で定義された規格に基づいてXMLとの相互変換を行う
//!
//!
#![recursion_limit = "256"]
use serde::{Deserialize, Serialize};

pub mod appdx;
pub mod article;
pub mod contents;
pub mod fig;
pub mod line;
pub mod list;
pub mod paragraph;
pub mod parser;
pub mod remarks;
pub mod result;
pub mod sentence;
pub mod structs;
pub mod suppl_provision;
pub mod table;
pub mod table_of_contents;
pub mod text;

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

/// 法令そのもの
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Law {
  /// 年号
  era: Era,
  /// 制定年
  year: usize,
  /// その年で制定された法令の通し番号
  num: usize,
  /// 公布月
  promulgate_month: Option<usize>,
  /// 公布日
  promulgate_day: Option<usize>,
  /// 法令の種類
  law_type: LawType,
  /// 言語
  lang: Lang,
  /// 法令番号
  law_num: String,
  /// 法令の中身
  law_body: LawBody,
}

/// 年号
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Era {
  /// 明治
  Meiji,
  /// 大正
  Taisho,
  /// 昭和
  Showa,
  /// 平成
  Heisei,
  /// 令和
  Reiwa,
}

/// 法令の種類
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum LawType {
  /// 憲法
  Constitution,
  /// 法律
  Act,
  /// 政令
  CabinetOrder,
  /// 勅令
  ImperialOrder,
  /// 府省令
  MinisterialOrdinance,
  /// 規則
  Rule,
  /// その他
  Misc,
}

/// 言語
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Lang {
  Ja,
  En,
}

/// 法令の中身
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct LawBody {
  /// 法令名
  law_title: Option<LawTitle>,
  /// 制定にかかる声明
  enact_statement: Vec<text::Text>,
  /// 主題
  subject: Option<String>,
  /// 目次
  toc: Option<table_of_contents::TOC>,
  /// 前文
  preamble: Option<Preamble>,
  /// 本文
  main_provision: MainProvision,
  /// 附則
  suppl_provision: Vec<suppl_provision::SupplProvision>,
  /// 付録表
  appdx_table: Vec<appdx::AppdxTable>,
  /// 付録記載
  appdx_note: Vec<appdx::AppdxNote>,
  /// 付録様式
  appdx_style: Vec<appdx::AppdxStyle>,
  /// 付録
  appdx: Vec<appdx::Appdx>,
  /// 付録図
  appdx_fig: Vec<appdx::AppdxFig>,
  /// 付録書式
  appdx_format: Vec<appdx::AppdxFormat>,
}

/// 法令名
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct LawTitle {
  /// ひらがなでの読み
  kana: Option<String>,
  /// 略称
  abbrev: Option<String>,
  /// 略称のひらがな読み
  abbrev_kana: Option<String>,
  /// 法令名
  contents: text::Text,
}

/// 前文
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Preamble {
  children: Vec<paragraph::Paragraph>,
}

/// 本文
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct MainProvision {
  /// 本文の要素
  children: Vec<MainProvisionContents>,
  extract: Option<bool>,
}

/// 本文の要素
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum MainProvisionContents {
  /// 編
  Part(article::Part),
  /// 章
  Chapter(article::Chapter),
  /// 節
  Section(article::Section),
  /// 条
  Article(article::Article),
  /// 段落
  Paragraph(paragraph::Paragraph),
}

/// 改正
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AmendProvision {
  sentence: sentence::Sentence,
  new_provision: Vec<NewProvision>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum NewProvision {
  LawTitle(LawTitle),
  Preamble(Preamble),
  TOC(table_of_contents::TOC),
  Part(article::Part),
  PartTitle(text::Text),
  Chapter(article::Chapter),
  ChapterTitle(text::Text),
  Section(article::Section),
  SectionTitle(text::Text),
  Subsection(article::Subsection),
  SubsectionTitle(text::Text),
  Division(article::Division),
  DivisionTitle(text::Text),
  Article(article::Article),
  SupplNote(text::Text),
  Paragraph(paragraph::Paragraph),
  Item(paragraph::Item),
  Subitem1(paragraph::Subitem1),
  Subitem2(paragraph::Subitem2),
  Subitem3(paragraph::Subitem3),
  Subitem4(paragraph::Subitem4),
  Subitem5(paragraph::Subitem5),
  Subitem6(paragraph::Subitem6),
  Subitem7(paragraph::Subitem7),
  Subitem8(paragraph::Subitem8),
  Subitem9(paragraph::Subitem9),
  Subitem10(paragraph::Subitem10),
  List(list::List),
  Sentence(sentence::Sentence),
  AmendProvision(AmendProvision),
  AppdxTable(appdx::AppdxTable),
  AppdxNote(appdx::AppdxNote),
  AppdxStyle(appdx::AppdxStyle),
  Appdx(appdx::Appdx),
  AppdxFig(appdx::AppdxFig),
  AppdxFormat(appdx::AppdxFormat),
  SupplProvisionAppdxStyle(suppl_provision::SupplProvisionAppdxStyle),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Class {
  class_title: Option<text::Text>,
  class_sentence: SentenceOrColumnOrTable,
  children: Vec<paragraph::Item>,
  num: String,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SentenceOrColumnOrTable {
  Sentence(Vec<sentence::Sentence>),
  Column(Vec<Column>),
  Table(table::Table),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Caption {
  text: text::Text,
  common_caption: Option<bool>,
}

impl Caption {
  pub fn new(text: text::Text, common_caption: Option<bool>) -> Self {
    Caption {
      text,
      common_caption,
    }
  }
}

impl parser::Parser for Caption {
  fn parser(node: &roxmltree::Node) -> result::Result<Self> {
    let text = text::Text::from_children(node.children());
    let common_caption = node
      .attribute("CommonCaption")
      .and_then(|s| s.parse::<bool>().ok());
    Ok(Caption::new(text, common_caption))
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Column {
  sentence: Vec<sentence::Sentence>,
  num: Option<usize>,
  line_break: bool,
  align: Option<Align>,
}

impl parser::Parser for Column {
  fn parser(node: &roxmltree::Node) -> result::Result<Self> {
    if node.tag_name().name() == "Column" {
      let num = node.attribute("Num").and_then(|s| s.parse::<usize>().ok());
      let line_break = node
        .attribute("LineBreak")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);
      let align = Align::from_attribute(node.attribute("Align"));
      let mut sentence = Vec::new();
      for node in node.children() {
        if let Ok(v) = sentence::Sentence::parser(&node) {
          sentence.push(v);
        }
      }
      Ok(Column {
        sentence,
        num,
        line_break,
        align,
      })
    } else {
      Err(result::Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Align {
  Left,
  Center,
  Right,
  Justify,
}

impl Align {
  pub fn from_attribute(att: Option<&str>) -> Option<Align> {
    match att {
      Some("left") => Some(Align::Left),
      Some("center") => Some(Align::Center),
      Some("right") => Some(Align::Right),
      Some("justify") => Some(Align::Justify),
      _ => None,
    }
  }
}
