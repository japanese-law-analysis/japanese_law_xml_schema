//! 法律本体

use crate::appdx::*;
use crate::article::*;
use crate::list::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::remarks::*;
use crate::result::Error;
use crate::sentence::*;
use crate::structs::*;
use crate::suppl_provision::*;
use crate::table::*;
use crate::table_of_contents::*;
use crate::text::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

/// 法令そのもの
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Law {
  /// 年号
  pub era: Era,
  /// 制定年
  pub year: usize,
  /// その年で制定された法令の通し番号
  pub num: usize,
  /// 公布月
  pub promulgate_month: Option<usize>,
  /// 公布日
  pub promulgate_day: Option<usize>,
  /// 法令の種類
  pub law_type: LawType,
  /// 言語
  pub lang: Lang,
  /// 法令番号
  pub law_num: String,
  /// 法令の中身
  pub law_body: LawBody,
}

impl Parser for Law {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Law" {
      let era = match element.attributes.get("Era").map(|s| s.as_str()) {
        Some("Meiji") => Era::Meiji,
        Some("Taisho") => Era::Taisho,
        Some("Showa") => Era::Showa,
        Some("Heisei") => Era::Heisei,
        Some("Reiwa") => Era::Reiwa,
        _ => {
          return Err(Error::AttributeParseError {
            tag_name: element.name.clone(),
            attribute_name: "Era".to_string(),
          })
        }
      };
      let year = get_attribute_with_parse(element, "Year")?;
      let num = get_attribute_with_parse(element, "Num")?;
      let promulgate_month = get_attribute_opt_with_parse(element, "PromulgateMonth")?;
      let promulgate_day = get_attribute_opt_with_parse(element, "PromulgateDay")?;
      let law_type = match element.attributes.get("LawType").map(|s| s.as_str()) {
        Some("Constitution") => LawType::Constitution,
        Some("Act") => LawType::Act,
        Some("CabinetOrder") => LawType::CabinetOrder,
        Some("ImperialOrder") => LawType::ImperialOrder,
        Some("MinisterialOrdinance") => LawType::MinisterialOrdinance,
        Some("Rule") => LawType::Rule,
        Some("Misc") => LawType::Misc,
        _ => {
          return Err(Error::AttributeParseError {
            tag_name: element.name.clone(),
            attribute_name: "LawType".to_string(),
          })
        }
      };
      let lang = match element.attributes.get("Lang").map(|s| s.as_str()) {
        Some("ja") => Lang::Ja,
        Some("en") => Lang::En,
        _ => {
          return Err(Error::AttributeParseError {
            tag_name: element.name.clone(),
            attribute_name: "Lang".to_string(),
          })
        }
      };
      let mut children = element.children.iter();
      let law_num_element = children
        .next()
        .and_then(|n| {
          if let XMLNode::Element(e) = n {
            Some(e)
          } else {
            None
          }
        })
        .ok_or(Error::missing_required_tag("LawNum"))?;
      let law_num_element_tag_name = law_num_element.name.clone();
      let law_num = if law_num_element_tag_name == "LawNum" {
        law_num_element
          .children
          .iter()
          .map(|n| {
            if let XMLNode::Text(s) = n {
              s.clone()
            } else {
              String::new()
            }
          })
          .collect::<String>()
      } else {
        return Err(Error::UnexpectedTag {
          wrong_name: law_num_element_tag_name,
          tag: "LawNum".to_string(),
        });
      };
      let law_body_element = children
        .next()
        .and_then(|n| {
          if let XMLNode::Element(e) = n {
            Some(e)
          } else {
            None
          }
        })
        .ok_or(Error::missing_required_tag("LawBody"))?;
      let law_body = LawBody::parser(law_body_element)?;
      Ok(Law {
        era,
        year,
        num,
        promulgate_month,
        promulgate_day,
        law_type,
        lang,
        law_num,
        law_body,
      })
    } else {
      Err(Error::wrong_tag_name(element, "Law"))
    }
  }
}

/// 年号
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum Lang {
  Ja,
  En,
}

/// 法令の中身
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct LawBody {
  /// 法令名
  pub law_title: Option<LawTitle>,
  /// 制定にかかる声明
  pub enact_statement: Vec<Text>,
  /// 主題
  pub subject: Option<String>,
  /// 目次
  pub toc: Option<TOC>,
  /// 前文
  pub preamble: Option<Preamble>,
  /// 本文
  pub main_provision: MainProvision,
  /// 附則
  pub suppl_provision: Vec<SupplProvision>,
  /// 付録表
  pub appdx_table: Vec<AppdxTable>,
  /// 付録記載
  pub appdx_note: Vec<AppdxNote>,
  /// 付録様式
  pub appdx_style: Vec<AppdxStyle>,
  /// 付録
  pub appdx: Vec<Appdx>,
  /// 付録図
  pub appdx_fig: Vec<AppdxFig>,
  /// 付録書式
  pub appdx_format: Vec<AppdxFormat>,
}

impl Parser for LawBody {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "LawBody" {
      let subject = get_attribute_opt_with_parse(element, "Subject")?;
      let mut law_title = None;
      let mut enact_statement = Vec::new();
      let mut toc = None;
      let mut preamble = None;
      let mut main_provision = None;
      let mut suppl_provision = Vec::new();
      let mut appdx_table = Vec::new();
      let mut appdx_note = Vec::new();
      let mut appdx_style = Vec::new();
      let mut appdx = Vec::new();
      let mut appdx_fig = Vec::new();
      let mut appdx_format = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "LawTitle" => {
              let v = LawTitle::parser(e)?;
              law_title = Some(v);
            }
            "EnactStatement" => enact_statement.push(Text::from_children(&e.children)),
            "TOC" => {
              let v = TOC::parser(e)?;
              toc = Some(v);
            }
            "Preamble" => {
              let v = Preamble::parser(e)?;
              preamble = Some(v);
            }
            "MainProvision" => {
              let v = MainProvision::parser(e)?;
              main_provision = Some(v);
            }
            "SupplProvision" => {
              let v = SupplProvision::parser(e)?;
              suppl_provision.push(v);
            }
            "AppdxTable" => {
              let v = AppdxTable::parser(e)?;
              appdx_table.push(v);
            }
            "AppdxNote" => {
              let v = AppdxNote::parser(e)?;
              appdx_note.push(v);
            }
            "AppdxStyle" => {
              let v = AppdxStyle::parser(e)?;
              appdx_style.push(v);
            }
            "Appdx" => {
              let v = Appdx::parser(e)?;
              appdx.push(v);
            }
            "AppdxFig" => {
              let v = AppdxFig::parser(e)?;
              appdx_fig.push(v);
            }
            "AppdxFormat" => {
              let v = AppdxFormat::parser(e)?;
              appdx_format.push(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      if let Some(main_provision) = main_provision {
        Ok(LawBody {
          law_title,
          enact_statement,
          subject,
          toc,
          preamble,
          main_provision,
          suppl_provision,
          appdx_table,
          appdx_note,
          appdx_style,
          appdx,
          appdx_fig,
          appdx_format,
        })
      } else {
        Err(Error::missing_required_tag("MainProvision"))
      }
    } else {
      Err(Error::wrong_tag_name(element, "LawBody"))
    }
  }
}

/// 法令名
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct LawTitle {
  /// ひらがなでの読み
  pub kana: Option<String>,
  /// 略称
  pub abbrev: Option<String>,
  /// 略称のひらがな読み
  pub abbrev_kana: Option<String>,
  /// 法令名
  pub text: Text,
}

impl Parser for LawTitle {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "LawTitle" {
      let kana = get_attribute_opt_with_parse(element, "Kana")?;
      let abbrev = get_attribute_opt_with_parse(element, "Abbrev")?;
      let abbrev_kana = get_attribute_opt_with_parse(element, "AbbrevKana")?;
      let text = Text::from_children(&element.children);
      Ok(LawTitle {
        kana,
        abbrev,
        abbrev_kana,
        text,
      })
    } else {
      Err(Error::wrong_tag_name(element, "LawTitle"))
    }
  }
}

/// 前文
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Preamble {
  pub children: Vec<Paragraph>,
}

impl Parser for Preamble {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Preamble" {
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          if e.name.as_str() == "Paragraph" {
            let v = Paragraph::parser(e)?;
            children.push(v)
          }
        }
      }
      Ok(Preamble { children })
    } else {
      Err(Error::wrong_tag_name(element, "Preamble"))
    }
  }
}

/// 本文
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct MainProvision {
  /// 本文の要素
  pub children: Vec<MainProvisionContents>,
  pub extract: Option<bool>,
}

impl Parser for MainProvision {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "MainProvision" {
      let extract = get_attribute_opt_with_parse(element, "Extract")?;
      let mut children = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Part" => {
              let v = Part::parser(e)?;
              children.push(MainProvisionContents::Part(v))
            }
            "Chapter" => {
              let v = Chapter::parser(e)?;
              children.push(MainProvisionContents::Chapter(v))
            }
            "Section" => {
              let v = Section::parser(e)?;
              children.push(MainProvisionContents::Section(v))
            }
            "Article" => {
              let v = Article::parser(e)?;
              children.push(MainProvisionContents::Article(v))
            }
            "Paragraph" => {
              let v = Paragraph::parser(e)?;
              children.push(MainProvisionContents::Paragraph(v))
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(MainProvision { children, extract })
    } else {
      Err(Error::wrong_tag_name(element, "MainProvision"))
    }
  }
}

/// 本文の要素
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum MainProvisionContents {
  /// 編
  Part(Part),
  /// 章
  Chapter(Chapter),
  /// 節
  Section(Section),
  /// 条
  Article(Article),
  /// 段落
  Paragraph(Paragraph),
}

/// 改正
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct AmendProvision {
  pub sentence: Option<Sentence>,
  pub new_provision: Vec<NewProvision>,
}

impl Parser for AmendProvision {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "AmendProvision" {
      let mut sentence = None;
      let mut new_provision = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "AmendProvisionSentence" => {
              let n = e
                .children
                .first()
                .and_then(|n| {
                  if let XMLNode::Element(e) = n {
                    Some(e)
                  } else {
                    None
                  }
                })
                .ok_or(Error::missing_required_tag("AmendProvision"))?;
              let v = Sentence::parser(n)?;
              sentence = Some(v)
            }
            "NewProvision" => {
              for node in e.children.iter() {
                if let XMLNode::Element(e) = node {
                  match e.name.as_str() {
                    "LawTitle" => {
                      let v = LawTitle::parser(e)?;
                      new_provision.push(NewProvision::LawTitle(v));
                    }
                    "Preamble" => {
                      let v = Preamble::parser(e)?;
                      new_provision.push(NewProvision::Preamble(v));
                    }
                    "TOC" => {
                      let v = TOC::parser(e)?;
                      new_provision.push(NewProvision::TOC(v));
                    }
                    "Part" => {
                      let v = Part::parser(e)?;
                      new_provision.push(NewProvision::Part(v));
                    }
                    "PartTitle" => {
                      new_provision.push(NewProvision::PartTitle(Text::from_children(&e.children)));
                    }
                    "Chapter" => {
                      let v = Chapter::parser(e)?;
                      new_provision.push(NewProvision::Chapter(v));
                    }
                    "ChapterTitle" => {
                      new_provision
                        .push(NewProvision::ChapterTitle(Text::from_children(&e.children)));
                    }
                    "Section" => {
                      let v = Section::parser(e)?;
                      new_provision.push(NewProvision::Section(v));
                    }
                    "SectionTitle" => {
                      new_provision
                        .push(NewProvision::SectionTitle(Text::from_children(&e.children)));
                    }
                    "Subsection" => {
                      let v = Subsection::parser(e)?;
                      new_provision.push(NewProvision::Subsection(v));
                    }
                    "SubsectionTitle" => {
                      new_provision.push(NewProvision::SubsectionTitle(Text::from_children(
                        &e.children,
                      )));
                    }
                    "Division" => {
                      let v = Division::parser(e)?;
                      new_provision.push(NewProvision::Division(v));
                    }
                    "DivisionTitle" => {
                      new_provision.push(NewProvision::DivisionTitle(Text::from_children(
                        &e.children,
                      )));
                    }
                    "Article" => {
                      let v = Article::parser(e)?;
                      new_provision.push(NewProvision::Article(v));
                    }
                    "SupplNote" => {
                      new_provision.push(NewProvision::SupplNote(Text::from_children(&e.children)));
                    }
                    "Paragraph" => {
                      let v = Paragraph::parser(e)?;
                      new_provision.push(NewProvision::Paragraph(v));
                    }
                    "Item" => {
                      let v = Item::parser(e)?;
                      new_provision.push(NewProvision::Item(v));
                    }
                    "Subitem1" => {
                      let v = Subitem1::parser(e)?;
                      new_provision.push(NewProvision::Subitem1(v));
                    }
                    "Subitem2" => {
                      let v = Subitem2::parser(e)?;
                      new_provision.push(NewProvision::Subitem2(v));
                    }
                    "Subitem3" => {
                      let v = Subitem3::parser(e)?;
                      new_provision.push(NewProvision::Subitem3(v));
                    }
                    "Subitem4" => {
                      let v = Subitem4::parser(e)?;
                      new_provision.push(NewProvision::Subitem4(v));
                    }
                    "Subitem5" => {
                      let v = Subitem5::parser(e)?;
                      new_provision.push(NewProvision::Subitem5(v));
                    }
                    "Subitem6" => {
                      let v = Subitem6::parser(e)?;
                      new_provision.push(NewProvision::Subitem6(v));
                    }
                    "Subitem7" => {
                      let v = Subitem7::parser(e)?;
                      new_provision.push(NewProvision::Subitem7(v));
                    }
                    "Subitem8" => {
                      let v = Subitem8::parser(e)?;
                      new_provision.push(NewProvision::Subitem8(v));
                    }
                    "Subitem9" => {
                      let v = Subitem9::parser(e)?;
                      new_provision.push(NewProvision::Subitem9(v));
                    }
                    "Subitem10" => {
                      let v = Subitem10::parser(e)?;
                      new_provision.push(NewProvision::Subitem10(v));
                    }
                    "List" => {
                      let v = List::parser(e)?;
                      new_provision.push(NewProvision::List(v));
                    }
                    "Sentence" => {
                      let v = Sentence::parser(e)?;
                      new_provision.push(NewProvision::Sentence(v));
                    }
                    "AmendProvision" => {
                      let v = AmendProvision::parser(e)?;
                      new_provision.push(NewProvision::AmendProvision(v));
                    }
                    "AppdxTable" => {
                      let v = AppdxTable::parser(e)?;
                      new_provision.push(NewProvision::AppdxTable(v));
                    }
                    "AppdxNote" => {
                      let v = AppdxNote::parser(e)?;
                      new_provision.push(NewProvision::AppdxNote(v));
                    }
                    "AppdxStyle" => {
                      let v = AppdxStyle::parser(e)?;
                      new_provision.push(NewProvision::AppdxStyle(v));
                    }
                    "Appdx" => {
                      let v = Appdx::parser(e)?;
                      new_provision.push(NewProvision::Appdx(v));
                    }
                    "AppdxFig" => {
                      let v = AppdxFig::parser(e)?;
                      new_provision.push(NewProvision::AppdxFig(v));
                    }
                    "AppdxFormat" => {
                      let v = AppdxFormat::parser(e)?;
                      new_provision.push(NewProvision::AppdxFormat(v));
                    }
                    "SupplProvisionAppdxStyle" => {
                      let v = SupplProvisionAppdxStyle::parser(e)?;
                      new_provision.push(NewProvision::SupplProvisionAppdxStyle(v));
                    }
                    "SupplProvisionAppdxTable" => {
                      let v = SupplProvisionAppdxTable::parser(e)?;
                      new_provision.push(NewProvision::SupplProvisionAppdxTable(v));
                    }
                    "SupplProvisionAppdx" => {
                      let v = SupplProvisionAppdx::parser(e)?;
                      new_provision.push(NewProvision::SupplProvisionAppdx(v));
                    }
                    "TableStruct" => {
                      let v = TableStruct::parser(e)?;
                      new_provision.push(NewProvision::TableStruct(v));
                    }
                    "TableRow" => {
                      let v = TableRow::parser(e)?;
                      new_provision.push(NewProvision::TableRow(v));
                    }
                    "TableColumn" => {
                      let v = TableColumn::parser(e)?;
                      new_provision.push(NewProvision::TableColumn(v));
                    }
                    "FigStruct" => {
                      let v = FigStruct::parser(e)?;
                      new_provision.push(NewProvision::FigStruct(v));
                    }
                    "NoteStruct" => {
                      let v = NoteStruct::parser(e)?;
                      new_provision.push(NewProvision::NoteStruct(v));
                    }
                    "StyleStruct" => {
                      let v = StyleStruct::parser(e)?;
                      new_provision.push(NewProvision::StyleStruct(v));
                    }
                    "FormatStruct" => {
                      let v = FormatStruct::parser(e)?;
                      new_provision.push(NewProvision::FormatStruct(v));
                    }
                    "Remarks" => {
                      let v = Remarks::parser(e)?;
                      new_provision.push(NewProvision::Remarks(v));
                    }
                    "LawBody" => {
                      let v = LawBody::parser(e)?;
                      new_provision.push(NewProvision::LawBody(v));
                    }
                    s => return Err(Error::unexpected_tag(e, s)),
                  }
                }
              }
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
        }
      }
      Ok(AmendProvision {
        sentence,
        new_provision,
      })
    } else {
      Err(Error::wrong_tag_name(element, "AmendProvision"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum NewProvision {
  LawTitle(LawTitle),
  Preamble(Preamble),
  TOC(TOC),
  Part(Part),
  PartTitle(Text),
  Chapter(Chapter),
  ChapterTitle(Text),
  Section(Section),
  SectionTitle(Text),
  Subsection(Subsection),
  SubsectionTitle(Text),
  Division(Division),
  DivisionTitle(Text),
  Article(Article),
  SupplNote(Text),
  Paragraph(Paragraph),
  Item(Item),
  Subitem1(Subitem1),
  Subitem2(Subitem2),
  Subitem3(Subitem3),
  Subitem4(Subitem4),
  Subitem5(Subitem5),
  Subitem6(Subitem6),
  Subitem7(Subitem7),
  Subitem8(Subitem8),
  Subitem9(Subitem9),
  Subitem10(Subitem10),
  List(List),
  Sentence(Sentence),
  AmendProvision(AmendProvision),
  AppdxTable(AppdxTable),
  AppdxNote(AppdxNote),
  AppdxStyle(AppdxStyle),
  Appdx(Appdx),
  AppdxFig(AppdxFig),
  AppdxFormat(AppdxFormat),
  SupplProvisionAppdxStyle(SupplProvisionAppdxStyle),
  SupplProvisionAppdxTable(SupplProvisionAppdxTable),
  SupplProvisionAppdx(SupplProvisionAppdx),
  TableStruct(TableStruct),
  TableRow(TableRow),
  TableColumn(TableColumn),
  FigStruct(FigStruct),
  NoteStruct(NoteStruct),
  StyleStruct(StyleStruct),
  FormatStruct(FormatStruct),
  Remarks(Remarks),
  LawBody(LawBody),
}
