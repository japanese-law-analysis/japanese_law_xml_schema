//! 枝番号にも対応した条番号
//!

use crate::result::*;
use kansuji::Kansuji;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ArticleNumber {
  /// 条や部などを含んだ実際の文字列
  pub str: String,
  /// "2_1"などのアンダーバー等でつなぐ形式の文字列
  pub num_str: String,
  /// 一番トップの番号
  pub base_number: usize,
  /// 枝番号
  pub eda_numbers: Vec<usize>,
  /// 範囲を持っていた場合の終わりの箇所 空の場合は範囲ではない
  pub range_end_numbers: Vec<usize>,
}

impl ArticleNumber {
  pub fn zero() -> Self {
    ArticleNumber {
      str: String::new(),
      num_str: String::new(),
      base_number: 0,
      eda_numbers: Vec::new(),
      range_end_numbers: Vec::new(),
    }
  }

  pub fn is_zero(&self) -> bool {
    self.base_number == 0
  }
}

pub fn parse_article_number(str: &str) -> Option<(ArticleNumber, String)> {
  let re_article = Regex::new(r"^(?<str>第((?<arabic_num>[0-9]+)|(?<zenkaku_num>[０-９]+)|(?<kansuji>[一二三四五六七八九十百千]+))(?<suffix>(編|章|節|款|目|条))(?<eda_str>(の([0-9]+|[０-９]+|[一二三四五六七八九十百千]+))*)(から(第((?<arabic_num_2>[0-9]+)|(?<zenkaku_num_2>[０-９]+)|(?<kansuji_2>[一二三四五六七八九十百千]+))(?<suffix_2>(編|章|節|款|目|条))(?<eda_str_2>(の([0-9]+|[０-９]+|[一二三四五六七八九十百千]+))*))まで)?)([　\s]*)(?<text>(.+))$").unwrap();
  let re_paragraph = Regex::new(
    r"^(?<str>(?<num>([０-９]+|[0-9]+))(?<eda_str>(の([０-９]+|[0-9]+))*))([　\s]*)(?<text>(.+))$",
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
    if !range_end_numbers.is_empty() {
      let l = range_end_numbers.clone();
      let mut l = l.iter();
      num_str.push(':');
      num_str.push_str(&l.next().unwrap().to_string());
      for n in l {
        num_str.push('_');
        num_str.push_str(&n.to_string());
      }
    }
    let str = caps["str"].to_string();
    let text = caps["text"].to_string();
    Some((
      ArticleNumber {
        str,
        num_str,
        base_number,
        eda_numbers: eda_numbers.clone(),
        range_end_numbers,
      },
      text,
    ))
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
    let mut num_str = base_number.to_string();
    for n in eda_numbers.iter() {
      num_str.push_str(&format!("_{n}"))
    }
    let str = caps["str"].to_string();
    let text = caps["text"].to_string();
    Some((
      ArticleNumber {
        str,
        num_str,
        base_number,
        eda_numbers: eda_numbers.clone(),
        range_end_numbers: Vec::new(), // TODO
      },
      text,
    ))
  } else {
    None
  }
}

pub fn parse_article_number_from_num_str(suffix: &str, num_str: &str) -> Result<ArticleNumber> {
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
    let base_kansuji = Kansuji::from(base_number);
    let mut str = format!("第{}{suffix}", base_kansuji.to_string());
    if !eda_numbers.is_empty() {
      let s = eda_numbers
        .iter()
        .map(|n| {
          let kansuji = Kansuji::from(n);
          kansuji.to_string()
        })
        .collect::<Vec<String>>()
        .join("の");
      str.push('の');
      str.push_str(&s);
    }

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
      if !range_end_numbers.is_empty() {
        let l = range_end_numbers.clone();
        let mut l = l.iter();
        str.push_str("から");
        str.push_str(&format!("第{}{suffix}", l.next().unwrap()));

        let s = l
          .map(|n| {
            let kansuji = Kansuji::from(n);
            kansuji.to_string()
          })
          .collect::<Vec<String>>()
          .join("の");
        str.push('の');
        str.push_str(&s);
        str.push_str("まで");
      }
    }
    Ok(ArticleNumber {
      str,
      num_str: num_str.to_string(),
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ItemNumber {
  /// 実際の文字列
  pub str: String,
  /// 番号
  pub number: usize,
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

pub fn parse_item_number(str: &str) -> Option<(ItemNumber, String)> {
  use ItemPattern::*;
  let re_item = Regex::new(r"^(?<str>(（((?<paren_iroha_katakana>[ア-ン]+)|(?<paren_iroha_hiragana>[あ-ん]+)|(?<paren_kansuji>[一二三四五六七八九十百千]+)|(?<paren_zenkaku_num>[０-９]+)|(?<paren_zenkaku_upper>[Ａ-Ｚ]+)|(?<paren_zenkaku_lower>[ａ-ｚ]+))）|((?<no_paren_iroha_katakana>[ア-ン]+)|(?<no_paren_iroha_hiragana>[あ-ん]+)|(?<no_paren_kansuji>[一二三四五六七八九十百千]+)|(?<no_paren_zenkaku_num>[０-９]+)|(?<no_paren_zenkaku_upper>[Ａ-Ｚ]+)|(?<no_paren_zenkaku_lower>[ａ-ｚ]+))))([　\s]*)(?<text>(.+))$").unwrap();
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
    let text = caps["text"].to_string();
    let str = caps["str"].to_string();
    Some((
      ItemNumber {
        str,
        number,
        pattern,
      },
      text,
    ))
  } else {
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

#[test]
fn check_parse_zenkaku_alphabet_lower() {
  assert_eq!(parse_zenkaku_alphabet("ｂ"), Some(2))
}
#[test]
fn check_parse_zenkaku_alphabet_upper() {
  assert_eq!(parse_zenkaku_alphabet("Ｂ"), Some(2))
}
