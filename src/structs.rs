//! 構造体一般

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

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
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
      _ => Err(Error::missing_required_tag(&node.range(), "Struct")),
    }
  }
}

/// 引用
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuoteStruct {
  pub contentes: contents::Contents,
}

impl Parser for QuoteStruct {
  fn parser(node: &Node) -> result::Result<Self> {
    contents::Contents::parser(node).map(|c| QuoteStruct { contentes: c })
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteStruct {
  pub title: Option<Text>,
  pub title_remarks: Vec<Remarks>,
  pub note: contents::Note,
  pub note_remarks: Vec<Remarks>,
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
            let v = Note::parser(&node)?;
            note = Some(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "Note".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "NoteStruct"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct StyleStruct {
  pub title: Option<Text>,
  pub title_remarks: Vec<Remarks>,
  pub style: Style,
  pub style_remarks: Vec<Remarks>,
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
            let v = Style::parser(&node)?;
            style = Some(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "Style".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "StyleStruct"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
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
            let v = Format::parser(&node)?;
            format = Some(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "Format".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "FormatStruct"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct FigStruct {
  pub title: Option<Text>,
  pub title_remarks: Vec<Remarks>,
  pub fig: Fig,
  pub fig_remarks: Vec<Remarks>,
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
          "Fig" => {
            let v = Fig::parser(&node)?;
            fig = Some(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "Fig".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "FigStruct"))
    }
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableStruct {
  pub title: Option<TextWithWritingMode>,
  pub title_remarks: Vec<Remarks>,
  pub table: Table,
  pub table_remarks: Vec<Remarks>,
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
            let v = TextWithWritingMode::parser(&node)?;
            title = Some(v);
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
            let v = Table::parser(&node)?;
            table = Some(v);
          }
          s => return Err(Error::unexpected_tag(&node, s)),
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
        Err(Error::MissingRequiredTag {
          range: node.range(),
          tag_name: "Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(node, "TableStruct"))
    }
  }
}
