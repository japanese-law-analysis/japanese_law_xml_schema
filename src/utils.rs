//! 条文に関する型と関数の定義
//!

use crate::{
  article::{
    Article, Chapter, ChapterContents, Division, Part, PartContents, Section, SectionContents,
    Subsection, SubsectionContents,
  },
  article_number::ArticleNumber,
  class::SentenceOrColumnOrTable,
  contents::ContentsElement,
  law::{MainProvision, MainProvisionContents},
  paragraph::{
    Paragraph, Subitem1, Subitem10, Subitem2, Subitem3, Subitem4, Subitem5, Subitem6, Subitem7,
    Subitem8, Subitem9,
  },
  sentence::SentenceElement,
  suppl_provision::{self, SupplProvision},
  table::{Table, TableColumnContents},
};
use serde::{Deserialize, Serialize};

/// 章番号や節番号などの情報つきの条
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithNumberArticle {
  pub(crate) part: Option<ArticleNumber>,
  pub(crate) chapter: Option<ArticleNumber>,
  pub(crate) section: Option<ArticleNumber>,
  pub(crate) subsection: Option<ArticleNumber>,
  pub(crate) division: Option<ArticleNumber>,
  pub(crate) article: Article,
}

impl WithNumberArticle {
  pub fn new(article: Article) -> Self {
    Self {
      part: None,
      chapter: None,
      section: None,
      subsection: None,
      division: None,
      article,
    }
  }
  pub fn set_part(&mut self, num: Option<ArticleNumber>) {
    self.part = num
  }
  pub fn get_part(&self) -> Option<ArticleNumber> {
    self.part.clone()
  }
  pub fn set_chapter(&mut self, num: Option<ArticleNumber>) {
    self.chapter = num
  }
  pub fn get_chapter(&self) -> Option<ArticleNumber> {
    self.chapter.clone()
  }
  pub fn set_section(&mut self, num: Option<ArticleNumber>) {
    self.section = num
  }
  pub fn get_section(&self) -> Option<ArticleNumber> {
    self.section.clone()
  }
  pub fn set_subsection(&mut self, num: Option<ArticleNumber>) {
    self.subsection = num
  }
  pub fn get_subsection(&self) -> Option<ArticleNumber> {
    self.subsection.clone()
  }
  pub fn set_division(&mut self, num: Option<ArticleNumber>) {
    self.division = num
  }
  pub fn get_division(&self) -> Option<ArticleNumber> {
    self.division.clone()
  }
  pub fn set_articl(&mut self, article: Article) {
    self.article = article
  }
  pub fn get_article(&self) -> Article {
    self.article.clone()
  }
}

/// 条文のリストを返す
/// 項しかない場合や前文は`Vec<Vec<Paragraph>>`に中身が入って返る
pub fn with_number_article_list_from_main_provision(
  main_provision: &MainProvision,
) -> (Vec<WithNumberArticle>, Vec<Vec<Paragraph>>) {
  let mut v = Vec::new();
  let mut para_v = Vec::new();
  let mut para_v_tmp = Vec::new();
  for main in &main_provision.children {
    match main {
      MainProvisionContents::Article(t) => {
        para_v.push(para_v_tmp);
        para_v_tmp = Vec::new();
        let w = WithNumberArticle::new(t.clone());
        v.push(w)
      }
      MainProvisionContents::Part(t) => {
        para_v.push(para_v_tmp);
        para_v_tmp = Vec::new();
        let mut v2 = article_list_from_part(t);
        v.append(&mut v2);
      }
      MainProvisionContents::Chapter(t) => {
        para_v.push(para_v_tmp);
        para_v_tmp = Vec::new();
        let mut v2 = article_list_from_chapter(t);
        v.append(&mut v2);
      }
      MainProvisionContents::Section(t) => {
        para_v.push(para_v_tmp);
        para_v_tmp = Vec::new();
        let mut v2 = article_list_from_section(t);
        v.append(&mut v2);
      }
      MainProvisionContents::Paragraph(t) => {
        para_v_tmp.push(t.clone());
      }
    }
  }
  para_v.push(para_v_tmp);
  (v, para_v)
}

/// 条文のリストを返す
/// 項しかない場合や前文は`Vec<Vec<Paragraph>>`に中身が入って返る
pub fn article_list_from_main_provision(
  main_provision: &MainProvision,
) -> (Vec<Article>, Vec<Vec<Paragraph>>) {
  let (article_v, para_v) = with_number_article_list_from_main_provision(main_provision);
  let a = article_v
    .iter()
    .map(|a| a.get_article())
    .collect::<Vec<_>>();
  (a, para_v)
}

/// 条文のリストを返す
/// 項しかない場合は`Vec<Vec<Paragraph>>`に中身が入って返る
pub fn article_list_from_suppl_provision(
  suppl_provision: &SupplProvision,
) -> (Vec<Article>, Vec<Vec<Paragraph>>) {
  let mut v = Vec::new();
  let mut para_v = Vec::new();
  let mut para_v_tmp = Vec::new();
  for se in suppl_provision.children.iter() {
    match se {
      suppl_provision::SupplProvisionChildrenElement::Article(t) => {
        para_v.push(para_v_tmp);
        para_v_tmp = Vec::new();
        v.push(t.clone())
      }
      suppl_provision::SupplProvisionChildrenElement::Chapter(t) => {
        para_v.push(para_v_tmp);
        para_v_tmp = Vec::new();
        let mut v2 = article_list_from_chapter(t)
          .iter()
          .map(|w| w.get_article())
          .collect();
        v.append(&mut v2);
      }
      suppl_provision::SupplProvisionChildrenElement::Paragraph(t) => {
        para_v_tmp.push(t.clone());
      }
      _ => (),
    }
  }
  para_v.push(para_v_tmp);
  (v, para_v)
}

fn article_list_from_part(part: &Part) -> Vec<WithNumberArticle> {
  let mut v = Vec::new();
  for contents in part.children.iter() {
    match contents {
      PartContents::Chapter(t) => {
        let mut v2 = article_list_from_chapter(t);
        let mut v3 = v2
          .iter_mut()
          .map(|w| {
            w.set_part(Some(t.num.clone()));
            w.clone()
          })
          .collect::<Vec<_>>();
        v.append(&mut v3)
      }
      PartContents::Article(t) => {
        let mut w = WithNumberArticle::new(t.clone());
        w.set_part(Some(t.num.clone()));
        v.push(w)
      }
    }
  }
  v
}

fn article_list_from_chapter(t: &Chapter) -> Vec<WithNumberArticle> {
  let mut v = Vec::new();
  for contents in t.children.iter() {
    match contents {
      ChapterContents::Section(t) => {
        let mut v2 = article_list_from_section(t);
        let mut v3 = v2
          .iter_mut()
          .map(|w| {
            w.set_chapter(Some(t.num.clone()));
            w.clone()
          })
          .collect::<Vec<_>>();
        v.append(&mut v3)
      }
      ChapterContents::Article(t) => {
        let mut w = WithNumberArticle::new(t.clone());
        w.set_chapter(Some(t.num.clone()));
        v.push(w)
      }
    }
  }
  v
}

fn article_list_from_section(t: &Section) -> Vec<WithNumberArticle> {
  let mut v = Vec::new();
  for contents in t.children.iter() {
    match contents {
      SectionContents::Subsection(t) => {
        let mut v2 = article_list_from_subsection(t);
        let mut v3 = v2
          .iter_mut()
          .map(|w| {
            w.set_section(Some(t.num.clone()));
            w.clone()
          })
          .collect::<Vec<_>>();
        v.append(&mut v3)
      }
      SectionContents::Division(t) => {
        let mut v2 = article_list_from_division(t);
        let mut v3 = v2
          .iter_mut()
          .map(|w| {
            w.set_section(Some(t.num.clone()));
            w.clone()
          })
          .collect::<Vec<_>>();
        v.append(&mut v3)
      }
      SectionContents::Article(t) => {
        let mut w = WithNumberArticle::new(t.clone());
        w.set_section(Some(t.num.clone()));
        v.push(w)
      }
    }
  }
  v
}

#[allow(clippy::too_many_arguments)]
fn article_list_from_subsection(t: &Subsection) -> Vec<WithNumberArticle> {
  let mut v = Vec::new();
  for contents in t.children.iter() {
    match contents {
      SubsectionContents::Division(t) => {
        let mut v2 = article_list_from_division(t);
        let mut v3 = v2
          .iter_mut()
          .map(|w| {
            w.set_subsection(Some(t.num.clone()));
            w.clone()
          })
          .collect::<Vec<_>>();
        v.append(&mut v3)
      }
      SubsectionContents::Article(t) => {
        let mut w = WithNumberArticle::new(t.clone());
        w.set_subsection(Some(t.num.clone()));
        v.push(w)
      }
    }
  }
  v
}

#[allow(clippy::too_many_arguments)]
fn article_list_from_division(t: &Division) -> Vec<WithNumberArticle> {
  let mut v = Vec::new();
  for a in t.children.iter() {
    let mut w = WithNumberArticle::new(a.clone());
    w.set_division(Some(t.num.clone()));
    v.push(w)
  }
  v
}

/// 条文内でのテキストの位置を示す
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextIndex {
  /// 段落番号
  pub paragraph: ArticleNumber,
  /// 号の番号を上の階層から並べる
  /// 何もないときは空
  pub items: Vec<Option<ArticleNumber>>,
}

/// 条から中身の文字列を生成する
pub fn text_from_article(article: &Article) -> String {
  let text_info_list = text_info_list_from_paragraph(&article.paragraph);
  text_info_list
    .iter()
    .map(|(_, s)| s.clone())
    .collect::<Vec<String>>()
    .join("\n")
}

/// 項のリストから中身の文字列を生成する
pub fn text_from_paragraph_list(list: &[Paragraph]) -> String {
  let text_info_list = text_info_list_from_paragraph(list);
  text_info_list
    .iter()
    .map(|(_, s)| s.clone())
    .collect::<Vec<String>>()
    .join("\n")
}

/// 段落のリストから文字列のリストとそのインデックスの組を生成する
/// 線は無視し、ルビはテキストのみを取得し、上付き文字は`^`、下付き文字は`_`で出力する
pub fn text_info_list_from_paragraph(lst: &[Paragraph]) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for para in lst.iter() {
    let paragraph_num = &para.num;
    let sentence_text = para
      .sentence
      .iter()
      .map(|sentence| sentence_element_to_str(&sentence.contents))
      .collect::<String>();
    v.push((
      TextIndex {
        paragraph: paragraph_num.clone(),
        items: Vec::new(),
      },
      sentence_text,
    ));
    let mut items = Vec::new();
    for item in para.children.iter() {
      let n = &item.num;
      items.push(n.clone());
      // childrenとsentenceの処理をする
      let sentence_str = match &item.sentence {
        SentenceOrColumnOrTable::Sentence(se) => se
          .iter()
          .map(|sentence| sentence_element_to_str(&sentence.contents))
          .collect::<String>(),
        SentenceOrColumnOrTable::Column(cs) => cs
          .iter()
          .flat_map(|column| {
            column
              .sentence
              .iter()
              .map(|sentence| sentence_element_to_str(&sentence.contents))
          })
          .collect::<Vec<_>>()
          .join(" "),
        SentenceOrColumnOrTable::Table(table) => table_to_str(table),
      };
      v.push((
        TextIndex {
          paragraph: paragraph_num.clone(),
          items: items.clone(),
        },
        sentence_str,
      ));
      let mut v2 = text_list_from_subitem1(paragraph_num, items.clone(), &item.children);
      v.append(&mut v2);
      items.pop();
    }
  }
  v
}

fn text_list_from_subitem1(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem1],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem2(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem2(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem2],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem3(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem3(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem3],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem4(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem4(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem4],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem5(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem5(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem5],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem6(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem6(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem6],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(_) => String::new(),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem7(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem7(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem7],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem8(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem8(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem8],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem9(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem9(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem9],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l.clone(),
      },
      sentence_str,
    ));
    let mut v2 = text_list_from_subitem10(para_num, l.clone(), &t.children);
    v.append(&mut v2);
  }
  v
}

fn text_list_from_subitem10(
  para_num: &ArticleNumber,
  items: Vec<Option<ArticleNumber>>,
  chldren: &[Subitem10],
) -> Vec<(TextIndex, String)> {
  let mut v = Vec::new();
  for t in chldren.iter() {
    let mut l = items.clone();
    l.push(t.num.clone());
    let sentence_str = match &t.sentence {
      SentenceOrColumnOrTable::Sentence(se) => se
        .iter()
        .map(|sentence| sentence_element_to_str(&sentence.contents))
        .collect::<String>(),
      SentenceOrColumnOrTable::Column(cs) => cs
        .iter()
        .flat_map(|column| {
          column
            .sentence
            .iter()
            .map(|sentence| sentence_element_to_str(&sentence.contents))
        })
        .collect::<Vec<_>>()
        .join(" "),
      SentenceOrColumnOrTable::Table(table) => table_to_str(table),
    };
    v.push((
      TextIndex {
        paragraph: para_num.clone(),
        items: l,
      },
      sentence_str,
    ));
  }
  v
}

pub fn table_to_str(table: &Table) -> String {
  let header_row_str = table
    .table_header_row
    .iter()
    .map(|row| {
      row
        .columns
        .iter()
        .map(|text| text.to_string())
        .collect::<String>()
    })
    .collect::<Vec<_>>()
    .join(" ");
  let row = table
    .table_row
    .iter()
    .map(|row| {
      row
        .columns
        .iter()
        .map(|column| {
          column
            .contents
            .iter()
            .map(|contents| {
              match contents {
                TableColumnContents::String(s) => s.clone(),
                TableColumnContents::Sentence(se) => sentence_element_to_str(&se.contents),
                TableColumnContents::Column(c) => c
                  .sentence
                  .iter()
                  .map(|sentence| sentence_element_to_str(&sentence.contents))
                  .collect::<String>(),
                _ => String::new(), // unsupported
              }
            })
            .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
    })
    .collect::<Vec<_>>()
    .join("\n");
  format!("{header_row_str}\n{row}")
}

pub fn sentence_element_to_str(element: &[SentenceElement]) -> String {
  let mut s = String::new();
  for e in element.iter() {
    match e {
      SentenceElement::String(s2) => s.push_str(s2),
      SentenceElement::Ruby(ruby) => {
        s.push_str(&ruby.text.to_string());
      }
      SentenceElement::Sub(s2) => {
        s.push_str("_{");
        s.push_str(&s2.text);
        s.push('}');
      }
      SentenceElement::Sup(s2) => {
        s.push_str("^{");
        s.push_str(&s2.text);
        s.push('}');
      }
      SentenceElement::ArithFormula(arith_formula) => {
        let contents = &arith_formula.contentes.contents;
        for c in contents.iter() {
          match c {
            ContentsElement::String(s2) => s.push_str(s2),
            ContentsElement::Ruby(ruby) => {
              s.push_str(&ruby.text.to_string());
            }
            ContentsElement::Sub(s2) => {
              s.push_str("_{");
              s.push_str(&s2.text);
              s.push('}');
            }
            ContentsElement::Sup(s2) => {
              s.push_str("^{");
              s.push_str(&s2.text);
              s.push('}');
            }
            _ => (),
          }
        }
      }
      SentenceElement::QuoteStruct(quote_struct) => {
        let contents = &quote_struct.contentes.contents;
        for c in contents.iter() {
          match c {
            ContentsElement::String(s2) => s.push_str(s2),
            ContentsElement::Ruby(ruby) => {
              s.push_str(&ruby.text.to_string());
            }
            ContentsElement::Sub(s2) => {
              s.push_str("_{");
              s.push_str(&s2.text);
              s.push('}');
            }
            ContentsElement::Sup(s2) => {
              s.push_str("^{");
              s.push_str(&s2.text);
              s.push('}');
            }
            _ => (),
          }
        }
      }
      _ => (),
    }
  }
  s
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Toc {
  pub(crate) part_number: Option<ArticleNumber>,
  pub(crate) chapter_number: Option<ArticleNumber>,
  pub(crate) section_number: Option<ArticleNumber>,
  pub(crate) subsection_number: Option<ArticleNumber>,
  pub(crate) division_number: Option<ArticleNumber>,
  pub(crate) article_number: Option<ArticleNumber>,
  pub(crate) title: Option<String>,
}

impl Toc {
  pub fn set_part(&mut self, num: Option<ArticleNumber>) {
    self.part_number = num;
  }
  pub fn set_chapter(&mut self, num: Option<ArticleNumber>) {
    self.chapter_number = num;
  }
  pub fn set_section(&mut self, num: Option<ArticleNumber>) {
    self.section_number = num;
  }
  pub fn set_subsection(&mut self, num: Option<ArticleNumber>) {
    self.subsection_number = num;
  }
  pub fn set_division(&mut self, num: Option<ArticleNumber>) {
    self.division_number = num;
  }
  pub fn set_article(&mut self, num: Option<ArticleNumber>) {
    self.division_number = num;
  }
  pub fn set_title(&mut self, title: Option<String>) {
    self.title = title;
  }
  pub fn get_part(&self) -> Option<ArticleNumber> {
    self.part_number.clone()
  }
  pub fn get_chapter(&self) -> Option<ArticleNumber> {
    self.chapter_number.clone()
  }
  pub fn get_section(&self) -> Option<ArticleNumber> {
    self.section_number.clone()
  }
  pub fn get_subsection(&self) -> Option<ArticleNumber> {
    self.subsection_number.clone()
  }
  pub fn get_division(&self) -> Option<ArticleNumber> {
    self.division_number.clone()
  }
  pub fn get_article(&self) -> Option<ArticleNumber> {
    self.article_number.clone()
  }
  pub fn get_title(&self) -> Option<String> {
    self.title.clone()
  }
}

pub fn toc_list_from_main_provision(main_provision: &MainProvision) -> Vec<Toc> {
  let mut v = Vec::new();
  for children in main_provision.children.iter() {
    match children {
      MainProvisionContents::Article(t) => {
        let mut toc = Toc::default();
        toc.set_article(Some(t.num.clone()));
        toc.set_title(Some(t.title.to_string()));
        v.push(toc.clone());
      }
      MainProvisionContents::Part(t) => {
        let mut v2 = toc_list_from_part(t);
        v.append(&mut v2);
      }
      MainProvisionContents::Chapter(t) => {
        let mut toc = Toc::default();
        let mut v2 = toc_list_from_chapter(t, &mut toc);
        v.append(&mut v2);
      }
      MainProvisionContents::Section(t) => {
        let mut toc = Toc::default();
        let mut v2 = toc_list_from_section(t, &mut toc);
        v.append(&mut v2);
      }
      _ => (),
    }
  }
  v
}

fn toc_list_from_part(t: &Part) -> Vec<Toc> {
  let mut v = Vec::new();
  let mut toc = Toc::default();
  let num = &t.num;
  toc.set_part(Some(num.clone()));
  toc.set_title(Some(t.part_title.to_string()));
  v.push(toc.clone());
  for children in t.children.iter() {
    match children {
      PartContents::Article(t) => {
        toc.set_article(Some(t.num.clone()));
        toc.set_title(Some(t.title.to_string()));
        v.push(toc.clone());
      }
      PartContents::Chapter(t) => {
        let mut toc_list = toc_list_from_chapter(t, &mut toc);
        v.append(&mut toc_list);
      }
    }
  }
  v
}

fn toc_list_from_chapter(t: &Chapter, toc: &mut Toc) -> Vec<Toc> {
  let mut v = Vec::new();
  let num = &t.num;
  toc.set_chapter(Some(num.clone()));
  toc.set_title(Some(t.chapter_title.to_string()));
  v.push(toc.clone());
  for children in t.children.iter() {
    match children {
      ChapterContents::Article(t) => {
        toc.set_article(Some(t.num.clone()));
        toc.set_title(Some(t.title.to_string()));
        v.push(toc.clone());
      }
      ChapterContents::Section(t) => {
        let mut toc_list = toc_list_from_section(t, &mut toc.clone());
        v.append(&mut toc_list);
      }
    }
  }
  v
}

fn toc_list_from_section(t: &Section, toc: &mut Toc) -> Vec<Toc> {
  let mut v = Vec::new();
  let num = &t.num;
  toc.set_section(Some(num.clone()));
  toc.set_title(Some(t.section_title.to_string()));
  v.push(toc.clone());
  for children in t.children.iter() {
    match children {
      SectionContents::Article(t) => {
        toc.set_article(Some(t.num.clone()));
        toc.set_title(Some(t.title.to_string()));
        v.push(toc.clone());
      }
      SectionContents::Subsection(t) => {
        let mut toc_list = toc_list_from_subsection(t, &mut toc.clone());
        v.append(&mut toc_list);
      }
      SectionContents::Division(t) => {
        let mut toc_list = toc_list_from_division(t, &mut toc.clone());
        v.append(&mut toc_list);
      }
    }
  }
  v
}

fn toc_list_from_subsection(t: &Subsection, toc: &mut Toc) -> Vec<Toc> {
  let mut v = Vec::new();
  let num = &t.num;
  toc.set_subsection(Some(num.clone()));
  toc.set_title(Some(t.subsection_title.to_string()));
  v.push(toc.clone());
  for children in t.children.iter() {
    match children {
      SubsectionContents::Article(t) => {
        toc.set_article(Some(t.num.clone()));
        toc.set_title(Some(t.title.to_string()));
        v.push(toc.clone());
      }
      SubsectionContents::Division(t) => {
        let mut toc_list = toc_list_from_division(t, &mut toc.clone());
        v.append(&mut toc_list);
      }
    }
  }
  v
}

fn toc_list_from_division(t: &Division, toc: &mut Toc) -> Vec<Toc> {
  let mut v = Vec::new();
  let num = &t.num;
  toc.set_division(Some(num.clone()));
  toc.set_title(Some(t.division_title.to_string()));
  v.push(toc.clone());
  for t in t.children.iter() {
    toc.set_article(Some(t.num.clone()));
    toc.set_title(Some(t.title.to_string()));
    v.push(toc.clone());
  }
  v
}

#[test]
fn check_para_to_text() {
  use crate::{class, paragraph, sentence, text};
  fn text_to_sentence(num: usize, text: &str) -> sentence::Sentence {
    sentence::Sentence {
      contents: vec![sentence::SentenceElement::String(text.to_string())],
      num: Some(num),
      function: None,
      indent: None,
      writing_mode: text::WritingMode::Vertical,
    }
  }
  let para_lst = vec![
    paragraph::Paragraph {
      caption: None,
      paragraph_num: text::Text{contents: Vec::new()},
      amend_provision: Vec::new(),
      class: Vec::new(),
      sentence: vec![sentence::Sentence {
        contents: vec![sentence::SentenceElement::String(
          "被保佐人が次に掲げる行為をするには、その保佐人の同意を得なければならない。ただし、第九条ただし書に規定する行為については、この限りでない。"
            .to_string()
        )],
        num: Some(1),
        function: None,
        indent: None,
        writing_mode: text::WritingMode::Vertical
      }],
      struct_list: Vec::new(),
      children: vec![
        paragraph::Item {
          title: None,
          sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "元本を領収し、又は利用すること。")]),
          children: Vec::new(),
          struct_list: Vec::new(),
          num: Some(ArticleNumber::from_num_str("1").unwrap()),
          delete: false,
          hide: false
        },
        paragraph::Item {
          title: None,
          sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "主たる債務者が法人である場合の次に掲げる者")]),
          children: vec![
            paragraph::Subitem1 {
              title: None,
              sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "主たる債務者の総株主の議決権（株主総会において決議をすることができる事項の全部につき議決権を行使することができない株式についての議決権を除く。以下この号において同じ。）の過半数を有する者")]),
              children: Vec::new(),
              struct_list: Vec::new(),
              num: Some(ArticleNumber::from_num_str("1").unwrap()),
              delete: false,
              hide: false
            }
          ],
          struct_list: Vec::new(),
          num: Some(ArticleNumber::from_num_str("2").unwrap()),
          delete: false,
          hide: false
        },
        paragraph::Item {
          title: None,
          sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "不動産その他重要な財産に関する権利の得喪を目的とする行為をすること。")]),
          children: Vec::new(),
          struct_list: Vec::new(),
          num: Some(ArticleNumber::from_num_str("3").unwrap()),
          delete: false,
          hide: false
        }
      ],
      num: ArticleNumber::from_num_str("1").unwrap(),
      old_style: false,
      old_num: false,
      hide: false,
    },paragraph::Paragraph {
      caption: None,
      paragraph_num: text::Text{contents: Vec::new()},
      amend_provision: Vec::new(),
      class: Vec::new(),
      sentence: vec![sentence::Sentence {
        contents: vec![sentence::SentenceElement::String(
          "家庭裁判所は、第十一条本文に規定する者又は保佐人若しくは保佐監督人の請求により、被保佐人が前項各号に掲げる行為以外の行為をする場合であってもその保佐人の同意を得なければならない旨の審判をすることができる。ただし、第九条ただし書に規定する行為については、この限りでない。"
            .to_string()
        )],
        num: Some(1),
        function: None,
        indent: None,
        writing_mode: text::WritingMode::Vertical
      }],
      struct_list: Vec::new(),
      children: Vec::new(),
      num: ArticleNumber::from_num_str("2").unwrap(),
      old_style: false,
      old_num: false,
      hide: false,
    }
  ];
  let text_lst = text_info_list_from_paragraph(&para_lst);
  assert_eq!(
    text_lst,
    vec![
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: Vec::new()
        },
        "被保佐人が次に掲げる行為をするには、その保佐人の同意を得なければならない。ただし、第九条ただし書に規定する行為については、この限りでない。".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: vec![Some(ArticleNumber{base_number:1,eda_numbers:Vec::new(),range_end_numbers:Vec::new()})]
        },
        "元本を領収し、又は利用すること。".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: vec![Some(ArticleNumber{base_number:2,eda_numbers:Vec::new(),range_end_numbers:Vec::new()})]
        },
        "主たる債務者が法人である場合の次に掲げる者".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: vec![Some(ArticleNumber{base_number:2,eda_numbers:Vec::new(),range_end_numbers:Vec::new()}),
          Some(ArticleNumber{base_number:1,eda_numbers:Vec::new(),range_end_numbers:Vec::new()})]
        },
        "主たる債務者の総株主の議決権（株主総会において決議をすることができる事項の全部につき議決権を行使することができない株式についての議決権を除く。以下この号において同じ。）の過半数を有する者".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 1,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: vec![Some(ArticleNumber{base_number:3,eda_numbers:Vec::new(),range_end_numbers:Vec::new()})]
        },
        "不動産その他重要な財産に関する権利の得喪を目的とする行為をすること。".to_string()
      ),
      (
        TextIndex {
          paragraph: ArticleNumber {
            base_number: 2,
            eda_numbers: Vec::new(),
            range_end_numbers: Vec::new()
          },
          items: Vec::new()
        },
        "家庭裁判所は、第十一条本文に規定する者又は保佐人若しくは保佐監督人の請求により、被保佐人が前項各号に掲げる行為以外の行為をする場合であってもその保佐人の同意を得なければならない旨の審判をすることができる。ただし、第九条ただし書に規定する行為については、この限りでない。".to_string()
      ),
    ]
  );
}
