use regex::Regex;
use crate::result::*;
use crate::*;


pub(crate) fn parse_body(title: &str, text: &str) -> Result<law::LawBody> {
  let lines = text.lines().map(|s| s.trim());
  for line in lines {}
  todo!()
}


fn get_articles(line: &str) {
  let re = Regex::new("第(([0-9]+)|([０-９]+)|([一二三四五六七八九十百千]+))条");
}

