//! 枝番号にも対応した条番号
//!

use std::str::FromStr;

use crate::result::*;
use kansuji::Kansuji;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ArticleNumber {
  /// 一番トップの番号
  pub base_number: usize,
  /// 枝番号
  pub eda_numbers: Vec<usize>,
  /// 範囲を持っていた場合の終わりの箇所 空の場合は範囲ではない
  pub range_end_numbers: Vec<usize>,
}

impl ArticleNumber {
  /// "2_1"などのアンダーバー等でつなぐ形式の文字列を生成する
  pub fn num_str(&self) -> String {
    let mut s = self.base_number.to_string();
    for n in self.eda_numbers.iter() {
      s.push('_');
      s.push_str(&n.to_string());
      if !self.range_end_numbers.is_empty() {
        let l = &self.range_end_numbers;
        let mut l = l.iter();
        s.push(':');
        s.push_str(&l.next().unwrap().to_string());
        for n in l {
          s.push('_');
          s.push_str(&n.to_string());
        }
      }
    }
    s
  }

  /// "1_2"などの文字列から生成する
  pub fn from_num_str(num_str: &str) -> Result<Self> {
    let mut num_s_lst = num_str.split(':');
    let num_str = num_s_lst.next().unwrap();
    let n_lst = num_str.split('_').map(|s| {
      s.parse::<usize>()
        .map_err(|_| Error::ParsingError("usize".to_string(), s.to_string()))
    });
    let mut base_number = None;
    let mut eda_numbers = Vec::new();
    for res in n_lst {
      match res {
        Ok(n) => {
          if base_number.is_none() {
            base_number = Some(n);
          } else {
            eda_numbers.push(n);
          }
        }
        Err(e) => return Err(e),
      }
    }
    if let Some(base_number) = base_number {
      let mut range_end_numbers = Vec::new();
      if let Some(s) = num_s_lst.next() {
        let n_lst = s.split('_').map(|s| {
          s.parse::<usize>()
            .map_err(|_| Error::ParsingError("usize".to_string(), s.to_string()))
        });
        for res in n_lst {
          match res {
            Ok(n) => range_end_numbers.push(n),
            Err(e) => return Err(e),
          }
        }
      }
      Ok(ArticleNumber {
        base_number,
        eda_numbers,
        range_end_numbers,
      })
    } else {
      Err(Error::ParsingError(
        "ArticleNumber".to_string(),
        num_str.to_string(),
      ))
    }
  }

  /// 編や条などを生成する
  fn gen_articles_text(&self, suffix: &str) -> String {
    let base_kansuji = Kansuji::from(self.base_number);
    let mut s = format!("第{}{suffix}", base_kansuji.to_string());
    for n in self.eda_numbers.iter() {
      let n_kansuji = Kansuji::from(n);
      s.push_str(&format!("の{}", n_kansuji.to_string()));
    }
    if !self.range_end_numbers.is_empty() {
      let mut l = self.range_end_numbers.iter();
      let n = l.next().unwrap();
      let n_kansuji = Kansuji::from(n);
      s.push_str(&format!("から第{}{suffix}", n_kansuji.to_string()));
      for n in l {
        let n_kansuji = Kansuji::from(n);
        s.push_str(&format!("の{}", n_kansuji.to_string()));
      }
      s.push_str("まで");
    }
    s
  }

  /// 編の文字列を生成する
  pub fn part_text(&self) -> String {
    self.gen_articles_text("編")
  }

  /// 章の文字列を生成する
  pub fn chapter_text(&self) -> String {
    self.gen_articles_text("章")
  }

  /// 節の文字列を生成する
  pub fn section_text(&self) -> String {
    self.gen_articles_text("節")
  }

  /// 款の文字列を生成する
  pub fn subsection_text(&self) -> String {
    self.gen_articles_text("款")
  }

  /// 目の文字列を生成する
  pub fn division_text(&self) -> String {
    self.gen_articles_text("目")
  }

  /// 条の文字列を生成する
  pub fn article_text(&self) -> String {
    self.gen_articles_text("条")
  }

  /// 段落の文字列を生成する
  pub fn paragraph_text(&self) -> String {
    let mut s = to_zenkaku_num(self.base_number).to_string();
    for n in self.eda_numbers.iter() {
      s.push_str(&format!("の{}", to_zenkaku_num(*n)));
    }
    if !self.range_end_numbers.is_empty() {
      let mut l = self.range_end_numbers.iter();
      let n = l.next().unwrap();
      s.push_str(&format!("から{}", to_zenkaku_num(*n)));
      for n in l {
        s.push_str(&format!("の{}", to_zenkaku_num(*n)));
      }
      s.push_str("まで");
    }
    s
  }

  /// 号の文字列を生成する
  pub fn item_text(&self, pattern: ItemPattern) -> String {
    let mut s = pattern.num_to_str(self.base_number);
    for n in self.eda_numbers.iter() {
      s.push_str(&format!("の{}", pattern.num_to_str_no_paren(*n)));
    }
    if !self.range_end_numbers.is_empty() {
      let mut l = self.range_end_numbers.iter();
      let n = l.next().unwrap();
      s.push_str(&format!("から{}", pattern.num_to_str(*n)));
      for n in l {
        s.push_str(&format!("の{}", pattern.num_to_str_no_paren(*n)));
      }
      s.push_str("まで");
    }
    s
  }

  pub fn from_text() -> Self {
    todo!()
  }

  pub fn zero() -> Self {
    ArticleNumber {
      base_number: 0,
      eda_numbers: Vec::new(),
      range_end_numbers: Vec::new(),
    }
  }

  pub fn is_zero(&self) -> bool {
    self.base_number == 0
  }

  pub fn from_item_number(n: &ItemNumber) -> Self {
    ArticleNumber {
      base_number: n.base_number,
      eda_numbers: n.eda_numbers.clone(),
      range_end_numbers: n.range_end_numbers.clone(),
    }
  }
}

impl FromStr for ArticleNumber {
  type Err = Error;
  fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
    ArticleNumber::from_num_str(s)
  }
}

pub fn parse_article_number(str: &str) -> Option<ArticleNumber> {
  let re_article = Regex::new(r"^第((?<arabic_num>[0-9]+)|(?<zenkaku_num>[０-９]+)|(?<kansuji>[一二三四五六七八九十百千]+))(?<suffix>(編|章|節|款|目|条|項|号))(?<eda_str>(の([0-9]+|[０-９]+|[一二三四五六七八九十百千]+))*)(から(第((?<arabic_num_2>[0-9]+)|(?<zenkaku_num_2>[０-９]+)|(?<kansuji_2>[一二三四五六七八九十百千]+))(?<suffix_2>(編|章|節|款|目|条|項|号))(?<eda_str_2>(の([0-9]+|[０-９]+|[一二三四五六七八九十百千]+))*))まで)?").unwrap();
  let re_paragraph = Regex::new(
    r"^(?<num>([０-９]+|[0-9]+))(?<eda_str>(の([０-９]+|[0-9]+))*)(から(?<num_2>([０-９]+|[0-9]+))(?<eda_str_2>(の([０-９]+|[0-9]+))*)まで)?",
  )
  .unwrap();
  if let Some(caps) = re_article.captures(str) {
    // 編-条
    let base_number = if let Some(arabic_num) = caps.name("arabic_num") {
      arabic_num.as_str().parse::<usize>().unwrap()
    } else if let Some(zenkaku_num) = caps.name("zenkaku_num") {
      parse_zenkaku_num(zenkaku_num.as_str()).unwrap()
    } else if let Some(kansuji) = caps.name("kansuji") {
      let kansuji = Kansuji::try_from(kansuji.as_str()).unwrap();
      let n: u128 = kansuji.into();
      n as usize
    } else {
      unreachable!()
    };
    let eda_numbers = &caps["eda_str"]
      .split('の')
      .filter(|s| !s.is_empty())
      .map(|s| {
        if let Ok(n) = s.parse::<usize>() {
          n
        } else if let Some(n) = parse_zenkaku_num(s) {
          n
        } else {
          let kansuji = Kansuji::try_from(s).unwrap();
          let n: u128 = kansuji.into();
          n as usize
        }
      })
      .collect::<Vec<usize>>();
    let mut num_str = base_number.to_string();
    for n in eda_numbers.iter() {
      num_str.push_str(&format!("_{n}"))
    }
    let mut range_end_numbers = Vec::new();
    if let Some(arabic_num) = caps.name("arabic_num_2") {
      range_end_numbers.push(arabic_num.as_str().parse::<usize>().unwrap())
    } else if let Some(zenkaku_num) = caps.name("zenkaku_num_2") {
      range_end_numbers.push(parse_zenkaku_num(zenkaku_num.as_str()).unwrap())
    } else if let Some(kansuji) = caps.name("kansuji_2") {
      let kansuji = Kansuji::try_from(kansuji.as_str()).unwrap();
      let n: u128 = kansuji.into();
      range_end_numbers.push(n as usize)
    };
    if let Some(s) = &caps.name("eda_str_2") {
      let eda_numbers_2 = s
        .as_str()
        .split('の')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
      for s in eda_numbers_2.iter() {
        if let Ok(n) = s.parse::<usize>() {
          range_end_numbers.push(n);
        } else if let Some(n) = parse_zenkaku_num(s) {
          range_end_numbers.push(n);
        } else {
          let kansuji = Kansuji::try_from(*s).unwrap();
          let n: u128 = kansuji.into();
          range_end_numbers.push(n as usize);
        }
      }
    }
    Some(ArticleNumber {
      base_number,
      eda_numbers: eda_numbers.clone(),
      range_end_numbers,
    })
  } else if let Some(caps) = re_paragraph.captures(str) {
    // 段落
    let base_number = if let Ok(n) = &caps["num"].parse::<usize>() {
      *n
    } else {
      parse_zenkaku_num(&caps["num"]).unwrap()
    };
    let eda_numbers = &caps["eda_str"]
      .split('の')
      .filter(|s| !s.is_empty())
      .map(|s| {
        if let Ok(n) = s.parse::<usize>() {
          n
        } else {
          parse_zenkaku_num(s).unwrap()
        }
      })
      .collect::<Vec<usize>>();
    let mut range_end_numbers = Vec::new();
    if let Some(m) = caps.name("num_2") {
      if let Ok(n) = m.as_str().parse::<usize>() {
        range_end_numbers.push(n)
      } else {
        range_end_numbers.push(parse_zenkaku_num(m.as_str()).unwrap());
      }
    }
    Some(ArticleNumber {
      base_number,
      eda_numbers: eda_numbers.clone(),
      range_end_numbers,
    })
  } else {
    None
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ItemNumber {
  /// 実際の文字列
  pub str: String,
  /// 番号
  pub base_number: usize,
  pub eda_numbers: Vec<usize>,
  pub range_end_numbers: Vec<usize>,
  /// 番号のパターン
  pub pattern: ItemPattern,
}

/// 号の数字を表す記号の種類
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum ItemPattern {
  /// 括弧なし漢数字
  NoParenKansuji,
  /// 括弧なしイロハ
  NoParenIrohaKatakana,
  /// 括弧なしいろは
  NoParenIrohaHiragana,
  /// 括弧なし全角数字
  NoParenZenkakuNum,
  /// 括弧なしローマ数字大文字
  NoParenZenkakuRomanUpper,
  /// 括弧なしローマ数字小文字
  NoParenZenkakuRomanLower,
  /// 括弧なし大文字
  NoParenZenkakuUpper,
  /// 括弧なし小文字
  NoParenZenkakuLower,
  /// 括弧あり漢数字
  ParenKansuji,
  /// 括弧ありイロハ
  ParenIrohaKatakana,
  /// 括弧ありいろは
  ParenIrohaHiragana,
  /// 括弧あり全角数字
  ParenZenkakuNum,
  /// 括弧ありローマ数字大文字
  ParenZenkakuRomanUpper,
  /// 括弧ありローマ数字小文字
  ParenZenkakuRomanLower,
  /// 括弧あり大文字
  ParenZenkakuUpper,
  /// 括弧あり小文字
  ParenZenkakuLower,
}

impl ItemPattern {
  fn num_to_str(&self, n: usize) -> String {
    match self {
      ItemPattern::NoParenKansuji => {
        let kansuji = Kansuji::from(n);
        kansuji.to_string()
      }
      ItemPattern::NoParenIrohaKatakana => to_iroha_katakana(n).unwrap(),
      ItemPattern::NoParenIrohaHiragana => to_iroha_hiragana(n).unwrap(),
      ItemPattern::NoParenZenkakuNum => to_zenkaku_num(n),
      ItemPattern::NoParenZenkakuRomanUpper => to_zenkaku_roman_upper(n).unwrap(),
      ItemPattern::NoParenZenkakuRomanLower => to_zenkaku_roman_lower(n).unwrap(),
      ItemPattern::NoParenZenkakuUpper => to_zenkaku_alphabet_upper(n),
      ItemPattern::NoParenZenkakuLower => to_zenkaku_alphabet_lower(n),
      ItemPattern::ParenKansuji => {
        let kansuji = Kansuji::from(n);
        format!("（{}）", kansuji.to_string())
      }
      ItemPattern::ParenIrohaKatakana => {
        format!("（{}）", to_iroha_katakana(n).unwrap())
      }
      ItemPattern::ParenIrohaHiragana => {
        format!("（{}）", to_iroha_hiragana(n).unwrap())
      }
      ItemPattern::ParenZenkakuNum => format!("（{}）", to_zenkaku_num(n)),
      ItemPattern::ParenZenkakuRomanUpper => {
        format!("（{}）", to_zenkaku_roman_lower(n).unwrap())
      }
      ItemPattern::ParenZenkakuRomanLower => {
        format!("（{}）", to_zenkaku_roman_upper(n).unwrap())
      }
      ItemPattern::ParenZenkakuUpper => {
        format!("（{}）", to_zenkaku_alphabet_upper(n))
      }
      ItemPattern::ParenZenkakuLower => {
        format!("（{}）", to_zenkaku_alphabet_lower(n))
      }
    }
  }

  fn num_to_str_no_paren(&self, n: usize) -> String {
    match self {
      ItemPattern::NoParenKansuji => {
        let kansuji = Kansuji::from(n);
        kansuji.to_string()
      }
      ItemPattern::NoParenIrohaKatakana => to_iroha_katakana(n).unwrap(),
      ItemPattern::NoParenIrohaHiragana => to_iroha_hiragana(n).unwrap(),
      ItemPattern::NoParenZenkakuNum => to_zenkaku_num(n),
      ItemPattern::NoParenZenkakuRomanUpper => to_zenkaku_roman_upper(n).unwrap(),
      ItemPattern::NoParenZenkakuRomanLower => to_zenkaku_roman_lower(n).unwrap(),
      ItemPattern::NoParenZenkakuUpper => to_zenkaku_alphabet_upper(n),
      ItemPattern::NoParenZenkakuLower => to_zenkaku_alphabet_lower(n),
      ItemPattern::ParenKansuji => {
        let kansuji = Kansuji::from(n);
        kansuji.to_string()
      }
      ItemPattern::ParenIrohaKatakana => to_iroha_katakana(n).unwrap(),
      ItemPattern::ParenIrohaHiragana => to_iroha_hiragana(n).unwrap(),
      ItemPattern::ParenZenkakuNum => to_zenkaku_num(n),
      ItemPattern::ParenZenkakuRomanUpper => to_zenkaku_roman_upper(n).unwrap(),
      ItemPattern::ParenZenkakuRomanLower => to_zenkaku_roman_lower(n).unwrap(),
      ItemPattern::ParenZenkakuUpper => to_zenkaku_alphabet_upper(n),
      ItemPattern::ParenZenkakuLower => to_zenkaku_alphabet_lower(n),
    }
  }
}

fn parse_item_number_base(str: &str) -> Option<(ItemPattern, usize)> {
  use ItemPattern::*;
  let re_item = Regex::new(r"^(?<str>(（((?<paren_iroha_katakana>[ア-ン]+)|(?<paren_iroha_hiragana>[あ-ん]+)|(?<paren_kansuji>[一二三四五六七八九十百千]+)|(?<paren_zenkaku_num>[０-９]+)|(?<paren_zenkaku_upper>[Ａ-Ｚ]+)|(?<paren_zenkaku_lower>[ａ-ｚ]+))）|((?<no_paren_iroha_katakana>[ア-ン]+)|(?<no_paren_iroha_hiragana>[あ-ん]+)|(?<no_paren_kansuji>[一二三四五六七八九十百千]+)|(?<no_paren_zenkaku_num>[０-９]+)|(?<no_paren_zenkaku_upper>[Ａ-Ｚ]+)|(?<no_paren_zenkaku_lower>[ａ-ｚ]+))))").unwrap();
  let re_is_roman = Regex::new(r"[ixvlcIXVLCｉｘｖｌｃＩＸＶＬＣ]+").unwrap();
  if let Some(caps) = re_item.captures(str) {
    let (pattern, number) = if let Some(s) = caps.name("paren_iroha_katakana") {
      (
        ParenIrohaKatakana,
        parse_iroha_katakana(s.as_str()).unwrap(),
      )
    } else if let Some(s) = caps.name("paren_iroha_hiragana") {
      (
        ParenIrohaHiragana,
        parse_iroha_hiragana(s.as_str()).unwrap(),
      )
    } else if let Some(s) = caps.name("paren_kansuji") {
      let kansuji = Kansuji::try_from(s.as_str()).unwrap();
      let n: u128 = kansuji.into();
      (ParenKansuji, n as usize)
    } else if let Some(s) = caps.name("paren_zenkaku_num") {
      (ParenZenkakuNum, parse_zenkaku_num(s.as_str()).unwrap())
    } else if let Some(s) = caps.name("paren_zenkaku_upper") {
      if re_is_roman.is_match(s.as_str()) {
        (ParenZenkakuRomanUpper, parse_roman(s.as_str()).unwrap())
      } else {
        (
          ParenZenkakuUpper,
          parse_zenkaku_alphabet(s.as_str()).unwrap(),
        )
      }
    } else if let Some(s) = caps.name("paren_zenkaku_lower") {
      if re_is_roman.is_match(s.as_str()) {
        (ParenZenkakuRomanLower, parse_roman(s.as_str()).unwrap())
      } else {
        (
          ParenZenkakuLower,
          parse_zenkaku_alphabet(s.as_str()).unwrap(),
        )
      }
    }
    // 括弧なし
    else if let Some(s) = caps.name("no_paren_iroha_katakana") {
      (
        NoParenIrohaKatakana,
        parse_iroha_katakana(s.as_str()).unwrap(),
      )
    } else if let Some(s) = caps.name("no_paren_iroha_hiragana") {
      (
        NoParenIrohaHiragana,
        parse_iroha_hiragana(s.as_str()).unwrap(),
      )
    } else if let Some(s) = caps.name("no_paren_kansuji") {
      let kansuji = Kansuji::try_from(s.as_str()).unwrap();
      let n: u128 = kansuji.into();
      (NoParenKansuji, n as usize)
    } else if let Some(s) = caps.name("no_paren_zenkaku_num") {
      (NoParenZenkakuNum, parse_zenkaku_num(s.as_str()).unwrap())
    } else if let Some(s) = caps.name("no_paren_zenkaku_upper") {
      if re_is_roman.is_match(s.as_str()) {
        (NoParenZenkakuRomanUpper, parse_roman(s.as_str()).unwrap())
      } else {
        (
          NoParenZenkakuUpper,
          parse_zenkaku_alphabet(s.as_str()).unwrap(),
        )
      }
    } else if let Some(s) = caps.name("no_paren_zenkaku_lower") {
      if re_is_roman.is_match(s.as_str()) {
        (NoParenZenkakuRomanLower, parse_roman(s.as_str()).unwrap())
      } else {
        (
          NoParenZenkakuLower,
          parse_zenkaku_alphabet(s.as_str()).unwrap(),
        )
      }
    } else {
      unreachable!()
    };
    Some((pattern, number))
  } else {
    None
  }
}

pub fn split_number(str: &str) -> Option<(String, String)> {
  let re = Regex::new(r"^(?<s>[^\s　]+)[\s　]*(?<text>.+)$").unwrap();
  re.captures(str).map(|caps| {
    let s = &caps["s"];
    let text = &caps["text"];
    (s.to_string(), text.to_string())
  })
}

#[test]
fn check_parse_article_number() {
  let cases = vec![
    (
      "第一条",
      ArticleNumber {
        base_number: 1,
        eda_numbers: Vec::new(),
        range_end_numbers: Vec::new(),
      },
    ),
    (
      "第一条の二",
      ArticleNumber {
        base_number: 1,
        eda_numbers: vec![2],
        range_end_numbers: Vec::new(),
      },
    ),
    (
      "第一項の二の三",
      ArticleNumber {
        base_number: 1,
        eda_numbers: vec![2, 3],
        range_end_numbers: Vec::new(),
      },
    ),
  ];
  for (s, a) in cases.iter() {
    let r = parse_article_number(s);
    assert_eq!(r, Some(a.clone()))
  }
}

pub fn parse_item_number(str: &str) -> Option<ItemNumber> {
  let re_item = Regex::new(r"^(?<base_number>(（(([ア-ン]+)|([あ-ん]+)|([一二三四五六七八九十百千]+)|([０-９]+)|([Ａ-Ｚ]+)|([ａ-ｚ]+))）|(([ア-ン]+)|([あ-ん]+)|([一二三四五六七八九十百千]+)|([０-９]+)|([Ａ-Ｚ]+)|([ａ-ｚ]+))))(?<eda_numbers>(の(（(([ア-ン]+)|([あ-ん]+)|([一二三四五六七八九十百千]+)|([０-９]+)|([Ａ-Ｚ]+)|([ａ-ｚ]+))）|(([ア-ン]+)|([あ-ん]+)|([一二三四五六七八九十百千]+)|([０-９]+)|([Ａ-Ｚ]+)|([ａ-ｚ]+))))+)?(から(?<base_number2>(（(([ア-ン]+)|([あ-ん]+)|([一二三四五六七八九十百千]+)|([０-９]+)|([Ａ-Ｚ]+)|([ａ-ｚ]+))）|(([ア-ン]+)|([あ-ん]+)|([一二三四五六七八九十百千]+)|([０-９]+)|([Ａ-Ｚ]+)|([ａ-ｚ]+))))(?<eda_numbers2>(の(（(([ア-ン]+)|([あ-ん]+)|([一二三四五六七八九十百千]+)|([０-９]+)|([Ａ-Ｚ]+)|([ａ-ｚ]+))）|(([ア-ン]+)|([あ-ん]+)|([一二三四五六七八九十百千]+)|([０-９]+)|([Ａ-Ｚ]+)|([ａ-ｚ]+))))+)?まで)?$").unwrap();
  if let Some(caps) = re_item.captures(str.trim()) {
    println!("ok!!!");
    if let Some((pat, base_number)) = parse_item_number_base(&caps["base_number"]) {
      let eda_numbers = if let Some(m) = &caps.name("eda_numbers") {
        m.as_str()
          .split('の')
          .filter(|s| !s.is_empty())
          .map(parse_item_number_base)
          .filter(|v| v.is_some())
          .map(|v| v.unwrap().1)
          .collect::<Vec<_>>()
      } else {
        Vec::new()
      };
      let mut range_end_numbers = Vec::new();
      if let Some(m) = &caps.name("base_number_2") {
        if let Some((_, base_number2)) = parse_item_number_base(m.as_str()) {
          range_end_numbers.push(base_number2);
        }
      }
      if let Some(m) = &caps.name("eda_number_2") {
        let eda_numbers2 = &m
          .as_str()
          .split('の')
          .filter(|s| !s.is_empty())
          .map(parse_item_number_base)
          .filter(|v| v.is_some())
          .map(|v| v.unwrap().1)
          .collect::<Vec<_>>();
        for v in eda_numbers2.iter() {
          range_end_numbers.push(*v)
        }
      }
      Some(ItemNumber {
        str: str.to_string(),
        base_number,
        eda_numbers,
        range_end_numbers,
        pattern: pat,
      })
    } else {
      None
    }
  } else {
    println!("err!!");
    None
  }
}

fn parse_zenkaku_num(str: &str) -> Option<usize> {
  str
    .replace('０', "0")
    .replace('１', "1")
    .replace('２', "2")
    .replace('３', "3")
    .replace('４', "4")
    .replace('５', "5")
    .replace('６', "6")
    .replace('７', "7")
    .replace('８', "8")
    .replace('９', "9")
    .parse::<usize>()
    .ok()
}

fn to_zenkaku_num(n: usize) -> String {
  n.to_string()
    .replace('0', "０")
    .replace('1', "１")
    .replace('2', "２")
    .replace('3', "３")
    .replace('4', "４")
    .replace('5', "５")
    .replace('6', "６")
    .replace('7', "７")
    .replace('8', "８")
    .replace('9', "９")
}

fn parse_iroha_katakana(str: &str) -> Option<usize> {
  match str {
    "イ" => Some(1),
    "ロ" => Some(2),
    "ハ" => Some(3),
    "ニ" => Some(4),
    "ホ" => Some(5),
    "ヘ" => Some(6),
    "ト" => Some(7),
    "チ" => Some(8),
    "リ" => Some(9),
    "ヌ" => Some(10),
    "ル" => Some(11),
    "ヲ" => Some(12),
    "ワ" => Some(13),
    "カ" => Some(14),
    "ヨ" => Some(15),
    "タ" => Some(16),
    "レ" => Some(17),
    "ソ" => Some(18),
    "ツ" => Some(19),
    "ネ" => Some(20),
    "ナ" => Some(21),
    "ラ" => Some(22),
    "ム" => Some(23),
    "ウ" => Some(24),
    "ヰ" => Some(25),
    "ノ" => Some(26),
    "オ" => Some(27),
    "ク" => Some(28),
    "ヤ" => Some(29),
    "マ" => Some(30),
    "ケ" => Some(31),
    "フ" => Some(32),
    "コ" => Some(33),
    "エ" => Some(34),
    "テ" => Some(35),
    "ア" => Some(36),
    "サ" => Some(37),
    "キ" => Some(38),
    "ユ" => Some(39),
    "メ" => Some(40),
    "ミ" => Some(41),
    "シ" => Some(42),
    "ヱ" => Some(43),
    "ヒ" => Some(44),
    "モ" => Some(45),
    "セ" => Some(46),
    "ス" => Some(47),
    _ => None,
  }
}

fn to_iroha_katakana(n: usize) -> Option<String> {
  match n {
    1 => Some("イ".to_string()),
    2 => Some("ロ".to_string()),
    3 => Some("ハ".to_string()),
    4 => Some("ニ".to_string()),
    5 => Some("ホ".to_string()),
    6 => Some("ヘ".to_string()),
    7 => Some("ト".to_string()),
    8 => Some("チ".to_string()),
    9 => Some("リ".to_string()),
    10 => Some("ヌ".to_string()),
    11 => Some("ル".to_string()),
    12 => Some("ヲ".to_string()),
    13 => Some("ワ".to_string()),
    14 => Some("カ".to_string()),
    15 => Some("ヨ".to_string()),
    16 => Some("タ".to_string()),
    17 => Some("レ".to_string()),
    18 => Some("ソ".to_string()),
    19 => Some("ツ".to_string()),
    20 => Some("ネ".to_string()),
    21 => Some("ナ".to_string()),
    22 => Some("ラ".to_string()),
    23 => Some("ム".to_string()),
    24 => Some("ウ".to_string()),
    25 => Some("ヰ".to_string()),
    26 => Some("ノ".to_string()),
    27 => Some("オ".to_string()),
    28 => Some("ク".to_string()),
    29 => Some("ヤ".to_string()),
    30 => Some("マ".to_string()),
    31 => Some("ケ".to_string()),
    32 => Some("フ".to_string()),
    33 => Some("コ".to_string()),
    34 => Some("エ".to_string()),
    35 => Some("テ".to_string()),
    36 => Some("ア".to_string()),
    37 => Some("サ".to_string()),
    38 => Some("キ".to_string()),
    39 => Some("ユ".to_string()),
    40 => Some("メ".to_string()),
    41 => Some("ミ".to_string()),
    42 => Some("シ".to_string()),
    43 => Some("ヱ".to_string()),
    44 => Some("ヒ".to_string()),
    45 => Some("モ".to_string()),
    46 => Some("セ".to_string()),
    47 => Some("ス".to_string()),
    _ => None,
  }
}

fn parse_iroha_hiragana(str: &str) -> Option<usize> {
  match str {
    "い" => Some(1),
    "ろ" => Some(2),
    "は" => Some(3),
    "に" => Some(4),
    "ほ" => Some(5),
    "へ" => Some(6),
    "と" => Some(7),
    "ち" => Some(8),
    "り" => Some(9),
    "ぬ" => Some(10),
    "る" => Some(11),
    "を" => Some(12),
    "わ" => Some(13),
    "か" => Some(14),
    "よ" => Some(15),
    "た" => Some(16),
    "れ" => Some(17),
    "そ" => Some(18),
    "つ" => Some(19),
    "ね" => Some(20),
    "な" => Some(21),
    "ら" => Some(22),
    "む" => Some(23),
    "う" => Some(24),
    "ゐ" => Some(25),
    "の" => Some(26),
    "お" => Some(27),
    "く" => Some(28),
    "や" => Some(29),
    "ま" => Some(30),
    "け" => Some(31),
    "ふ" => Some(32),
    "こ" => Some(33),
    "え" => Some(34),
    "て" => Some(35),
    "あ" => Some(36),
    "さ" => Some(37),
    "き" => Some(38),
    "ゆ" => Some(39),
    "め" => Some(40),
    "み" => Some(41),
    "し" => Some(42),
    "ゑ" => Some(43),
    "ひ" => Some(44),
    "も" => Some(45),
    "せ" => Some(46),
    "す" => Some(47),
    _ => None,
  }
}

fn to_iroha_hiragana(n: usize) -> Option<String> {
  match n {
    1 => Some("い".to_string()),
    2 => Some("ろ".to_string()),
    3 => Some("は".to_string()),
    4 => Some("に".to_string()),
    5 => Some("ほ".to_string()),
    6 => Some("へ".to_string()),
    7 => Some("と".to_string()),
    8 => Some("ち".to_string()),
    9 => Some("り".to_string()),
    10 => Some("ぬ".to_string()),
    11 => Some("る".to_string()),
    12 => Some("を".to_string()),
    13 => Some("わ".to_string()),
    14 => Some("か".to_string()),
    15 => Some("よ".to_string()),
    16 => Some("た".to_string()),
    17 => Some("れ".to_string()),
    18 => Some("そ".to_string()),
    19 => Some("つ".to_string()),
    20 => Some("ね".to_string()),
    21 => Some("な".to_string()),
    22 => Some("ら".to_string()),
    23 => Some("む".to_string()),
    24 => Some("う".to_string()),
    25 => Some("ゐ".to_string()),
    26 => Some("の".to_string()),
    27 => Some("お".to_string()),
    28 => Some("く".to_string()),
    29 => Some("や".to_string()),
    30 => Some("ま".to_string()),
    31 => Some("け".to_string()),
    32 => Some("ふ".to_string()),
    33 => Some("こ".to_string()),
    34 => Some("え".to_string()),
    35 => Some("て".to_string()),
    36 => Some("あ".to_string()),
    37 => Some("さ".to_string()),
    38 => Some("き".to_string()),
    39 => Some("ゆ".to_string()),
    40 => Some("め".to_string()),
    41 => Some("み".to_string()),
    42 => Some("し".to_string()),
    43 => Some("ゑ".to_string()),
    44 => Some("ひ".to_string()),
    45 => Some("も".to_string()),
    46 => Some("せ".to_string()),
    47 => Some("す".to_string()),
    _ => None,
  }
}

fn parse_roman(str: &str) -> Option<usize> {
  roman::from(
    &str
      .replace('i', "I")
      .replace('x', "X")
      .replace('l', "L")
      .replace('c', "C")
      .replace('ｉ', "I")
      .replace('ｘ', "X")
      .replace('ｌ', "L")
      .replace('ｃ', "C")
      .replace('Ｉ', "I")
      .replace('Ｘ', "X")
      .replace('Ｌ', "L")
      .replace('Ｃ', "C"),
  )
  .map(|n| n as usize)
}

fn to_zenkaku_roman_upper(n: usize) -> Option<String> {
  roman::to(n as i32).map(|s| {
    s.replace('I', "Ｉ")
      .replace('X', "Ｘ")
      .replace('L', "Ｌ")
      .replace('C', "Ｃ")
  })
}

fn to_zenkaku_roman_lower(n: usize) -> Option<String> {
  roman::to(n as i32).map(|s| {
    s.replace('I', "ｉ")
      .replace('X', "ｘ")
      .replace('L', "ｌ")
      .replace('C', "ｃ")
  })
}

fn parse_zenkaku_alphabet(str: &str) -> Option<usize> {
  let s = str
    .chars()
    .map(|c| {
      let v = c as u32;
      let n =
    // 大文字
    if v < 0xFF41 {
      v - 0xFF20
    } else {
      v - 0xFF40
    };
      n.to_string()
    })
    .collect::<String>();
  s.parse::<usize>().ok()
}

fn to_zenkaku_alphabet_upper(n: usize) -> String {
  n.to_string()
    .chars()
    .map(|s| {
      let ns = s.to_string().parse::<u32>().unwrap();
      char::from_digit(ns + 0xFF20, 10).unwrap()
    })
    .collect::<String>()
}

fn to_zenkaku_alphabet_lower(n: usize) -> String {
  n.to_string()
    .chars()
    .map(|s| {
      let ns = s.to_string().parse::<u32>().unwrap();
      char::from_digit(ns + 0xFF40, 10).unwrap()
    })
    .collect::<String>()
}

#[test]
fn check_parse_zenkaku_alphabet_lower() {
  assert_eq!(parse_zenkaku_alphabet("ｂ"), Some(2))
}
#[test]
fn check_parse_zenkaku_alphabet_upper() {
  assert_eq!(parse_zenkaku_alphabet("Ｂ"), Some(2))
}
