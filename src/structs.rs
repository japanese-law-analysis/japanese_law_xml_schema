//! 構造体一般

use crate::contents::*;
use crate::fig::*;
use crate::parser::*;
use crate::remarks::*;
use crate::result::Error;
use crate::table::*;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum Struct {
  TableStruct(TableStruct),
  FigStruct(FigStruct),
  StyleStruct(StyleStruct),
  List(list::List),
}

impl Parser for Struct {
  fn parser(element: &Element) -> result::Result<Self> {
    match element.name.as_str() {
      "TableStruct" => TableStruct::parser(element).map(Struct::TableStruct),
      "FigStruct" => FigStruct::parser(element).map(Struct::FigStruct),
      "StyleStruct" => StyleStruct::parser(element).map(Struct::StyleStruct),
      "List" => list::List::parser(element).map(Struct::List),
      _ => Err(Error::missing_required_tag("Struct")),
    }
  }
}

impl ToXmlElement for Struct {
  fn to_xml_element(&self) -> Element {
    match self {
      Struct::FigStruct(v) => v.to_xml_element(),
      Struct::TableStruct(v) => v.to_xml_element(),
      Struct::StyleStruct(v) => v.to_xml_element(),
      Struct::List(v) => v.to_xml_element(),
    }
  }
}

/// 引用
#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuoteStruct {
  pub contentes: contents::Contents,
}

impl Parser for QuoteStruct {
  fn parser(element: &Element) -> result::Result<Self> {
    contents::Contents::parser(element).map(|c| QuoteStruct { contentes: c })
  }
}

impl ToXmlElement for QuoteStruct {
  fn to_xml_element(&self) -> Element {
    self.contentes.to_xml_element_with_name("QuoteStruct")
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "NoteStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut note = None;
      let mut note_remarks = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "NoteStructTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Remarks" => {
              if let Ok(r) = Remarks::parser(e) {
                if note.is_none() {
                  title_remarks.push(r);
                } else {
                  note_remarks.push(r);
                }
              }
            }
            "Note" => {
              let v = Note::parser(e)?;
              note = Some(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
          tag_name: "Note".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "NoteStruct"))
    }
  }
}

impl ToXmlElement for NoteStruct {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("NoteStruct");
    if let Some(t) = &self.title {
      e.children.push(XMLNode::Element(
        t.to_xml_element_with_name("NoteStructTitle"),
      ))
    }
    for n in self.title_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e.children
      .push(XMLNode::Element(self.note.to_xml_element()));
    for n in self.note_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "StyleStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut style = None;
      let mut style_remarks = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "StyleStructTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Remarks" => {
              if let Ok(r) = Remarks::parser(e) {
                if style.is_none() {
                  title_remarks.push(r);
                } else {
                  style_remarks.push(r);
                }
              }
            }
            "Style" => {
              let v = Style::parser(e)?;
              style = Some(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
          tag_name: "Style".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "StyleStruct"))
    }
  }
}

impl ToXmlElement for StyleStruct {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("StyleStruct");
    if let Some(t) = &self.title {
      e.children.push(XMLNode::Element(
        t.to_xml_element_with_name("StyleStructTitle"),
      ))
    }
    for n in self.title_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e.children
      .push(XMLNode::Element(self.style.to_xml_element()));
    for n in self.style_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "FormatStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut format = None;
      let mut format_remarks = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "FormatStructTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Remarks" => {
              if let Ok(r) = Remarks::parser(e) {
                if format.is_none() {
                  title_remarks.push(r);
                } else {
                  format_remarks.push(r);
                }
              }
            }
            "Format" => {
              let v = Format::parser(e)?;
              format = Some(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
          tag_name: "Format".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "FormatStruct"))
    }
  }
}

impl ToXmlElement for FormatStruct {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("FormatStruct");
    if let Some(t) = &self.title {
      e.children.push(XMLNode::Element(
        t.to_xml_element_with_name("FormatStructTitle"),
      ))
    }
    for n in self.title_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e.children
      .push(XMLNode::Element(self.format.to_xml_element()));
    for n in self.format_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "FigStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut fig = None;
      let mut fig_remarks = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "FigStructTitle" => {
              title = Some(Text::from_children(&e.children));
            }
            "Remarks" => {
              if let Ok(r) = Remarks::parser(e) {
                if fig.is_none() {
                  title_remarks.push(r);
                } else {
                  fig_remarks.push(r);
                }
              }
            }
            "Fig" => {
              let v = Fig::parser(e)?;
              fig = Some(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
          tag_name: "Fig".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "FigStruct"))
    }
  }
}

impl ToXmlElement for FigStruct {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("FigStruct");
    if let Some(t) = &self.title {
      e.children.push(XMLNode::Element(
        t.to_xml_element_with_name("FigStructTitle"),
      ))
    }
    for n in self.title_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e.children.push(XMLNode::Element(self.fig.to_xml_element()));
    for n in self.fig_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e
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
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TableStruct" {
      let mut title = None;
      let mut title_remarks = Vec::new();
      let mut table = None;
      let mut table_remarks = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "TableStructTitle" => {
              let v = TextWithWritingMode::parser(e)?;
              title = Some(v);
            }
            "Remarks" => {
              if let Ok(r) = Remarks::parser(e) {
                if table.is_none() {
                  title_remarks.push(r);
                } else {
                  table_remarks.push(r);
                }
              }
            }
            "Table" => {
              let v = Table::parser(e)?;
              table = Some(v);
            }
            s => return Err(Error::unexpected_tag(e, s)),
          }
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
          tag_name: "Table".to_string(),
        })
      }
    } else {
      Err(Error::wrong_tag_name(element, "TableStruct"))
    }
  }
}

impl ToXmlElement for TableStruct {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TableStruct");
    if let Some(t) = &self.title {
      e.children.push(XMLNode::Element(
        t.to_xml_element_with_name("TableStructTitle"),
      ))
    }
    for n in self.title_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e.children
      .push(XMLNode::Element(self.table.to_xml_element()));
    for n in self.table_remarks.iter() {
      e.children.push(XMLNode::Element(n.to_xml_element()))
    }
    e
  }
}
