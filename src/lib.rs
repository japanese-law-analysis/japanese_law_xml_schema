//! [法令標準XMLスキーマ](https://elaws.e-gov.go.jp/file/XMLSchemaForJapaneseLaw_v3.xsd)で定義された規格に基づいてXMLとの相互変換を行う
//!
//!

use serde::{Deserialize, Serialize};

pub mod article;
pub mod line;
pub mod list;
pub mod parser;
pub mod result;
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

/// 目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOC {
  /// 目次タイトル（概ね「目次」）
  toc_label: Option<text::Text>,
  /// 前文タイトル（概ね「前文」）
  toc_preamble_label: Option<text::Text>,
  /// 本文の目次
  toc_main_contents: Vec<TOCMainContents>,
  /// 附則の目次
  toc_suppl_provision: Option<TOCSupplProvision>,
  /// 付録表のタイトル
  toc_appdx_table_label: Vec<text::Text>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum TOCMainContents {
  /// 編の目次
  TOCPart(TOCPart),
  /// 章の目次
  TOCChapter(TOCChapter),
  /// 節の目次
  TOCSection(TOCSection),
  /// 条の目次
  TOCArticle(TOCArticle),
}

/// 編の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCPart {
  /// 編のタイトル
  part_title: text::Text,
  /// 条の範囲
  article_range: Option<text::Text>,
  /// 編の番号
  num: String,
  /// 削除された編かどうか
  delete: bool,
  /// 子要素
  children: Vec<TOCChapter>,
}

/// 章の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCChapter {
  /// 章のタイトル
  chapter_title: text::Text,
  /// 条の範囲
  article_range: Option<text::Text>,
  /// 章の番号
  num: String,
  /// 削除された章かどうか
  delete: bool,
  /// 子要素
  children: Vec<TOCSection>,
}

/// 節の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCSection {
  /// 節のタイトル
  section_title: text::Text,
  /// 条の範囲
  article_range: Option<text::Text>,
  /// 節の番号
  num: String,
  /// 削除された節かどうか
  delete: bool,
  /// 子要素
  children: Vec<TOCSectionContents>,
}

/// 節目次の子要素
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum TOCSectionContents {
  /// 款
  TOCSubsection(TOCSubsection),
  /// 目
  TOCDivision(TOCDivision),
}

/// 款の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCSubsection {
  /// 款のタイトル
  subsection_title: text::Text,
  /// 条の範囲
  article_range: Option<text::Text>,
  /// 款の番号
  num: String,
  /// 削除された款かどうか
  delete: bool,
  /// 子要素
  children: Vec<TOCDivision>,
}

/// 目の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCDivision {
  /// 目のタイトル
  division_title: text::Text,
  /// 条の範囲
  article_range: Option<text::Text>,
  /// 目の番号
  num: String,
  /// 削除された目かどうか
  delete: bool,
}

/// 条の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCArticle {
  /// 条のタイトル
  article_title: text::Text,
  /// 見出し
  article_caption: text::Text,
  /// 条番号
  num: String,
  /// 削除された条かどうか
  delete: bool,
}

/// 附則の目次
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TOCSupplProvision {
  /// 見出し（概ね「附則」）
  suppl_provision_label: text::Text,
  /// 範囲
  article_range: Option<text::Text>,
  /// 子要素
  children: Vec<TOCSupplProvisionContents>,
}

/// 附則の目次の中身
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum TOCSupplProvisionContents {
  /// 条
  TOCArticle(TOCArticle),
  /// 章
  TOCChapter(TOCChapter),
}

/// 前文
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Preamble {
  children: Vec<Paragraph>,
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
  Paragraph(Paragraph),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Paragraph {
  caption: ParagraphCaption,
  paragraph_num: text::Text,
  amend_provision: Vec<AmendProvision>,
  class: Vec<Class>,
  sentence: Vec<Sentence>,
  struct_list: Vec<Struct>,
  children: Vec<Item>,
  num: usize,
  old_style: bool,
  old_num: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ParagraphCaption {
  text: text::Text,
  common_caption: Option<bool>,
}

/// 改正
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AmendProvision {
  sentence: Sentence,
  new_provision: Vec<NewProvision>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum NewProvision {
  LawTitle(LawTitle),
  Preamble(Preamble),
  TOC(TOC),
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
  List(list::List),
  Sentence(Sentence),
  AmendProvision(AmendProvision),
  AppdxTable(AppdxTable),
  AppdxNote(AppdxNote),
  AppdxStyle(AppdxStyle),
  Appdx(Appdx),
  AppdxFig(AppdxFig),
  AppdxFormat(AppdxFormat),
  SupplProvisionAppdxStyle(SupplProvisionAppdxStyle),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Struct {
  TableStruct(TableStruct),
  FigStruct(FigStruct),
  StyleStruct(StyleStruct),
  List(list::List),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Class {
  class_title: Option<text::Text>,
  class_sentence: SentenceOrColumnOrTable,
  children: Vec<Item>,
  num: String,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SentenceOrColumnOrTable {
  Sentence(Vec<Sentence>),
  Column(Vec<Column>),
  Table(Table),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Item {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem1>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem1 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem2>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem2 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem3>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem3 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem4>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem4 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem5>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem5 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem6>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem6 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem7>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem7 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem8>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem8 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem9>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem9 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  children: Vec<Subitem10>,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Subitem10 {
  title: Option<text::Text>,
  sentence: SentenceOrColumnOrTable,
  struct_list: Vec<Struct>,
  num: String,
  delete: bool,
  hide: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sentence {
  contents: Vec<SentenceElement>,
  num: usize,
  function: SentenceFunction,
  indent: SentenceIndent,
  writing_mode: text::WritingMode,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SentenceElement {
  Line(line::Line),
  QuoteStruct(QuoteStruct),
  ArithFormula(ArithFormula),
  Ruby(text::Ruby),
  Sup(text::Sup),
  Sub(text::Sub),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SentenceFunction {
  Main,
  Proviso,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SentenceIndent {
  Paragraph,
  Item,
  Subitem1,
  Subitem2,
  Subitem3,
  Subitem4,
  Subitem5,
  Subitem6,
  Subitem7,
  Subitem8,
  Subitem9,
  Subitem10,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Column {
  sentence: Vec<Sentence>,
  num: usize,
  line_break: bool,
  align: Align,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Align {
  Left,
  Center,
  Right,
  Justify,
}

/// 附則
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvision {
  label: text::Text,
  children: Vec<SupplProvisionChildrenElement>,
  appdx_table: Vec<SupplProvisionAppdxTable>,
  appdx_style: Vec<SupplProvisionAppdxStyle>,
  appdx: Vec<SupplProvisionAppdx>,
  suppl_provision_type: Option<SupplProvisionType>,
  amend_law_num: Option<String>,
  extract: Option<bool>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SupplProvisionChildrenElement {
  Chapter(article::Chapter),
  Article(article::Article),
  Paragraph(Paragraph),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SupplProvisionType {
  New,
  Amend,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvisionAppdxTable {
  title: text::TextWithWritingMode,
  related_article_num: Option<text::Text>,
  table_struct: Vec<TableStruct>,
  num: Option<usize>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvisionAppdxStyle {
  title: text::TextWithWritingMode,
  related_article_num: Option<text::Text>,
  table_struct: Vec<StyleStruct>,
  num: Option<usize>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SupplProvisionAppdx {
  airth_formula_num: Option<text::Text>,
  related_article_num: Option<text::Text>,
  airth_formula: Vec<ArithFormula>,
  num: Option<usize>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxTable {
  title: Option<text::TextWithWritingMode>,
  related_article_num: Option<text::Text>,
  children: Vec<AppdxTableContents>,
  remarks: Option<Remarks>,
  num: Option<usize>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum AppdxTableContents {
  TableStruct(TableStruct),
  Item(Item),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxNote {
  title: Option<text::TextWithWritingMode>,
  related_article_num: Option<text::Text>,
  children: Vec<AppdxNoteContents>,
  remarks: Option<Remarks>,
  num: Option<usize>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum AppdxNoteContents {
  NoteStruct(NoteStruct),
  FigStruct(FigStruct),
  TableStruct(TableStruct),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxStyle {
  title: Option<text::TextWithWritingMode>,
  related_article_num: Option<text::Text>,
  children: Vec<StyleStruct>,
  remarks: Option<Remarks>,
  num: Option<usize>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxFormat {
  title: Option<text::TextWithWritingMode>,
  related_article_num: Option<text::Text>,
  children: Vec<FormatStruct>,
  remarks: Option<Remarks>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Appdx {
  airth_formula_num: Option<text::Text>,
  related_article_num: Option<text::Text>,
  arith_formula: Vec<ArithFormula>,
  remarks: Option<Remarks>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AppdxFig {
  title: Option<text::TextWithWritingMode>,
  related_article_num: Option<text::Text>,
  children: Vec<AppdxFigContents>,
  num: Option<usize>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum AppdxFigContents {
  FigStruct(FigStruct),
  TableStruct(TableStruct),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TableStruct {
  title: Option<text::TextWithWritingMode>,
  title_remarks: Vec<Remarks>,
  table: Table,
  table_remarks: Vec<Remarks>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Table {
  table_header_row: Vec<TableHeaderRow>,
  table_row: Vec<TableRow>,
  writing_mode: text::WritingMode,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TableHeaderRow {
  columns: Vec<text::Text>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TableRow {
  columns: Vec<TableColumn>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TableColumn {
  contents: TableColumnContents,
  border_top: line::LineStyle,
  border_bottom: line::LineStyle,
  border_left: line::LineStyle,
  border_right: line::LineStyle,
  rowspan: Option<String>,
  colspan: Option<String>,
  align: Option<Align>,
  valign: Option<Position>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum TableColumnContents {
  Part(article::Part),
  Chapter(article::Chapter),
  Section(article::Section),
  Subsection(article::Subsection),
  Division(article::Division),
  Article(article::Article),
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
  FigStruct(FigStruct),
  Sentence(Sentence),
  Column(Column),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Position {
  Top,
  Middle,
  Bottom,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct FigStruct {
  title: Option<text::Text>,
  title_remarks: Option<Remarks>,
  fig: Fig,
  fig_remarks: Option<Remarks>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Fig {
  src: String,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct NoteStruct {
  title: Option<text::Text>,
  title_remarks: Option<Remarks>,
  note: Note,
  note_remarks: Option<Remarks>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Note {
  // TODO
  // contents: any
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct StyleStruct {
  title: Option<text::Text>,
  title_remarks: Option<Remarks>,
  style: Style,
  style_remarks: Option<Remarks>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Style {
  // TODO
  // contents: any
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct FormatStruct {
  title: Option<text::Text>,
  title_remarks: Option<Remarks>,
  format: Format,
  format_remarks: Option<Remarks>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Format {
  // TODO
  // contents: any
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Remarks {
  label: RemarksLabel,
  children: Vec<RemarksContents>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct RemarksLabel {
  text: text::Text,
  line_break: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum RemarksContents {
  Item(Item),
  Sentence(Sentence),
}

// TODO
/// 引用
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct QuoteStruct {
  // contents: any
}

/// 数式
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ArithFormula {
  num: text::Text,
  // contents: any
}
