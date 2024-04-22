use crate::result::*;
use crate::*;
use article_number::ItemPattern;
use regex::Regex;

fn parse_part(
  n: &article_number::ArticleNumber,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Part {
  let part_title = text::Text::from_value(title);
  let delete = title.trim() == "削除" || title.trim() == "（削除）";
  let hide = false;
  let mut children = Vec::new();
  let mut caption_opt = None;
  loop {
    let next = lines.peek();
    match next {
      Some(LineContents::Caption(cap)) => {
        lines.next();
        caption_opt = Some(cap);
      }
      Some(LineContents::Article(n, text)) => {
        lines.next();
        let article = parse_article(caption_opt, n, text, lines);
        children.push(article::PartContents::Article(article));
        caption_opt = None;
      }
      Some(LineContents::Chapter(n, title)) => {
        lines.next();
        let chap = parse_chapter(n, title, lines);
        children.push(article::PartContents::Chapter(chap));
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Part {
    part_title,
    children,
    num: n.clone(),
    delete,
    hide,
  }
}

fn parse_chapter(
  n: &article_number::ArticleNumber,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Chapter {
  let chapter_title = text::Text::from_value(title);
  let delete = title.trim() == "削除" || title.trim() == "（削除）";
  let hide = false;
  let mut children = Vec::new();
  let mut caption_opt = None;
  loop {
    let next = lines.peek();
    match next {
      Some(LineContents::Caption(cap)) => {
        lines.next();
        caption_opt = Some(cap);
      }
      Some(LineContents::Article(n, text)) => {
        lines.next();
        let article = parse_article(caption_opt, n, text, lines);
        children.push(article::ChapterContents::Article(article));
        caption_opt = None;
      }
      Some(LineContents::Section(n, title)) => {
        lines.next();
        let sec = parse_section(n, title, lines);
        children.push(article::ChapterContents::Section(sec));
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Chapter {
    chapter_title,
    children,
    num: n.clone(),
    delete,
    hide,
  }
}

fn parse_section(
  n: &article_number::ArticleNumber,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Section {
  let section_title = text::Text::from_value(title);
  let delete = title.trim() == "削除" || title.trim() == "（削除）";
  let hide = false;
  let mut children = Vec::new();
  let mut caption_opt = None;
  loop {
    let next = lines.peek();
    match next {
      Some(LineContents::Caption(cap)) => {
        lines.next();
        caption_opt = Some(cap);
      }
      Some(LineContents::Article(n, text)) => {
        lines.next();
        let article = parse_article(caption_opt, n, text, lines);
        children.push(article::SectionContents::Article(article));
        caption_opt = None;
      }
      Some(LineContents::Subsection(n, title)) => {
        lines.next();
        let subsec = parse_subsection(n, title, lines);
        children.push(article::SectionContents::Subsection(subsec));
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Section {
    section_title,
    children,
    num: n.clone(),
    delete,
    hide,
  }
}

fn parse_subsection(
  n: &article_number::ArticleNumber,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Subsection {
  let subsection_title = text::Text::from_value(title);
  let delete = title.trim() == "削除" || title.trim() == "（削除）";
  let hide = false;
  let mut children = Vec::new();
  let mut caption_opt = None;
  loop {
    let next = lines.peek();
    match next {
      Some(LineContents::Caption(cap)) => {
        caption_opt = Some(cap);
        lines.next();
      }
      Some(LineContents::Article(n, text)) => {
        lines.next();
        let article = parse_article(caption_opt, n, text, lines);
        children.push(article::SubsectionContents::Article(article));
        caption_opt = None;
      }
      Some(LineContents::Division(n, title)) => {
        lines.next();
        let div = parse_division(n, title, lines);
        children.push(article::SubsectionContents::Division(div));
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Subsection {
    subsection_title,
    children,
    num: n.clone(),
    delete,
    hide,
  }
}

fn parse_division(
  n: &article_number::ArticleNumber,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Division {
  let division_title = text::Text::from_value(title);
  let delete = title.trim() == "削除" || title.trim() == "（削除）";
  let hide = false;
  let mut children = Vec::new();
  let mut caption_opt = None;
  loop {
    let next = lines.peek();
    match next {
      Some(LineContents::Caption(cap)) => {
        caption_opt = Some(cap);
        lines.next();
      }
      Some(LineContents::Article(n, text)) => {
        lines.next();
        let article = parse_article(caption_opt, n, text, lines);
        children.push(article);
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Division {
    division_title,
    children,
    num: n.clone(),
    delete,
    hide,
  }
}

fn parse_article(
  caption_opt: Option<&String>,
  n: &article_number::ArticleNumber,
  text: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Article {
  let caption = caption_opt.map(|s| class::Caption {
    text: text::Text::from_value(s),
    common_caption: None,
  });
  let para = parse_paragraph(1, text, lines);
  let mut paragraph = vec![para];
  while let Some(LineContents::Paragraph(n, text)) = lines.peek() {
    lines.next();
    let para = parse_paragraph(n.base_number, text, lines);
    paragraph.push(para)
  }
  let article = article::Article {
    caption,
    title: text::Text::from_value(String::new()), // TODO 「第○条」に生成
    paragraph,
    suppl_note: None,
    num: n.clone(),
    delete: text.trim() == "削除" || text.trim() == "（削除）",
    hide: false,
  };
  article
}

fn parse_paragraph(
  n: usize,
  text: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> paragraph::Paragraph {
  let mut sentence_text = vec![text];
  while let Some(LineContents::Text(s)) = lines.peek() {
    lines.next();
    sentence_text.push(s);
  }
  let sentence = sentence_text
    .iter()
    .enumerate()
    .map(|(n, s)| text_to_sentence(n + 1, s))
    .collect::<Vec<_>>();

  let mut children = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    lines.next();
    let mut sentence_text = vec![text];
    while let Some(LineContents::Text(s)) = lines.peek() {
      lines.next();
      sentence_text.push(s);
    }
    let mut subitem1 = Vec::new();
    if let Some(LineContents::Item(item_number_sub1, _)) = lines.peek() {
      if item_number_sub1.pattern != item_number.pattern {
        subitem1 = parse_subitem1(item_number.pattern, item_number_sub1.pattern, lines);
      }
    }
    let item = paragraph::Item {
      title: None,
      sentence: class::SentenceOrColumnOrTable::Sentence(
        sentence_text
          .iter()
          .enumerate()
          .map(|(n, s)| text_to_sentence(n + 1, s))
          .collect(),
      ),
      children: subitem1,
      struct_list: Vec::new(),
      num: item_number.number.to_string(),
      delete: text.trim() == "削除" || text.trim() == "（削除）",
      hide: false,
    };
    children.push(item);
  }

  paragraph::Paragraph {
    caption: None,
    paragraph_num: text::Text::new(), // TODO 謎
    amend_provision: Vec::new(),
    class: Vec::new(),
    sentence,
    struct_list: Vec::new(),
    children,
    num: n,
    old_style: false,
    old_num: false,
    hide: false,
  }
}

fn parse_subitem1(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem1> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem2 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem2 = parse_subitem2(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem1 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem2,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem2(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem2> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem3 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem3 = parse_subitem3(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem2 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem3,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem3(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem3> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem4 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem4 = parse_subitem4(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem3 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem4,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem4(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem4> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem5 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem5 = parse_subitem5(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem4 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem5,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem5(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem5> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem6 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem6 = parse_subitem6(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem5 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem6,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem6(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem6> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem7 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem7 = parse_subitem7(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem6 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem7,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem7(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem7> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem8 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem8 = parse_subitem8(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem7 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem8,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem8(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem8> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem9 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem9 = parse_subitem9(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem8 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem9,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem9(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem9> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem10 = Vec::new();
      if let Some(LineContents::Item(item_number, _)) = lines.peek() {
        if item_number.pattern != now_pat && item_number.pattern != parent_pattern {
          subitem10 = parse_subitem10(now_pat, item_number.pattern, lines);
        }
      }
      v.push(paragraph::Subitem9 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        children: subitem10,
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn parse_subitem10(
  parent_pattern: ItemPattern,
  now_pat: ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem10> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(item_number, text)) = lines.peek() {
    if item_number.pattern == parent_pattern {
      break;
    } else if item_number.pattern == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      v.push(paragraph::Subitem10 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n + 1, s))
            .collect(),
        ),
        struct_list: Vec::new(),
        num: item_number.number.to_string(),
        delete: text.trim() == "削除" || text.trim() == "（削除）",
        hide: false,
      })
    } else {
      unreachable!()
    }
  }
  v
}

fn text_to_sentence(num: usize, text: &str) -> sentence::Sentence {
  sentence::Sentence {
    contents: vec![sentence::SentenceElement::String(text.to_string())],
    num: Some(num),
    function: None,
    indent: None,
    writing_mode: text::WritingMode::Vertical,
  }
}

pub(crate) fn parse_body(title: &str, text: &str) -> Result<law::LawBody> {
  let mut lines = Vec::new();
  for line in text.lines().map(|s| s.trim()) {
    let line_contents = parse_line_contents(line);
    lines.push(line_contents)
  }
  let mut preamble_text_list: Vec<String> = Vec::new();
  let mut is_preamble = true;
  let mut main_provision_children = Vec::new();
  let mut suppl_provision = Vec::new();
  let mut suppl_provision_children = Vec::new();
  let mut suppl_provision_law_num_opt: Option<Option<String>> = None;

  let mut caption_opt = None;
  let mut lines = lines.iter().peekable();

  while let Some(line_contents) = lines.peek() {
    match line_contents {
      LineContents::Part(n, text) => {
        lines.next();
        let part = parse_part(n, text, &mut lines);
        main_provision_children.push(law::MainProvisionContents::Part(part));
        caption_opt = None;
        is_preamble = false;
      }
      LineContents::Chapter(n, text) => {
        lines.next();
        let chap = parse_chapter(n, text, &mut lines);
        if suppl_provision_law_num_opt.is_some() {
          suppl_provision_children.push(suppl_provision::SupplProvisionChildrenElement::Chapter(
            chap,
          ))
        } else {
          main_provision_children.push(law::MainProvisionContents::Chapter(chap));
        }
        caption_opt = None;
        is_preamble = false;
      }
      LineContents::Section(n, text) => {
        lines.next();
        let sec = parse_section(n, text, &mut lines);
        main_provision_children.push(law::MainProvisionContents::Section(sec));
        caption_opt = None;
        is_preamble = false;
      }
      LineContents::Caption(cap) => {
        caption_opt = Some(cap);
        is_preamble = false;
        lines.next();
      }
      LineContents::Article(n, text) => {
        lines.next();
        let article = parse_article(caption_opt, n, text, &mut lines);
        if suppl_provision_law_num_opt.is_some() {
          suppl_provision_children.push(suppl_provision::SupplProvisionChildrenElement::Article(
            article,
          ))
        } else {
          main_provision_children.push(law::MainProvisionContents::Article(article));
        }
        caption_opt = None;
        is_preamble = false;
      }
      LineContents::Text(s) => {
        if is_preamble {
          preamble_text_list.push(s.to_string());
          lines.next();
        }
      }
      LineContents::SupplProvision(law_num) => {
        if let Some(amend_law_num) = suppl_provision_law_num_opt {
          if !suppl_provision_children.is_empty() {
            suppl_provision.push(suppl_provision::SupplProvision {
              label: text::Text::from_value("附則"),
              children: suppl_provision_children,
              suppl_provision_type: None,
              amend_law_num,
              extract: None,
            })
          }
        }
        suppl_provision_law_num_opt = Some(law_num.clone());
        suppl_provision_children = Vec::new();
      }
      _ => break,
    }
  }

  let preamble = if preamble_text_list.is_empty() {
    None
  } else {
    let para_list = preamble_text_list
      .iter()
      .enumerate()
      .map(|(n, s)| {
        let sentence = text_to_sentence(0, s);
        paragraph::Paragraph {
          caption: None,
          paragraph_num: text::Text::new(), // TODO 謎
          amend_provision: Vec::new(),
          class: Vec::new(),
          sentence: vec![sentence],
          struct_list: Vec::new(),
          children: Vec::new(), // TODO
          num: n,
          old_style: false,
          old_num: false,
          hide: false,
        }
      })
      .collect();
    Some(law::Preamble {
      children: para_list,
    })
  };
  let main_provision = law::MainProvision {
    children: main_provision_children,
    extract: None,
  };

  if let Some(amend_law_num) = suppl_provision_law_num_opt {
    if !suppl_provision_children.is_empty() {
      suppl_provision.push(suppl_provision::SupplProvision {
        label: text::Text::from_value("附則"),
        children: suppl_provision_children,
        suppl_provision_type: None,
        amend_law_num,
        extract: None,
      })
    }
  }

  let law_title = law::LawTitle {
    kana: None,
    abbrev: None,
    abbrev_kana: None,
    text: text::Text::from_value(title),
  };
  Ok(law::LawBody {
    law_title: Some(law_title),
    enact_statement: Vec::new(),
    subject: None,
    toc: None,
    preamble,
    main_provision,
    suppl_provision,
    appdx_table: Vec::new(),
    appdx_note: Vec::new(),
    appdx_style: Vec::new(),
    appdx: Vec::new(),
    appdx_fig: Vec::new(),
    appdx_format: Vec::new(),
  })
}

/// 各行が何に当てはまるのかの種類
#[derive(Clone, Debug, PartialEq, Eq)]
enum LineContents {
  /// 見出し：（見出し）
  Caption(String),
  /// 編：第一編　タイトル
  Part(article_number::ArticleNumber, String),
  /// 章：第一章　タイトル
  Chapter(article_number::ArticleNumber, String),
  /// 節：第一節　タイトル
  Section(article_number::ArticleNumber, String),
  /// 款：第一款　タイトル
  Subsection(article_number::ArticleNumber, String),
  /// 目：第一目　タイトル
  Division(article_number::ArticleNumber, String),
  /// 条：第二条 本文
  Article(article_number::ArticleNumber, String),
  /// 項：２　本文
  Paragraph(article_number::ArticleNumber, String),
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
  Item(article_number::ItemNumber, String),
  /// 附則：附則（昭和三一年四月二日法律第六三号）
  SupplProvision(Option<String>),
  /// その他テキスト
  Text(String),
}

fn parse_line_contents(line: &str) -> LineContents {
  use LineContents::*;
  let line = line.trim();
  let re_caption = Regex::new("^（(?<caption>[^）]+)）$").unwrap();
  let re_suppl_provision =
    Regex::new(r"^附([　\s]*)則([　\s]*)(（(?<law_num>.+)）)?[^（）]*$").unwrap();
  if let Some(caps) = re_caption.captures(line) {
    Caption(caps["caption"].to_string())
  } else if let Some((article_number, text)) = article_number::parse_article_number(line) {
    let s = &article_number.str;
    if s.contains('編') {
      Part(article_number, text)
    } else if s.contains('章') {
      Chapter(article_number, text)
    } else if s.contains('節') {
      Section(article_number, text)
    } else if s.contains('款') {
      Subsection(article_number, text)
    } else if s.contains('目') {
      Division(article_number, text)
    } else if s.contains('条') {
      Article(article_number, text)
    } else {
      Paragraph(article_number, text)
    }
  } else if let Some((item_number, text)) = article_number::parse_item_number(line) {
    Item(item_number, text)
  } else if let Some(caps) = re_suppl_provision.captures(line) {
    let law_num = caps.name("law_num").map(|m| m.as_str().to_string());
    SupplProvision(law_num)
  } else {
    Text(line.to_string())
  }
}

#[test]
fn check_parse_line_contents_1() {
  use article_number::ArticleNumber;
  use article_number::ItemNumber;
  use article_number::ItemPattern::*;
  let s = r"第一編　総則
  第一章　通則
  （基本原則）
  第一条　私権は、公共の福祉に適合しなければならない。
  ２　権利の行使及び義務の履行は、信義に従い誠実に行わなければならない。
  ３　権利の濫用は、これを許さない。
  （解釈の基準）
  第二条の二　この法律は、個人の尊厳と両性の本質的平等を旨として、解釈しなければならない。
  第十三条　被保佐人が次に掲げる行為をするには、その保佐人の同意を得なければならない。ただし、第九条ただし書に規定する行為については、この限りでない。
  一　元本を領収し、又は利用すること。
  二　主たる債務者が法人である場合の次に掲げる者
    イ　主たる債務者の総株主の議決権（株主総会において決議をすることができる事項の全部につき議決権を行使することができない株式についての議決権を除く。以下この号において同じ。）の過半数を有する者
  三　不動産その他重要な財産に関する権利の得喪を目的とする行為をすること。
  ２　家庭裁判所は、第十一条本文に規定する者又は保佐人若しくは保佐監督人の請求により、被保佐人が前項各号に掲げる行為以外の行為をする場合であってもその保佐人の同意を得なければならない旨の審判をすることができる。ただし、第九条ただし書に規定する行為については、この限りでない。";
  let r = s.lines().map(parse_line_contents).collect::<Vec<_>>();
  assert_eq!(
    r,
    vec![
      LineContents::Part(ArticleNumber { str: "第一編".to_string(), num_str: "1".to_string(), base_number: 1, eda_numbers: Vec::new() }, "総則".to_string()),
      LineContents::Chapter(ArticleNumber { str: "第一章".to_string(), num_str: "1".to_string(), base_number: 1, eda_numbers: Vec::new() }, "通則".to_string()),
      LineContents::Caption("基本原則".to_string()),
      LineContents::Article(
        ArticleNumber { str: "第一条".to_string(), num_str: "1".to_string(), base_number: 1, eda_numbers: Vec::new() },
        "私権は、公共の福祉に適合しなければならない。".to_string()
      ),
      LineContents::Paragraph(
        ArticleNumber { str: "２".to_string(), num_str: "2".to_string(), base_number: 2, eda_numbers: Vec::new() },
        "権利の行使及び義務の履行は、信義に従い誠実に行わなければならない。".to_string()
      ),
      LineContents::Paragraph(ArticleNumber { str: "３".to_string(), num_str: "3".to_string(), base_number: 3, eda_numbers: Vec::new() }, "権利の濫用は、これを許さない。".to_string()),
      LineContents::Caption("解釈の基準".to_string()),
      LineContents::Article(
        ArticleNumber { str: "第二条の二".to_string(), num_str: "2_2".to_string(), base_number: 2, eda_numbers: vec![2] },
        "この法律は、個人の尊厳と両性の本質的平等を旨として、解釈しなければならない。".to_string()
      ),
      LineContents::Article(
        ArticleNumber { str: "第十三条".to_string(), num_str: "13".to_string(), base_number: 13, eda_numbers: Vec::new() },
        "被保佐人が次に掲げる行為をするには、その保佐人の同意を得なければならない。ただし、第九条ただし書に規定する行為については、この限りでない。".to_string()
      ),
      LineContents::Item(ItemNumber{pattern: NoParenKansuji, number:1, str: "一".to_string()}, "元本を領収し、又は利用すること。".to_string()),
      LineContents::Item(ItemNumber{pattern: NoParenKansuji, number:2, str: "二".to_string()}, "主たる債務者が法人である場合の次に掲げる者".to_string()),
      LineContents::Item(ItemNumber{pattern: NoParenIrohaKatakana, number:1, str: "イ".to_string()}, "主たる債務者の総株主の議決権（株主総会において決議をすることができる事項の全部につき議決権を行使することができない株式についての議決権を除く。以下この号において同じ。）の過半数を有する者".to_string()),
      LineContents::Item(ItemNumber{pattern: NoParenKansuji, number:3, str: "三".to_string()}, "不動産その他重要な財産に関する権利の得喪を目的とする行為をすること。".to_string()),
      LineContents::Paragraph(ArticleNumber { str: "２".to_string(), num_str: "2".to_string(), base_number: 2, eda_numbers: Vec::new() }, "家庭裁判所は、第十一条本文に規定する者又は保佐人若しくは保佐監督人の請求により、被保佐人が前項各号に掲げる行為以外の行為をする場合であってもその保佐人の同意を得なければならない旨の審判をすることができる。ただし、第九条ただし書に規定する行為については、この限りでない。".to_string()),
    ]
  )
}

#[test]
fn check_parse_body_1() {
  let s = r"第一編　総則
  第一章　通則
  （基本原則）
  第一条　私権は、公共の福祉に適合しなければならない。
  ２　権利の行使及び義務の履行は、信義に従い誠実に行わなければならない。
  ３　権利の濫用は、これを許さない。
  （解釈の基準）
  第二条　この法律は、個人の尊厳と両性の本質的平等を旨として、解釈しなければならない。
  第十三条　被保佐人が次に掲げる行為をするには、その保佐人の同意を得なければならない。ただし、第九条ただし書に規定する行為については、この限りでない。
  一　元本を領収し、又は利用すること。
  二　主たる債務者が法人である場合の次に掲げる者
    イ　主たる債務者の総株主の議決権（株主総会において決議をすることができる事項の全部につき議決権を行使することができない株式についての議決権を除く。以下この号において同じ。）の過半数を有する者
  三　不動産その他重要な財産に関する権利の得喪を目的とする行為をすること。
  ２　家庭裁判所は、第十一条本文に規定する者又は保佐人若しくは保佐監督人の請求により、被保佐人が前項各号に掲げる行為以外の行為をする場合であってもその保佐人の同意を得なければならない旨の審判をすることができる。ただし、第九条ただし書に規定する行為については、この限りでない。";
  let main_provision = parse_body("title", s).unwrap().main_provision.children;
  assert_eq!(
    main_provision,
    vec![law::MainProvisionContents::Part(article::Part {
      part_title: text::Text::from_value("総則"),
      children: vec![article::PartContents::Chapter(article::Chapter {
        chapter_title: text::Text::from_value("通則"),
        children: vec![
          article::ChapterContents::Article(article::Article {
            caption: Some(class::Caption {
              text: text::Text::from_value("基本原則"),
              common_caption: None
            }),
            title: text::Text::from_value(""),
            paragraph: vec![
              paragraph::Paragraph {
                caption: None,
                paragraph_num: text::Text::new(),
                amend_provision: Vec::new(),
                class: Vec::new(),
                sentence: vec![sentence::Sentence {
                  contents: vec![sentence::SentenceElement::String(
                    "私権は、公共の福祉に適合しなければならない。".to_string()
                  )],
                  num: Some(1),
                  function: None,
                  indent: None,
                  writing_mode: text::WritingMode::Vertical
                }],
                struct_list: Vec::new(),
                children: Vec::new(),
                num: 1,
                old_style: false,
                old_num: false,
                hide: false,
              },
              paragraph::Paragraph {
                caption: None,
                paragraph_num: text::Text::new(),
                amend_provision: Vec::new(),
                class: Vec::new(),
                sentence: vec![sentence::Sentence {
                  contents: vec![sentence::SentenceElement::String(
                    "権利の行使及び義務の履行は、信義に従い誠実に行わなければならない。"
                      .to_string()
                  )],
                  num: Some(1),
                  function: None,
                  indent: None,
                  writing_mode: text::WritingMode::Vertical
                }],
                struct_list: Vec::new(),
                children: Vec::new(),
                num: 2,
                old_style: false,
                old_num: false,
                hide: false,
              },
              paragraph::Paragraph {
                caption: None,
                paragraph_num: text::Text::new(),
                amend_provision: Vec::new(),
                class: Vec::new(),
                sentence: vec![sentence::Sentence {
                  contents: vec![sentence::SentenceElement::String(
                    "権利の濫用は、これを許さない。".to_string()
                  )],
                  num: Some(1),
                  function: None,
                  indent: None,
                  writing_mode: text::WritingMode::Vertical
                }],
                struct_list: Vec::new(),
                children: Vec::new(),
                num: 3,
                old_style: false,
                old_num: false,
                hide: false,
              }
            ],
            suppl_note: None,
            num: article_number::ArticleNumber{str: "第一条".to_string(),num_str: "1".to_string(), base_number: 1, eda_numbers: Vec::new()},
            delete: false,
            hide: false
          }),
          article::ChapterContents::Article(article::Article {
            caption: Some(class::Caption {
              text: text::Text::from_value("解釈の基準"),
              common_caption: None
            }),
            title: text::Text::from_value(""),
            paragraph: vec![paragraph::Paragraph {
              caption: None,
              paragraph_num: text::Text::new(),
              amend_provision: Vec::new(),
              class: Vec::new(),
              sentence: vec![sentence::Sentence {
                contents: vec![sentence::SentenceElement::String(
                  "この法律は、個人の尊厳と両性の本質的平等を旨として、解釈しなければならない。"
                    .to_string()
                )],
                num: Some(1),
                function: None,
                indent: None,
                writing_mode: text::WritingMode::Vertical
              }],
              struct_list: Vec::new(),
              children: Vec::new(),
              num: 1,
              old_style: false,
              old_num: false,
              hide: false,
            }],
            suppl_note: None,
            num: article_number::ArticleNumber{str: "第二条".to_string(),num_str: "2".to_string(), base_number: 2, eda_numbers: Vec::new()},
            delete: false,
            hide: false
          }),
          article::ChapterContents::Article(article::Article {
            caption: None,
            title: text::Text::from_value(""),
            paragraph: vec![paragraph::Paragraph {
              caption: None,
              paragraph_num: text::Text::new(),
              amend_provision: Vec::new(),
              class: Vec::new(),
              sentence: vec![sentence::Sentence {
                contents: vec![sentence::SentenceElement::String(
                  "被保佐人が次に掲げる行為をするには、その保佐人の同意を得なければならない。ただし、第九条ただし書に規定する行為については、この限りでない。"
                    .to_string()
                )],
                num: Some(1),
                function: None,
                indent: None,
                writing_mode: text::WritingMode::Vertical
              }],
              struct_list: Vec::new(),
              children: vec![
                paragraph::Item {
                  title: None,
                  sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "元本を領収し、又は利用すること。")]),
                  children: Vec::new(),
                  struct_list: Vec::new(),
                  num: "1".to_string(),
                  delete: false,
                  hide: false
                },
                paragraph::Item {
                  title: None,
                  sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "主たる債務者が法人である場合の次に掲げる者")]),
                  children: vec![
                    paragraph::Subitem1 {
                      title: None,
                      sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "主たる債務者の総株主の議決権（株主総会において決議をすることができる事項の全部につき議決権を行使することができない株式についての議決権を除く。以下この号において同じ。）の過半数を有する者")]),
                      children: Vec::new(),
                      struct_list: Vec::new(),
                      num: "1".to_string(),
                      delete: false,
                      hide: false
                    }
                  ],
                  struct_list: Vec::new(),
                  num: "2".to_string(),
                  delete: false,
                  hide: false
                },
                paragraph::Item {
                  title: None,
                  sentence: class::SentenceOrColumnOrTable::Sentence(vec![text_to_sentence(1, "不動産その他重要な財産に関する権利の得喪を目的とする行為をすること。")]),
                  children: Vec::new(),
                  struct_list: Vec::new(),
                  num: "3".to_string(),
                  delete: false,
                  hide: false
                }
              ],
              num: 1,
              old_style: false,
              old_num: false,
              hide: false,
            },paragraph::Paragraph {
              caption: None,
              paragraph_num: text::Text::new(),
              amend_provision: Vec::new(),
              class: Vec::new(),
              sentence: vec![sentence::Sentence {
                contents: vec![sentence::SentenceElement::String(
                  "家庭裁判所は、第十一条本文に規定する者又は保佐人若しくは保佐監督人の請求により、被保佐人が前項各号に掲げる行為以外の行為をする場合であってもその保佐人の同意を得なければならない旨の審判をすることができる。ただし、第九条ただし書に規定する行為については、この限りでない。"
                    .to_string()
                )],
                num: Some(1),
                function: None,
                indent: None,
                writing_mode: text::WritingMode::Vertical
              }],
              struct_list: Vec::new(),
              children: Vec::new(),
              num: 2,
              old_style: false,
              old_num: false,
              hide: false,
            }],
            suppl_note: None,
            num: article_number::ArticleNumber{str: "第十三条".to_string(),num_str: "13".to_string(), base_number: 13, eda_numbers: Vec::new()},
            delete: false,
            hide: false
          }),
        ],
        num: article_number::ArticleNumber{str: "第一章".to_string(),num_str: "1".to_string(), base_number: 1, eda_numbers: Vec::new()},
        delete: false,
        hide: false
      }),],
      num: article_number::ArticleNumber{str: "第一編".to_string(),num_str: "1".to_string(), base_number: 1, eda_numbers: Vec::new()},
      delete: false,
      hide: false
    })]
  )
}
