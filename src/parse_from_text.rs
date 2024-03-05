use crate::result::*;
use crate::*;
use kansuji::Kansuji;
use regex::Regex;

pub(crate) fn parse_body(title: &str, text: &str) -> Result<law::LawBody> {
  let lines = text.lines().map(|s| s.trim());
  let mut contents = Vec::new();
  for line in lines {
    let line_contents = parse_line_contents(line);
    contents.push(line_contents)
  }
  todo!()
}

/// 各行が何に当てはまるのかの種類
#[derive(Clone, Debug)]
enum LineContents {
  /// 見出し：（見出し）
  Caption(String),
  /// 編：第一編　タイトル
  Part(usize, String),
  /// 章：第一章　タイトル
  Chapter(usize, String),
  /// 節：第一節　タイトル
  Section(usize, String),
  /// 款：第一款　タイトル
  Subsection(usize, String),
  /// 目：第一目　タイトル
  Division(usize, String),
  /// 条：第二条 本文
  Article(usize, String),
  /// 項：２　本文
  Paragraph(usize, String),
  /// 号：
  /// - 一　本文
  /// - イ　本文
  /// - （一）　本文
  /// - （１）　本文
  /// - （ｉ）　本文
  /// - （ａ）　本文
  ///
  /// 深さは先頭の文字によって変わり、細かい規則は存在しないため、文字が変わり1になったときに深くなり、1以外になったときに浅くなったと解釈するのが妥当である。
  /// 詳しくは以下を参照
  /// - <https://elaws.e-gov.go.jp/document?lawid=403M50000400049#Mp-At_9>
  /// - <https://note.com/lawyer_alpaca/n/ne09c189e813b#Gcaq9>
  Item(ItemPattern, usize, String),
  /// 附則：附則（昭和三一年四月二日法律第六三号）
  SupplProvision(Option<String>),
  /// その他テキスト
  Text(String),
}

/// 号の数字を表す記号の種類
#[derive(Clone, Copy, Debug)]
enum ItemPattern {
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

fn parse_line_contents(line: &str) -> LineContents {
  use ItemPattern::*;
  use LineContents::*;
  let re_caption = Regex::new("（(?<caption>[^）]+)）$").unwrap();
  let re_article = Regex::new(r"第((?<arabic_num>[0-9]+)|(?<zenkaku_num>[０-９]+)|(?<kansuji>[一二三四五六七八九十百千]+))(?<suffix>(編|章|節|款|目|条))([　\s]*)(?<text>(.+))$").unwrap();
  let re_paragraph = Regex::new(r"(?<num>[０-９]+)([　\s]*)(?<text>(.+))$").unwrap();
  let re_item = Regex::new(r"(（((?<paren_iroha_katakana>[ア-ン]+)|(?<paren_iroha_hiragana>[あ-ん]+)|(?<paren_kansuji>[一二三四五六七八九十百千]+)|(?<paren_zenkaku_num>[０-９]+)|(?<paren_zenkaku_upper>[Ａ-Ｚ]+)|(?<paren_zenkaku_lower>[ａ-ｚ]+))）|((?<no_paren_iroha_katakana>[ア-ン]+)|(?<no_paren_iroha_hiragana>[あ-ん]+)|(?<no_paren_kansuji>[一二三四五六七八九十百千]+)|(?<no_paren_zenkaku_num>[０-９]+)|(?<no_paren_zenkaku_upper>[Ａ-Ｚ]+)|(?<no_paren_zenkaku_lower>[ａ-ｚ]+)))([　\s]*)(?<text>(.+))$").unwrap();
  let re_is_roman = Regex::new(r"[ixvlcIXVLCｉｘｖｌｃＩＸＶＬＣ]+").unwrap();
  let re_suppl_provision =
    Regex::new(r"附([　\s]*)則([　\s]*)(（(?<law_num>.+)）)?[^（）]*").unwrap();
  if let Some(caps) = re_caption.captures(line) {
    Caption(caps["caption"].to_string())
  } else if let Some(caps) = re_article.captures(line) {
    let num = if let Some(arabic_num) = caps.name("arabic_num") {
      arabic_num.as_str().parse::<usize>().unwrap()
    } else if let Some(zenkaku_num) = caps.name("zenkaku_num") {
      parse_zenkaku_num(zenkaku_num.as_str())
    } else if let Some(kansuji) = caps.name("kansuji") {
      let kansuji = Kansuji::try_from(kansuji.as_str()).unwrap();
      let n: u128 = kansuji.into();
      n as usize
    } else {
      unreachable!()
    };
    let text = caps["text"].to_string();
    let suffix = &caps["suffix"];
    if suffix == "編" {
      Part(num, text)
    } else if suffix == "章" {
      Chapter(num, text)
    } else if suffix == "節" {
      Section(num, text)
    } else if suffix == "款" {
      Subsection(num, text)
    } else if suffix == "目" {
      Division(num, text)
    } else if suffix == "条" {
      Article(num, text)
    } else {
      unreachable!()
    }
  } else if let Some(caps) = re_paragraph.captures(line) {
    let num = parse_zenkaku_num(&caps["num"]);
    let text = caps["text"].to_string();
    Paragraph(num, text.to_string())
  } else if let Some(caps) = re_item.captures(line) {
    let (item_pattern, num) = if let Some(s) = caps.name("paren_iroha_katakana") {
      (ParenIrohaKatakana, parse_iroha_katakana(s.as_str()))
    } else if let Some(s) = caps.name("paren_iroha_hiragana") {
      (ParenIrohaHiragana, parse_iroha_hiragana(s.as_str()))
    } else if let Some(s) = caps.name("paren_kansuji") {
      let kansuji = Kansuji::try_from(s.as_str()).unwrap();
      let n: u128 = kansuji.into();
      (ParenKansuji, n as usize)
    } else if let Some(s) = caps.name("paren_zenkaku_num") {
      (ParenZenkakuNum, parse_zenkaku_num(s.as_str()))
    } else if let Some(s) = caps.name("paren_zenkaku_upper") {
      if re_is_roman.is_match(s.as_str()) {
        (ParenZenkakuRomanUpper, parse_roman(s.as_str()))
      } else {
        (ParenZenkakuUpper, parse_zenkaku_alphabet(s.as_str()))
      }
    } else if let Some(s) = caps.name("paren_zenkaku_lower") {
      if re_is_roman.is_match(s.as_str()) {
        (ParenZenkakuRomanLower, parse_roman(s.as_str()))
      } else {
        (ParenZenkakuLower, parse_zenkaku_alphabet(s.as_str()))
      }
    }
    // 括弧なし
    else if let Some(s) = caps.name("no_paren_iroha_katakana") {
      (NoParenIrohaKatakana, parse_iroha_katakana(s.as_str()))
    } else if let Some(s) = caps.name("no_paren_iroha_hiragana") {
      (NoParenIrohaHiragana, parse_iroha_hiragana(s.as_str()))
    } else if let Some(s) = caps.name("no_paren_kansuji") {
      let kansuji = Kansuji::try_from(s.as_str()).unwrap();
      let n: u128 = kansuji.into();
      (NoParenKansuji, n as usize)
    } else if let Some(s) = caps.name("no_paren_zenkaku_num") {
      (NoParenZenkakuNum, parse_zenkaku_num(s.as_str()))
    } else if let Some(s) = caps.name("no_paren_zenkaku_upper") {
      if re_is_roman.is_match(s.as_str()) {
        (NoParenZenkakuRomanUpper, parse_roman(s.as_str()))
      } else {
        (NoParenZenkakuUpper, parse_zenkaku_alphabet(s.as_str()))
      }
    } else if let Some(s) = caps.name("no_paren_zenkaku_lower") {
      if re_is_roman.is_match(s.as_str()) {
        (NoParenZenkakuRomanLower, parse_roman(s.as_str()))
      } else {
        (NoParenZenkakuLower, parse_zenkaku_alphabet(s.as_str()))
      }
    } else {
      unreachable!()
    };
    let text = caps["text"].to_string();
    Item(item_pattern, num, text)
  } else if let Some(caps) = re_suppl_provision.captures(line) {
    let law_num = caps.name("law_num").map(|m| m.as_str().to_string());
    SupplProvision(law_num)
  } else {
    Text(line.to_string())
  }
}

fn parse_zenkaku_num(str: &str) -> usize {
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
    .unwrap()
}

fn parse_iroha_katakana(str: &str) -> usize {
  match str {
    "イ" => 1,
    "ロ" => 2,
    "ハ" => 3,
    "ニ" => 4,
    "ホ" => 5,
    "ヘ" => 6,
    "ト" => 7,
    "チ" => 8,
    "リ" => 9,
    "ヌ" => 10,
    "ル" => 11,
    "ヲ" => 12,
    "ワ" => 13,
    "カ" => 14,
    "ヨ" => 15,
    "タ" => 16,
    "レ" => 17,
    "ソ" => 18,
    "ツ" => 19,
    "ネ" => 20,
    "ナ" => 21,
    "ラ" => 22,
    "ム" => 23,
    "ウ" => 24,
    "ヰ" => 25,
    "ノ" => 26,
    "オ" => 27,
    "ク" => 28,
    "ヤ" => 29,
    "マ" => 30,
    "ケ" => 31,
    "フ" => 32,
    "コ" => 33,
    "エ" => 34,
    "テ" => 35,
    "ア" => 36,
    "サ" => 37,
    "キ" => 38,
    "ユ" => 39,
    "メ" => 40,
    "ミ" => 41,
    "シ" => 42,
    "ヱ" => 43,
    "ヒ" => 44,
    "モ" => 45,
    "セ" => 46,
    "ス" => 47,
    _ => unreachable!(),
  }
}

fn parse_iroha_hiragana(str: &str) -> usize {
  match str {
    "い" => 1,
    "ろ" => 2,
    "は" => 3,
    "に" => 4,
    "ほ" => 5,
    "へ" => 6,
    "と" => 7,
    "ち" => 8,
    "り" => 9,
    "ぬ" => 10,
    "る" => 11,
    "を" => 12,
    "わ" => 13,
    "か" => 14,
    "よ" => 15,
    "た" => 16,
    "れ" => 17,
    "そ" => 18,
    "つ" => 19,
    "ね" => 20,
    "な" => 21,
    "ら" => 22,
    "む" => 23,
    "う" => 24,
    "ゐ" => 25,
    "の" => 26,
    "お" => 27,
    "く" => 28,
    "や" => 29,
    "ま" => 30,
    "け" => 31,
    "ふ" => 32,
    "こ" => 33,
    "え" => 34,
    "て" => 35,
    "あ" => 36,
    "さ" => 37,
    "き" => 38,
    "ゆ" => 39,
    "め" => 40,
    "み" => 41,
    "し" => 42,
    "ゑ" => 43,
    "ひ" => 44,
    "も" => 45,
    "せ" => 46,
    "す" => 47,
    _ => unreachable!(),
  }
}

fn parse_roman(str: &str) -> usize {
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
  .unwrap() as usize
}

fn parse_zenkaku_alphabet(str: &str) -> usize {
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
  s.parse::<usize>().unwrap()
}

#[test]
fn check_parse_zenkaku_alphabet_lower() {
  assert_eq!(parse_zenkaku_alphabet("ｂ"), 2)
}
#[test]
fn check_parse_zenkaku_alphabet_upper() {
  assert_eq!(parse_zenkaku_alphabet("Ｂ"), 2)
}
