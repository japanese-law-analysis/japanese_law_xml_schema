#[cfg(test)]
mod parse_mod {
  use crate::{article::PartContents, law::MainProvisionContents, sentence::SentenceElement, *};
  #[test]
  fn test_129ac0000000089() {
    // 民法
    use table_of_contents::*;
    use text::*;
    let law = parse_xml_file("src/tests/129AC0000000089_20230614_505AC0000000053.xml").unwrap();
    let toc = &law.law_body.toc.unwrap().toc_main_contents[0];
    match &toc {
      &TOCMainContents::TOCPart(v) => {
        assert_eq!(v.part_title, Text::from_value("第一編　総則"));
        let chap = &v.children[0];
        assert_eq!(&chap.chapter_title, &Text::from_value("第一章　通則"));
        assert_eq!(
          &chap.article_range.clone().unwrap(),
          &Text::from_value("（第一条・第二条）")
        );
        assert_eq!(chap.num, "1".to_string());
      }
      _ => unreachable!(),
    }

    let main = &law.law_body.main_provision.children[0];
    match &main {
      &MainProvisionContents::Part(v) => match &v.children[0] {
        PartContents::Chapter(v) => match &v.children[0] {
          article::ChapterContents::Article(v) => {
            assert_eq!(&v.title, &Text::from_value("第一条"));
            assert_eq!(
              v.caption.clone().unwrap().text,
              Text::from_value("（基本原則）")
            );
            let para_lst = &v.paragraph;
            let para_1 = &para_lst[0];
            let sentence = &para_1.sentence[0];
            match &sentence.contents[0] {
              SentenceElement::String(s) => {
                assert_eq!(s, "私権は、公共の福祉に適合しなければならない。")
              }
              _ => unreachable!(),
            }
          }
          _ => unreachable!(),
        },
        _ => unreachable!(),
      },
      _ => unreachable!(),
    }
  }
}
