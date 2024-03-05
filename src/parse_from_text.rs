use crate::result::*;
use crate::*;
use regex::Regex;

pub(crate) fn parse_body(title: &str, text: &str) -> Result<law::LawBody> {
  let lines = text.lines().map(|s| s.trim());
  for line in lines {}
  todo!()
}

/// 各行が何に当てはまるのかの種類
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
}

/// 号の数字を表す記号の種類
enum ItemPattern {
  /// 括弧なし漢数字
  NoParenKansuji,
  /// 括弧なしイロハ
  NoParenIroha,
  /// 括弧なし全角数字
  NoParenZenkakuNum,
  /// 括弧なしローマ数字
  NoParenZenkakuRoman,
  /// 括弧なし大文字
  NoParenZenkakuUpper,
  /// 括弧なし小文字
  NoParenZenkakuLower,
  /// 括弧あり漢数字
  ParenKansuji,
  /// 括弧ありイロハ
  ParenIroha,
  /// 括弧あり全角数字
  ParenZenkakuNum,
  /// 括弧ありローマ数字
  ParenZenkakuRoman,
  /// 括弧あり大文字
  ParenZenkakuUpper,
  /// 括弧あり小文字
  ParenZenkakuLower,
}

fn get_articles(line: &str) -> LineContents {
  let re_caption = Regex::new("（(?<caption>[^）]+)）$").unwrap();
  let re_article = Regex::new(r"第((?<arabic_num>[0-9]+)|(?<zenkaku_num>[０-９]+)|(?<kansuji>[一二三四五六七八九十百千]+))(?<suffix>(編|章|節|款|目|条))([　\s]+)(?<text>(.+))$");
  let re_paragraph = Regex::new(r"(?<num>[０-９]+)([　\s]+)(?<text>(.+))$").unwrap();
  let re_item = Regex::new(r"([　\s]+)(?<text>(.+))$").unwrap();
  todo!()
}
