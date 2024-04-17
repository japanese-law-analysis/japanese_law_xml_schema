#[cfg(test)]
mod write {
  use crate::{
    article::PartContents, law::MainProvisionContents, sentence::SentenceElement, text::*, *,
  };

  #[test]
  fn test_129ac0000000089() {
    let law = parse_xml_file("src/tests/129AC0000000089_20230614_505AC0000000053.xml").unwrap();
    let s = to_xml(&law).unwrap();
    println!("{s}");
    let law2 = parse_xml(&s.as_bytes()).unwrap();
    let main = &law2.law_body.main_provision.children[0];
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
