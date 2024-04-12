//! 枝番号にも対応した条番号
//!

use kansuji::Kansuji;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ArticleNumber {
  /// 条や部などを含んだ実際の文字列
  pub str: String,
  /// "2_1"などのアンダーバー等でつなぐ形式の文字列
  pub num_str: String,
  /// 一番トップの番号
  pub base_number: usize,
  /// 枝番号
  pub eda_numbers: Vec<usize>,
}

pub fn parse_article_number(str: &str) -> Option<(ArticleNumber, String)> {
  let re_article = Regex::new(r"^(?<str>第((?<arabic_num>[0-9]+)|(?<zenkaku_num>[０-９]+)|(?<kansuji>[一二三四五六七八九十百千]+))(?<suffix>(編|章|節|款|目|条))(?<eda_str>(の([0-9]+|[０-９]+|[一二三四五六七八九十百千]+))*))([　\s]*)(?<text>(.+))$").unwrap();
  let re_paragraph = Regex::new(
    r"^(?<str>(?<num>([０-９]+|[0-9]+))(?<eda_str>(の([０-９]+|[0-9]+))*))([　\s]*)(?<text>(.+))$",
  )
  .unwrap();
  if let Some(caps) = re_article.captures(str) {
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
    let str = caps["str"].to_string();
    let text = caps["text"].to_string();
    Some((
      ArticleNumber {
        str,
        num_str,
        base_number,
        eda_numbers: eda_numbers.clone(),
      },
      text,
    ))
  } else if let Some(caps) = re_paragraph.captures(str) {
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
      },
      text,
    ))
  } else {
    None
  }
}

/// 号の数字を表す記号の種類
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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