//! 表
use crate::article::*;
use crate::class::*;
use crate::line::*;
use crate::paragraph::*;
use crate::parser::*;
use crate::remarks::*;
use crate::result::Error;
use crate::sentence::*;
use crate::structs::*;
use crate::text::*;
use crate::to_xml::*;
use crate::*;
use serde::{Deserialize, Serialize};
use xmltree::{Element, XMLNode};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct Table {
  pub table_header_row: Vec<TableHeaderRow>,
  pub table_row: Vec<TableRow>,
  pub writing_mode: text::WritingMode,
}

impl Parser for Table {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "Table" {
      let writing_mode = match element.attributes.get("WritingMode").map(|s| s.as_str()) {
        Some("vetical") => WritingMode::Vertical,
        Some("horizontal") => WritingMode::Horizontal,
        _ => WritingMode::Vertical,
      };
      let mut table_header_row = Vec::new();
      let mut table_row = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "TableHeaderRow" => {
              let v = TableHeaderRow::parser(e)?;
              table_header_row.push(v)
            }
            "TableRow" => {
              let v = TableRow::parser(e)?;
              table_row.push(v)
            }
            s => return Err(Error::unexpected_tag(element, s)),
          }
        }
      }
      Ok(Table {
        table_header_row,
        table_row,
        writing_mode,
      })
    } else {
      Err(Error::wrong_tag_name(element, "Table"))
    }
  }
}

impl ToXmlElement for Table {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("Table");
    for v in self.table_header_row.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()))
    }
    for v in self.table_row.iter() {
      e.children.push(XMLNode::Element(v.to_xml_element()))
    }
    if text::WritingMode::Horizontal == self.writing_mode {
      e.attributes
        .insert("WritingMode".to_string(), "horizontal".to_string());
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableHeaderRow {
  pub columns: Vec<Text>,
}

impl Parser for TableHeaderRow {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TableHeaderRow" {
      let mut columns = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          if e.name.as_str() == "TableHeaderColumn" {
            let text = Text::from_children(&e.children);
            columns.push(text);
          }
        }
      }
      Ok(TableHeaderRow { columns })
    } else {
      Err(Error::wrong_tag_name(element, "TableHeaderRow"))
    }
  }
}

impl ToXmlElement for TableHeaderRow {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TableHeaderRow");
    e.children = self
      .columns
      .iter()
      .map(|text| XMLNode::Element(text.to_xml_element_with_name("TableHeaderColumn")))
      .collect();
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableRow {
  pub columns: Vec<TableColumn>,
}

impl Parser for TableRow {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TableRow" {
      let mut columns = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          if e.name.as_str() == "TableColumn" {
            let v = TableColumn::parser(e)?;
            columns.push(v);
          }
        }
      }
      Ok(TableRow { columns })
    } else {
      Err(Error::wrong_tag_name(element, "TableRow"))
    }
  }
}

impl ToXmlElement for TableRow {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TableRow");
    e.children = self
      .columns
      .iter()
      .map(|c| XMLNode::Element(c.to_xml_element()))
      .collect();
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableColumn {
  pub contents: Vec<TableColumnContents>,
  pub border_top: LineStyle,
  pub border_bottom: LineStyle,
  pub border_left: LineStyle,
  pub border_right: LineStyle,
  pub rowspan: Option<String>,
  pub colspan: Option<String>,
  pub align: Option<Align>,
  pub valign: Option<Position>,
}

impl Parser for TableColumn {
  fn parser(element: &Element) -> result::Result<Self> {
    if element.name.as_str() == "TableColumn" {
      let border_top =
        LineStyle::from_attribute(element.attributes.get("BorderTop")).unwrap_or(LineStyle::Solid);
      let border_bottom = LineStyle::from_attribute(element.attributes.get("BorderBottom"))
        .unwrap_or(LineStyle::Solid);
      let border_left =
        LineStyle::from_attribute(element.attributes.get("BorderLeft")).unwrap_or(LineStyle::Solid);
      let border_right = LineStyle::from_attribute(element.attributes.get("BorderRight"))
        .unwrap_or(LineStyle::Solid);
      let rowspan = get_attribute_opt_with_parse(element, "rowspan")?;
      let colspan = get_attribute_opt_with_parse(element, "colspan")?;
      let align = Align::from_attribute(element.attributes.get("Align"));
      let valign = Position::from_attribute(element.attributes.get("Valign"));
      let mut contents = Vec::new();
      for node in element.children.iter() {
        if let XMLNode::Element(e) = node {
          match e.name.as_str() {
            "Part" => {
              let v = Part::parser(e)?;
              contents.push(TableColumnContents::Part(v));
            }
            "Chapter" => {
              let v = Chapter::parser(e)?;
              contents.push(TableColumnContents::Chapter(v));
            }
            "Section" => {
              let v = Section::parser(e)?;
              contents.push(TableColumnContents::Section(v));
            }
            "Subsection" => {
              let v = Subsection::parser(e)?;
              contents.push(TableColumnContents::Subsection(v));
            }
            "Division" => {
              let v = Division::parser(e)?;
              contents.push(TableColumnContents::Division(v));
            }
            "Article" => {
              let v = Article::parser(e)?;
              contents.push(TableColumnContents::Article(v));
            }
            "Paragraph" => {
              let v = Paragraph::parser(e)?;
              contents.push(TableColumnContents::Paragraph(v));
            }
            "Item" => {
              let v = Item::parser(e)?;
              contents.push(TableColumnContents::Item(v));
            }
            "Subitem1" => {
              let v = Subitem1::parser(e)?;
              contents.push(TableColumnContents::Subitem1(v));
            }
            "Subitem2" => {
              let v = Subitem2::parser(e)?;
              contents.push(TableColumnContents::Subitem2(v));
            }
            "Subitem3" => {
              let v = Subitem3::parser(e)?;
              contents.push(TableColumnContents::Subitem3(v));
            }
            "Subitem4" => {
              let v = Subitem4::parser(e)?;
              contents.push(TableColumnContents::Subitem4(v));
            }
            "Subitem5" => {
              let v = Subitem5::parser(e)?;
              contents.push(TableColumnContents::Subitem5(v));
            }
            "Subitem6" => {
              let v = Subitem6::parser(e)?;
              contents.push(TableColumnContents::Subitem6(v));
            }
            "Subitem7" => {
              let v = Subitem7::parser(e)?;
              contents.push(TableColumnContents::Subitem7(v));
            }
            "Subitem8" => {
              let v = Subitem8::parser(e)?;
              contents.push(TableColumnContents::Subitem8(v));
            }
            "Subitem9" => {
              let v = Subitem9::parser(e)?;
              contents.push(TableColumnContents::Subitem9(v));
            }
            "Subitem10" => {
              let v = Subitem10::parser(e)?;
              contents.push(TableColumnContents::Subitem10(v));
            }
            "FigStruct" => {
              let v = FigStruct::parser(e)?;
              contents.push(TableColumnContents::FigStruct(v));
            }
            "Remarks" => {
              let v = Remarks::parser(e)?;
              contents.push(TableColumnContents::Remarks(v));
            }
            "Sentence" => {
              let v = Sentence::parser(e)?;
              contents.push(TableColumnContents::Sentence(v));
            }
            "Column" => {
              let v = Column::parser(e)?;
              contents.push(TableColumnContents::Column(v));
            }
            s => return Err(Error::unexpected_tag(element, s)),
          }
        } else if let XMLNode::Text(s) = node {
          contents.push(TableColumnContents::String(s.clone()))
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
      Err(Error::wrong_tag_name(element, "TableColumn"))
    }
  }
}

impl ToXmlElement for TableColumn {
  fn to_xml_element(&self) -> Element {
    let mut e = Element::new("TableColumn");
    e.attributes
      .insert("BorderTop".to_string(), self.border_top.to_attribute());
    e.attributes.insert(
      "BorderBottom".to_string(),
      self.border_bottom.to_attribute(),
    );
    e.attributes
      .insert("BorderLeft".to_string(), self.border_left.to_attribute());
    e.attributes
      .insert("BorderRight".to_string(), self.border_right.to_attribute());
    if let Some(s) = &self.rowspan {
      e.attributes.insert("rowspan".to_string(), s.clone());
    }
    if let Some(s) = &self.colspan {
      e.attributes.insert("colspan".to_string(), s.clone());
    }
    if let Some(a) = &self.align {
      e.attributes.insert("rowspan".to_string(), a.to_attribute());
    }
    if let Some(v) = &self.valign {
      e.attributes.insert("rowspan".to_string(), v.to_attribute());
    }
    for n in self.contents.iter() {
      match n {
        TableColumnContents::Part(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Chapter(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Section(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subsection(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Division(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Article(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Paragraph(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Item(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem1(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem2(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem3(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem4(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem5(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem6(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem7(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem8(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem9(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Subitem10(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::FigStruct(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Remarks(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Sentence(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::Column(v) => e.children.push(XMLNode::Element(v.to_xml_element())),
        TableColumnContents::String(s) => e.children.push(XMLNode::Text(s.clone())),
      }
    }
    e
  }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum Position {
  Top,
  Middle,
  Bottom,
}

impl Position {
  pub fn from_attribute(att: Option<&String>) -> Option<Self> {
    match att.map(|s| s.as_str()) {
      Some("top") => Some(Position::Top),
      Some("middle") => Some(Position::Middle),
      Some("bottom") => Some(Position::Bottom),
      _ => None,
    }
  }

  pub fn to_attribute(&self) -> String {
    match self {
      Self::Bottom => "bottom".to_string(),
      Self::Top => "top".to_string(),
      Self::Middle => "middle".to_string(),
    }
  }
}
