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
};
use serde::{Deserialize, Serialize};

/// 条文のリストを返す
/// 項しかない場合や前文は`Vec<Vec<Paragraph>>`に中身が入って返る
pub fn article_list_from_main_provision(
  main_provision: &MainProvision,
) -> (Vec<Article>, Vec<Vec<Paragraph>>) {
  let mut v = Vec::new();
  let mut para_v = Vec::new();
  let mut para_v_tmp = Vec::new();
  for main in &main_provision.children {
    match main {
      MainProvisionContents::Article(t) => {
        para_v.push(para_v_tmp);
        para_v_tmp = Vec::new();
        v.push(t.clone())
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
        let mut v2 = article_list_from_chapter(t);
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

fn article_list_from_part(part: &Part) -> Vec<Article> {
  let mut v = Vec::new();
  for contents in part.children.iter() {
    match contents {
      PartContents::Chapter(t) => {
        let mut v2 = article_list_from_chapter(t);
        v.append(&mut v2)
      }
      PartContents::Article(t) => v.push(t.clone()),
    }
  }
  v
}

fn article_list_from_chapter(t: &Chapter) -> Vec<Article> {
  let mut v = Vec::new();
  for contents in t.children.iter() {
    match contents {
      ChapterContents::Section(t) => {
        let mut v2 = article_list_from_section(t);
        v.append(&mut v2)
      }
      ChapterContents::Article(t) => v.push(t.clone()),
    }
  }
  v
}

fn article_list_from_section(t: &Section) -> Vec<Article> {
  let mut v = Vec::new();
  for contents in t.children.iter() {
    match contents {
      SectionContents::Subsection(t) => {
        let mut v2 = article_list_from_subsection(t);
        v.append(&mut v2)
      }
      SectionContents::Division(t) => {
        let mut v2 = article_list_from_division(t);
        v.append(&mut v2)
      }
      SectionContents::Article(t) => v.push(t.clone()),
    }
  }
  v
}

#[allow(clippy::too_many_arguments)]
fn article_list_from_subsection(t: &Subsection) -> Vec<Article> {
  let mut v = Vec::new();
  for contents in t.children.iter() {
    match contents {
      SubsectionContents::Division(t) => {
        let mut v2 = article_list_from_division(t);
        v.append(&mut v2)
      }
      SubsectionContents::Article(t) => v.push(t.clone()),
    }
  }
  v
}

#[allow(clippy::too_many_arguments)]
fn article_list_from_division(t: &Division) -> Vec<Article> {
  let mut v = Vec::new();
  for t in t.children.iter() {
    v.push(t.clone())
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
        _ => String::new(),
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
      _ => String::new(),
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
      _ => String::new(),
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
      _ => String::new(),
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
      _ => String::new(),
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
      _ => String::new(),
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
      _ => String::new(),
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
      _ => String::new(),
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
      _ => String::new(),
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
      _ => String::new(),
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
      SentenceOrColumnOrTable::Table(_) => String::new(),
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
