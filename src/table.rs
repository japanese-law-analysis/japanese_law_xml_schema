//! 表
use crate::article::*;
use crate::line::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::remarks::*;
use crate::result::Error;
use crate::sentence::*;
use crate::structs::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Table {
  table_header_row: Vec<TableHeaderRow>,
  table_row: Vec<TableRow>,
  writing_mode: text::WritingMode,
}

impl Parser for Table {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "Table" {
      let writing_mode = match node.attribute("WritingMode") {
        Some("vetical") => WritingMode::Vertical,
        Some("horizontal") => WritingMode::Horizontal,
        _ => return Err(Error::Attribute),
      };
      let mut table_header_row = Vec::new();
      let mut table_row = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "TableHeaderRow" => {
            if let Ok(v) = TableHeaderRow::parser(&node) {
              table_header_row.push(v)
            }
          }
          "TableRow" => {
            if let Ok(v) = TableRow::parser(&node) {
              table_row.push(v)
            }
          }
          _ => {}
        }
      }
      Ok(Table {
        table_header_row,
        table_row,
        writing_mode,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TableHeaderRow {
  columns: Vec<Text>,
}

impl Parser for TableHeaderRow {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TableHeaderRow" {
      let mut columns = Vec::new();
      for node in node.children() {
        if node.tag_name().name() == "TableHeaderColumn" {
          let text = Text::from_children(node.children());
          columns.push(text);
        }
      }
      Ok(TableHeaderRow { columns })
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TableRow {
  columns: Vec<TableColumn>,
}

impl Parser for TableRow {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TableRow" {
      let mut columns = Vec::new();
      for node in node.children() {
        if node.tag_name().name() == "TableColumn" {
          if let Ok(v) = TableColumn::parser(&node) {
            columns.push(v);
          }
        }
      }
      Ok(TableRow { columns })
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TableColumn {
  contents: Vec<TableColumnContents>,
  border_top: LineStyle,
  border_bottom: LineStyle,
  border_left: LineStyle,
  border_right: LineStyle,
  rowspan: Option<String>,
  colspan: Option<String>,
  align: Option<Align>,
  valign: Option<Position>,
}

impl Parser for TableColumn {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TableColumn" {
      let border_top =
        LineStyle::from_attribute(node.attribute("BorderTop")).unwrap_or(LineStyle::Solid);
      let border_bottom =
        LineStyle::from_attribute(node.attribute("BorderBottom")).unwrap_or(LineStyle::Solid);
      let border_left =
        LineStyle::from_attribute(node.attribute("BorderLeft")).unwrap_or(LineStyle::Solid);
      let border_right =
        LineStyle::from_attribute(node.attribute("BorderRight")).unwrap_or(LineStyle::Solid);
      let rowspan = node.attribute("rowspan").map(|s| s.to_string());
      let colspan = node.attribute("colspan").map(|s| s.to_string());
      let align = Align::from_attribute(node.attribute("Align"));
      let valign = Position::from_attribute(node.attribute("Valign"));
      let mut contents = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "Part" => {
            if let Ok(v) = Part::parser(&node) {
              contents.push(TableColumnContents::Part(v));
            }
          }
          "Chapter" => {
            if let Ok(v) = Chapter::parser(&node) {
              contents.push(TableColumnContents::Chapter(v));
            }
          }
          "Section" => {
            if let Ok(v) = Section::parser(&node) {
              contents.push(TableColumnContents::Section(v));
            }
          }
          "Subsection" => {
            if let Ok(v) = Subsection::parser(&node) {
              contents.push(TableColumnContents::Subsection(v));
            }
          }
          "Division" => {
            if let Ok(v) = Division::parser(&node) {
              contents.push(TableColumnContents::Division(v));
            }
          }
          "Article" => {
            if let Ok(v) = Article::parser(&node) {
              contents.push(TableColumnContents::Article(v));
            }
          }
          "Item" => {
            if let Ok(v) = Item::parser(&node) {
              contents.push(TableColumnContents::Item(v));
            }
          }
          "Subitem1" => {
            if let Ok(v) = Subitem1::parser(&node) {
              contents.push(TableColumnContents::Subitem1(v));
            }
          }
          "Subitem2" => {
            if let Ok(v) = Subitem2::parser(&node) {
              contents.push(TableColumnContents::Subitem2(v));
            }
          }
          "Subitem3" => {
            if let Ok(v) = Subitem3::parser(&node) {
              contents.push(TableColumnContents::Subitem3(v));
            }
          }
          "Subitem4" => {
            if let Ok(v) = Subitem4::parser(&node) {
              contents.push(TableColumnContents::Subitem4(v));
            }
          }
          "Subitem5" => {
            if let Ok(v) = Subitem5::parser(&node) {
              contents.push(TableColumnContents::Subitem5(v));
            }
          }
          "Subitem6" => {
            if let Ok(v) = Subitem6::parser(&node) {
              contents.push(TableColumnContents::Subitem6(v));
            }
          }
          "Subitem7" => {
            if let Ok(v) = Subitem7::parser(&node) {
              contents.push(TableColumnContents::Subitem7(v));
            }
          }
          "Subitem8" => {
            if let Ok(v) = Subitem8::parser(&node) {
              contents.push(TableColumnContents::Subitem8(v));
            }
          }
          "Subitem9" => {
            if let Ok(v) = Subitem9::parser(&node) {
              contents.push(TableColumnContents::Subitem9(v));
            }
          }
          "Subitem10" => {
            if let Ok(v) = Subitem10::parser(&node) {
              contents.push(TableColumnContents::Subitem10(v));
            }
          }
          "FigStruct" => {
            if let Ok(v) = FigStruct::parser(&node) {
              contents.push(TableColumnContents::FigStruct(v));
            }
          }
          "Remarks" => {
            if let Ok(v) = Remarks::parser(&node) {
              contents.push(TableColumnContents::Remarks(v));
            }
          }
          "Sentence" => {
            if let Ok(v) = Sentence::parser(&node) {
              contents.push(TableColumnContents::Sentence(v));
            }
          }
          "Column" => {
            if let Ok(v) = Column::parser(&node) {
              contents.push(TableColumnContents::Column(v));
            }
          }
          "" => {
            if let Some(v) = node.text() {
              contents.push(TableColumnContents::String(v.to_string()));
            }
          }
          _ => {}
        }
      }
      Ok(TableColumn {
        contents,
        border_top,
        border_bottom,
        border_left,
        border_right,
        rowspan,
        colspan,
        align,
        valign,
      })
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum TableColumnContents {
  Part(Part),
  Chapter(Chapter),
  Section(Section),
  Subsection(Subsection),
  Division(Division),
  Article(Article),
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
  Remarks(Remarks),
  Column(Column),
  String(String),
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Position {
  Top,
  Middle,
  Bottom,
}

impl Position {
  pub fn from_attribute(att: Option<&str>) -> Option<Self> {
    match att {
      Some("top") => Some(Position::Top),
      Some("middle") => Some(Position::Middle),
      Some("bottom") => Some(Position::Bottom),
      _ => None,
    }
  }
}
