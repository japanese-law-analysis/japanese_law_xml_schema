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
use roxmltree::Node;
use serde::{Deserialize, Serialize};

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

impl Parser for Law {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Law" {
      let era = match node.attribute("Era") {
        Some("Meiji") => Era::Meiji,
        Some("Taisho") => Era::Taisho,
        Some("Showa") => Era::Showa,
        Some("Heisei") => Era::Heisei,
        Some("Reiwa") => Era::Reiwa,
        _ => {
          return Err(Error::AttributeParseError {
            range: node.range(),
            tag_name: node.tag_name().name().to_string(),
            attribute_name: "Era".to_string(),
          })
        }
      };
      let year = get_attribute_with_parse(node, "Year")?;
      let num = get_attribute_with_parse(node, "Num")?;
      let promulgate_month = get_attribute_opt_with_parse(node, "PromulgateMonth")?;
      let promulgate_day = get_attribute_opt_with_parse(node, "PromulgateDay")?;
      let law_type = match node.attribute("LawType") {
        Some("Constitution") => LawType::Constitution,
        Some("Act") => LawType::Act,
        Some("CabinetOrder") => LawType::CabinetOrder,
        Some("ImperialOrder") => LawType::ImperialOrder,
        Some("MinisterialOrdinance") => LawType::MinisterialOrdinance,
        Some("Rule") => LawType::Rule,
        Some("Misc") => LawType::Misc,
        _ => {
          return Err(Error::AttributeParseError {
            range: node.range(),
            tag_name: node.tag_name().name().to_string(),
            attribute_name: "LawType".to_string(),
          })
        }
      };
      let lang = match node.attribute("Lang") {
        Some("ja") => Lang::Ja,
        Some("en") => Lang::En,
        _ => {
          return Err(Error::AttributeParseError {
            range: node.range(),
            tag_name: node.tag_name().name().to_string(),
            attribute_name: "Lang".to_string(),
          })
        }
      };
      let mut children = node.children();
      let law_num_node = children
        .next()
        .ok_or(Error::missing_required_tag(&node.range(), "LawNum"))?;
      let law_num_node_tag_name = law_num_node.tag_name().name();
      let law_num = if law_num_node_tag_name == "LawNum" {
        law_num_node.text().unwrap_or_default().to_string()
      } else {
        return Err(Error::UnexpectedTag {
          range: law_num_node.range(),
          wrong_name: law_num_node_tag_name.to_string(),
          tag: "LawNum".to_string(),
        });
      };
      let law_body_node = children
        .next()
        .ok_or(Error::missing_required_tag(&node.range(), "LawBody"))?;
      let law_body = LawBody::parser(&law_body_node)?;
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
      Err(Error::wrong_tag_name(node, "Law"))
    }
  }
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
  enact_statement: Vec<Text>,
  /// 主題
  subject: Option<String>,
  /// 目次
  toc: Option<TOC>,
  /// 前文
  preamble: Option<Preamble>,
  /// 本文
  main_provision: MainProvision,
  /// 附則
  suppl_provision: Vec<SupplProvision>,
  /// 付録表
  appdx_table: Vec<AppdxTable>,
  /// 付録記載
  appdx_note: Vec<AppdxNote>,
  /// 付録様式
  appdx_style: Vec<AppdxStyle>,
  /// 付録
  appdx: Vec<Appdx>,
  /// 付録図
  appdx_fig: Vec<AppdxFig>,
  /// 付録書式
  appdx_format: Vec<AppdxFormat>,
}

impl Parser for LawBody {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "LawBody" {
      let subject = get_attribute_opt_with_parse(node, "Subject")?;
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
      for node in node.children() {
        match node.tag_name().name() {
          "LawTitle" => {
            let v = LawTitle::parser(&node)?;
            law_title = Some(v);
          }
          "EnactStatement" => enact_statement.push(Text::from_children(node.children())),
          "TOC" => {
            let v = TOC::parser(&node)?;
            toc = Some(v);
          }
          "Preamble" => {
            let v = Preamble::parser(&node)?;
            preamble = Some(v);
          }
          "MainProvision" => {
            let v = MainProvision::parser(&node)?;
            main_provision = Some(v);
          }
          "SupplProvision" => {
            let v = SupplProvision::parser(&node)?;
            suppl_provision.push(v);
          }
          "AppdxTable" => {
            let v = AppdxTable::parser(&node)?;
            appdx_table.push(v);
          }
          "AppdxNote" => {
            let v = AppdxNote::parser(&node)?;
            appdx_note.push(v);
          }
          "AppdxStyle" => {
            let v = AppdxStyle::parser(&node)?;
            appdx_style.push(v);
          }
          "Appdx" => {
            let v = Appdx::parser(&node)?;
            appdx.push(v);
          }
          "AppdxFig" => {
            let v = AppdxFig::parser(&node)?;
            appdx_fig.push(v);
          }
          "AppdxFormat" => {
            let v = AppdxFormat::parser(&node)?;
            appdx_format.push(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
        Err(Error::missing_required_tag(&node.range(), "MainProvision"))
      }
    } else {
      Err(Error::wrong_tag_name(node, "LawBody"))
    }
  }
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
  text: Text,
}

impl Parser for LawTitle {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "LawTitle" {
      let kana = get_attribute_opt_with_parse(node, "Kana")?;
      let abbrev = get_attribute_opt_with_parse(node, "Abbrev")?;
      let abbrev_kana = get_attribute_opt_with_parse(node, "AbbrevKana")?;
      let text = Text::from_children(node.children());
      Ok(LawTitle {
        kana,
        abbrev,
        abbrev_kana,
        text,
      })
    } else {
      Err(Error::wrong_tag_name(node, "LawTitle"))
    }
  }
}

/// 前文
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Preamble {
  children: Vec<Paragraph>,
}

impl Parser for Preamble {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Preamble" {
      let mut children = Vec::new();
      for node in node.children() {
        if node.tag_name().name() == "Paragraph" {
          let v = Paragraph::parser(&node)?;
          children.push(v)
        }
      }
      Ok(Preamble { children })
    } else {
      Err(Error::wrong_tag_name(node, "Preamble"))
    }
  }
}

/// 本文
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct MainProvision {
  /// 本文の要素
  children: Vec<MainProvisionContents>,
  extract: Option<bool>,
}

impl Parser for MainProvision {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "MainProvision" {
      let extract = get_attribute_opt_with_parse(node, "Extract")?;
      let mut children = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "Part" => {
            let v = Part::parser(&node)?;
            children.push(MainProvisionContents::Part(v))
          }
          "Chapter" => {
            let v = Chapter::parser(&node)?;
            children.push(MainProvisionContents::Chapter(v))
          }
          "Section" => {
            let v = Section::parser(&node)?;
            children.push(MainProvisionContents::Section(v))
          }
          "Article" => {
            let v = Article::parser(&node)?;
            children.push(MainProvisionContents::Article(v))
          }
          "Paragraph" => {
            let v = Paragraph::parser(&node)?;
            children.push(MainProvisionContents::Paragraph(v))
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(MainProvision { children, extract })
    } else {
      Err(Error::wrong_tag_name(node, "MainProvision"))
    }
  }
}

/// 本文の要素
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AmendProvision {
  sentence: Option<Sentence>,
  new_provision: Vec<NewProvision>,
}

impl Parser for AmendProvision {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "AmendProvision" {
      let mut sentence = None;
      let mut new_provision = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "AmendProvisionSentence" => {
            let n = node
              .children()
              .next()
              .ok_or(Error::missing_required_tag(&node.range(), "AmendProvision"))?;
            let v = Sentence::parser(&n)?;
            sentence = Some(v)
          }
          "NewProvision" => {
            for node in node.children() {
              match node.tag_name().name() {
                "LawTitle" => {
                  let v = LawTitle::parser(&node)?;
                  new_provision.push(NewProvision::LawTitle(v));
                }
                "Preamble" => {
                  let v = Preamble::parser(&node)?;
                  new_provision.push(NewProvision::Preamble(v));
                }
                "TOC" => {
                  let v = TOC::parser(&node)?;
                  new_provision.push(NewProvision::TOC(v));
                }
                "Part" => {
                  let v = Part::parser(&node)?;
                  new_provision.push(NewProvision::Part(v));
                }
                "PartTitle" => {
                  new_provision.push(NewProvision::PartTitle(Text::from_children(
                    node.children(),
                  )));
                }
                "Chapter" => {
                  let v = Chapter::parser(&node)?;
                  new_provision.push(NewProvision::Chapter(v));
                }
                "ChapterTitle" => {
                  new_provision.push(NewProvision::ChapterTitle(Text::from_children(
                    node.children(),
                  )));
                }
                "Section" => {
                  let v = Section::parser(&node)?;
                  new_provision.push(NewProvision::Section(v));
                }
                "SectionTitle" => {
                  new_provision.push(NewProvision::SectionTitle(Text::from_children(
                    node.children(),
                  )));
                }
                "Subsection" => {
                  let v = Subsection::parser(&node)?;
                  new_provision.push(NewProvision::Subsection(v));
                }
                "SubsectionTitle" => {
                  new_provision.push(NewProvision::SubsectionTitle(Text::from_children(
                    node.children(),
                  )));
                }
                "Division" => {
                  let v = Division::parser(&node)?;
                  new_provision.push(NewProvision::Division(v));
                }
                "DivisionTitle" => {
                  new_provision.push(NewProvision::DivisionTitle(Text::from_children(
                    node.children(),
                  )));
                }
                "Article" => {
                  let v = Article::parser(&node)?;
                  new_provision.push(NewProvision::Article(v));
                }
                "SupplNote" => {
                  new_provision.push(NewProvision::SupplNote(Text::from_children(
                    node.children(),
                  )));
                }
                "Paragraph" => {
                  let v = Paragraph::parser(&node)?;
                  new_provision.push(NewProvision::Paragraph(v));
                }
                "Item" => {
                  let v = Item::parser(&node)?;
                  new_provision.push(NewProvision::Item(v));
                }
                "Subitem1" => {
                  let v = Subitem1::parser(&node)?;
                  new_provision.push(NewProvision::Subitem1(v));
                }
                "Subitem2" => {
                  let v = Subitem2::parser(&node)?;
                  new_provision.push(NewProvision::Subitem2(v));
                }
                "Subitem3" => {
                  let v = Subitem3::parser(&node)?;
                  new_provision.push(NewProvision::Subitem3(v));
                }
                "Subitem4" => {
                  let v = Subitem4::parser(&node)?;
                  new_provision.push(NewProvision::Subitem4(v));
                }
                "Subitem5" => {
                  let v = Subitem5::parser(&node)?;
                  new_provision.push(NewProvision::Subitem5(v));
                }
                "Subitem6" => {
                  let v = Subitem6::parser(&node)?;
                  new_provision.push(NewProvision::Subitem6(v));
                }
                "Subitem7" => {
                  let v = Subitem7::parser(&node)?;
                  new_provision.push(NewProvision::Subitem7(v));
                }
                "Subitem8" => {
                  let v = Subitem8::parser(&node)?;
                  new_provision.push(NewProvision::Subitem8(v));
                }
                "Subitem9" => {
                  let v = Subitem9::parser(&node)?;
                  new_provision.push(NewProvision::Subitem9(v));
                }
                "Subitem10" => {
                  let v = Subitem10::parser(&node)?;
                  new_provision.push(NewProvision::Subitem10(v));
                }
                "List" => {
                  let v = List::parser(&node)?;
                  new_provision.push(NewProvision::List(v));
                }
                "Sentence" => {
                  let v = Sentence::parser(&node)?;
                  new_provision.push(NewProvision::Sentence(v));
                }
                "AmendProvision" => {
                  let v = AmendProvision::parser(&node)?;
                  new_provision.push(NewProvision::AmendProvision(v));
                }
                "AppdxTable" => {
                  let v = AppdxTable::parser(&node)?;
                  new_provision.push(NewProvision::AppdxTable(v));
                }
                "AppdxNote" => {
                  let v = AppdxNote::parser(&node)?;
                  new_provision.push(NewProvision::AppdxNote(v));
                }
                "AppdxStyle" => {
                  let v = AppdxStyle::parser(&node)?;
                  new_provision.push(NewProvision::AppdxStyle(v));
                }
                "Appdx" => {
                  let v = Appdx::parser(&node)?;
                  new_provision.push(NewProvision::Appdx(v));
                }
                "AppdxFig" => {
                  let v = AppdxFig::parser(&node)?;
                  new_provision.push(NewProvision::AppdxFig(v));
                }
                "AppdxFormat" => {
                  let v = AppdxFormat::parser(&node)?;
                  new_provision.push(NewProvision::AppdxFormat(v));
                }
                "SupplProvisionAppdxStyle" => {
                  let v = SupplProvisionAppdxStyle::parser(&node)?;
                  new_provision.push(NewProvision::SupplProvisionAppdxStyle(v));
                }
                "SupplProvisionAppdxTable" => {
                  let v = SupplProvisionAppdxTable::parser(&node)?;
                  new_provision.push(NewProvision::SupplProvisionAppdxTable(v));
                }
                "SupplProvisionAppdx" => {
                  let v = SupplProvisionAppdx::parser(&node)?;
                  new_provision.push(NewProvision::SupplProvisionAppdx(v));
                }
                "TableStruct" => {
                  let v = TableStruct::parser(&node)?;
                  new_provision.push(NewProvision::TableStruct(v));
                }
                "TableRow" => {
                  let v = TableRow::parser(&node)?;
                  new_provision.push(NewProvision::TableRow(v));
                }
                "TableColumn" => {
                  let v = TableColumn::parser(&node)?;
                  new_provision.push(NewProvision::TableColumn(v));
                }
                "FigStruct" => {
                  let v = FigStruct::parser(&node)?;
                  new_provision.push(NewProvision::FigStruct(v));
                }
                "NoteStruct" => {
                  let v = NoteStruct::parser(&node)?;
                  new_provision.push(NewProvision::NoteStruct(v));
                }
                "StyleStruct" => {
                  let v = StyleStruct::parser(&node)?;
                  new_provision.push(NewProvision::StyleStruct(v));
                }
                "FormatStruct" => {
                  let v = FormatStruct::parser(&node)?;
                  new_provision.push(NewProvision::FormatStruct(v));
                }
                "Remarks" => {
                  let v = Remarks::parser(&node)?;
                  new_provision.push(NewProvision::Remarks(v));
                }
                "LawBody" => {
                  let v = LawBody::parser(&node)?;
                  new_provision.push(NewProvision::LawBody(v));
                }
                s => return Err(Error::unexpected_tag(&node, s)),
              }
            }
          }
          s => return Err(Error::unexpected_tag(&node, s)),
        }
      }
      Ok(AmendProvision {
        sentence,
        new_provision,
      })
    } else {
      Err(Error::wrong_tag_name(node, "AmendProvision"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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
