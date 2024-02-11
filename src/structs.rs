use crate::contents::*;
use crate::fig::*;
use crate::parser::*;
use crate::remarks::*;
use crate::result::Error;
use crate::table::*;
use crate::text::*;
use crate::*;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Struct {
  TableStruct(TableStruct),
  FigStruct(FigStruct),
  StyleStruct(StyleStruct),
  List(list::List),
}

impl Parser for Struct {
  fn parser(node: &Node) -> result::Result<Self> {
    match node.tag_name().name() {
      "TableStruct" => TableStruct::parser(node).map(Struct::TableStruct),
      "FigStruct" => FigStruct::parser(node).map(Struct::FigStruct),
      "StyleStruct" => StyleStruct::parser(node).map(Struct::StyleStruct),
      "List" => list::List::parser(node).map(Struct::List),
      _ => Err(Error::Tag),
    }
  }
}

/// 引用
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct QuoteStruct {
  contentes: contents::Contents,
}

impl Parser for QuoteStruct {
  fn parser(node: &Node) -> result::Result<Self> {
    contents::Contents::parser(node).map(|c| QuoteStruct { contentes: c })
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct NoteStruct {
  title: Option<Text>,
  title_remarks: Vec<Remarks>,
  note: contents::Note,
  note_remarks: Vec<Remarks>,
}

impl Parser for NoteStruct {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "NoteStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut note = None;
      let mut note_remarks = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "NoteStructTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Remarks" => {
            if let Ok(r) = Remarks::parser(&node) {
              if note.is_none() {
                title_remarks.push(r);
              } else {
                note_remarks.push(r);
              }
            }
          }
          "Note" => {
            if let Ok(n) = Note::parser(&node) {
              note = Some(n);
            }
          }
          _ => {}
        }
      }
      if let Some(note) = note {
        Ok(NoteStruct {
          title,
          title_remarks,
          note,
          note_remarks,
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct StyleStruct {
  title: Option<Text>,
  title_remarks: Vec<Remarks>,
  style: Style,
  style_remarks: Vec<Remarks>,
}

impl Parser for StyleStruct {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "StyleStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut style = None;
      let mut style_remarks = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "StyleStructTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Remarks" => {
            if let Ok(r) = Remarks::parser(&node) {
              if style.is_none() {
                title_remarks.push(r);
              } else {
                style_remarks.push(r);
              }
            }
          }
          "Style" => {
            if let Ok(s) = Style::parser(&node) {
              style = Some(s);
            }
          }
          _ => {}
        }
      }
      if let Some(style) = style {
        Ok(StyleStruct {
          title,
          title_remarks,
          style,
          style_remarks,
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct FormatStruct {
  title: Option<Text>,
  title_remarks: Vec<Remarks>,
  format: Format,
  format_remarks: Vec<Remarks>,
}

impl Parser for FormatStruct {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "FormatStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut format = None;
      let mut format_remarks = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "FormatStructTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Remarks" => {
            if let Ok(r) = Remarks::parser(&node) {
              if format.is_none() {
                title_remarks.push(r);
              } else {
                format_remarks.push(r);
              }
            }
          }
          "Format" => {
            if let Ok(f) = Format::parser(&node) {
              format = Some(f);
            }
          }
          _ => {}
        }
      }
      if let Some(format) = format {
        Ok(FormatStruct {
          title,
          title_remarks,
          format,
          format_remarks,
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct FigStruct {
  title: Option<Text>,
  title_remarks: Vec<Remarks>,
  fig: Fig,
  fig_remarks: Vec<Remarks>,
}

impl Parser for FigStruct {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "FigStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut fig = None;
      let mut fig_remarks = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "FigStructTitle" => {
            title = Some(Text::from_children(node.children()));
          }
          "Remarks" => {
            if let Ok(r) = Remarks::parser(&node) {
              if fig.is_none() {
                title_remarks.push(r);
              } else {
                fig_remarks.push(r);
              }
            }
          }
          "FIg" => {
            if let Ok(f) = Fig::parser(&node) {
              fig = Some(f);
            }
          }
          _ => {}
        }
      }
      if let Some(fig) = fig {
        Ok(FigStruct {
          title,
          title_remarks,
          fig,
          fig_remarks,
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct TableStruct {
  title: Option<TextWithWritingMode>,
  title_remarks: Vec<Remarks>,
  table: Table,
  table_remarks: Vec<Remarks>,
}

impl Parser for TableStruct {
  fn parser(node: &Node) -> result::Result<Self> {
    if node.tag_name().name() == "TableStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut table = None;
      let mut table_remarks = Vec::new();
      for node in node.children() {
        match node.tag_name().name() {
          "TableStructTitle" => {
            if let Ok(t) = TextWithWritingMode::parser(&node) {
              title = Some(t);
            } else {
              return Err(Error::Tag);
            }
          }
          "Remarks" => {
            if let Ok(r) = Remarks::parser(&node) {
              if table.is_none() {
                title_remarks.push(r);
              } else {
                table_remarks.push(r);
              }
            }
          }
          "Table" => {
            if let Ok(t) = Table::parser(&node) {
              table = Some(t);
            }
          }
          _ => {}
        }
      }
      if let Some(table) = table {
        Ok(TableStruct {
          title,
          title_remarks,
          table,
          table_remarks,
        })
      } else {
        Err(Error::Tag)
      }
    } else {
      Err(Error::Tag)
    }
  }
}
