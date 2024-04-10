use crate::result::*;
use crate::*;
use kansuji::Kansuji;
use regex::Regex;

fn parse_part(
  n: usize,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Part {
  let part_title = text::Text::from_value(title);
  let num = n.to_string();
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
        let article = parse_article(caption_opt, *n, text, lines);
        children.push(article::PartContents::Article(article));
        caption_opt = None;
      }
      Some(LineContents::Chapter(n, title)) => {
        lines.next();
        let chap = parse_chapter(*n, title, lines);
        children.push(article::PartContents::Chapter(chap));
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Part {
    part_title,
    children,
    num,
    delete,
    hide,
  }
}

fn parse_chapter(
  n: usize,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Chapter {
  let chapter_title = text::Text::from_value(title);
  let num = n.to_string();
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
        let article = parse_article(caption_opt, *n, text, lines);
        children.push(article::ChapterContents::Article(article));
        caption_opt = None;
      }
      Some(LineContents::Section(n, title)) => {
        lines.next();
        let sec = parse_section(*n, title, lines);
        children.push(article::ChapterContents::Section(sec));
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Chapter {
    chapter_title,
    children,
    num,
    delete,
    hide,
  }
}

fn parse_section(
  n: usize,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Section {
  let section_title = text::Text::from_value(title);
  let num = n.to_string();
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
        let article = parse_article(caption_opt, *n, text, lines);
        children.push(article::SectionContents::Article(article));
        caption_opt = None;
      }
      Some(LineContents::Subsection(n, title)) => {
        lines.next();
        let subsec = parse_subsection(*n, title, lines);
        children.push(article::SectionContents::Subsection(subsec));
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Section {
    section_title,
    children,
    num,
    delete,
    hide,
  }
}

fn parse_subsection(
  n: usize,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Subsection {
  let subsection_title = text::Text::from_value(title);
  let num = n.to_string();
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
        let article = parse_article(caption_opt, *n, text, lines);
        children.push(article::SubsectionContents::Article(article));
        caption_opt = None;
      }
      Some(LineContents::Division(n, title)) => {
        lines.next();
        let div = parse_division(*n, title, lines);
        children.push(article::SubsectionContents::Division(div));
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Subsection {
    subsection_title,
    children,
    num,
    delete,
    hide,
  }
}

fn parse_division(
  n: usize,
  title: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Division {
  let division_title = text::Text::from_value(title);
  let num = n.to_string();
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
        let article = parse_article(caption_opt, *n, text, lines);
        children.push(article);
        caption_opt = None;
      }
      _ => break,
    }
  }
  article::Division {
    division_title,
    children,
    num,
    delete,
    hide,
  }
}

fn parse_article(
  caption_opt: Option<&String>,
  n: usize,
  text: &str,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> article::Article {
  let caption = caption_opt.map(|s| class::Caption {
    text: text::Text::from_value(s),
    common_caption: None,
  });
  let mut paragraph = vec![paragraph::Paragraph {
    caption: None,
    paragraph_num: text::Text::new(), // TODO 謎
    amend_provision: Vec::new(),      // TODO
    class: Vec::new(),                // TODO
    sentence: Vec::new(),             // TODO textをここに入れる
    struct_list: Vec::new(),          // TODO
    children: Vec::new(),             // TODO
    num: 1,
    old_style: false,
    old_num: false,
    hide: false,
  }];
  while let Some(LineContents::Paragraph(n, text)) = lines.peek() {
    lines.next();
    let para = parse_paragraph(*n, text, lines);
    paragraph.push(para)
  }
  let article = article::Article {
    caption,
    title: text::Text::from_value(String::new()), // TODO 「第○条」に生成
    paragraph,
    suppl_note: None,
    num: n.to_string(),
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
    .map(|(n, s)| text_to_sentence(n, s))
    .collect::<Vec<_>>();

  let mut children = Vec::new();
  while let Some(LineContents::Item(pattern, n, text)) = lines.peek() {
    lines.next();
    let mut sentence_text = vec![text];
    while let Some(LineContents::Text(s)) = lines.peek() {
      lines.next();
      sentence_text.push(s);
    }
    let mut subitem1 = Vec::new();
    if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
      if pat != pattern {
        subitem1 = parse_subitem1(pattern, pat, lines);
      }
    }
    let item = paragraph::Item {
      title: None,
      sentence: class::SentenceOrColumnOrTable::Sentence(
        sentence_text
          .iter()
          .enumerate()
          .map(|(n, s)| text_to_sentence(n, s))
          .collect(),
      ),
      children: subitem1,
      struct_list: Vec::new(),
      num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem1> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem2 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem2 = parse_subitem2(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem1 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem2,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem2> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem3 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem3 = parse_subitem3(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem2 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem3,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem3> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem4 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem4 = parse_subitem4(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem3 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem4,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem4> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem5 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem5 = parse_subitem5(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem4 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem5,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem5> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem6 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem6 = parse_subitem6(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem5 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem6,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem6> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem7 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem7 = parse_subitem7(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem6 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem7,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem7> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem8 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem8 = parse_subitem8(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem7 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem8,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem8> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem9 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem9 = parse_subitem9(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem8 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem9,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem9> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
      lines.next();
      let mut sentence_text = vec![text];
      while let Some(LineContents::Text(s)) = lines.peek() {
        lines.next();
        sentence_text.push(s);
      }
      let mut subitem10 = Vec::new();
      if let Some(LineContents::Item(pat, _, _)) = lines.peek() {
        if pat != now_pat {
          subitem10 = parse_subitem10(now_pat, pat, lines);
        }
      }
      v.push(paragraph::Subitem9 {
        title: None,
        sentence: class::SentenceOrColumnOrTable::Sentence(
          sentence_text
            .iter()
            .enumerate()
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        children: subitem10,
        struct_list: Vec::new(),
        num: n.to_string(),
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
  parent_pattern: &ItemPattern,
  now_pat: &ItemPattern,
  lines: &mut std::iter::Peekable<std::slice::Iter<LineContents>>,
) -> Vec<paragraph::Subitem10> {
  let mut v = Vec::new();
  while let Some(LineContents::Item(pat, n, text)) = lines.peek() {
    if pat == parent_pattern {
      break;
    } else if pat == now_pat {
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
            .map(|(n, s)| text_to_sentence(n, s))
            .collect(),
        ),
        struct_list: Vec::new(),
        num: n.to_string(),
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
    num,
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
        let part = parse_part(*n, text, &mut lines);
        main_provision_children.push(law::MainProvisionContents::Part(part));
        caption_opt = None;
        is_preamble = false;
      }
      LineContents::Chapter(n, text) => {
        lines.next();
        let chap = parse_chapter(*n, text, &mut lines);
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
        let sec = parse_section(*n, text, &mut lines);
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
        let article = parse_article(caption_opt, *n, text, &mut lines);
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
#[derive(Clone, Debug)]
enum LineContents {
  /// 見出し：（見出し）
  Caption(String),
  /// 編：第一編　タイトル
  Part(usize, String),
  /// 章：第一章　タイトル
  Chapter(usize, String),
  /// 節：第一節　タイトル
  Section(usize, String),
  /// 款：第一款　タイトル
  Subsection(usize, String),
  /// 目：第一目　タイトル
  Division(usize, String),
  /// 条：第二条 本文
  Article(usize, String),
  /// 項：２　本文
  Paragraph(usize, String),
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
  Item(ItemPattern, usize, String),
  /// 附則：附則（昭和三一年四月二日法律第六三号）
  SupplProvision(Option<String>),
  /// その他テキスト
  Text(String),
}

/// 号の数字を表す記号の種類
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemPattern {
  /// 括弧なし漢数字
  NoParenKansuji,
  /// 括弧なしイロハ
  NoParenIrohaKatakana,
  /// 括弧なしいろは
  NoParenIrohaHiragana,
  /// 括弧なし全角数字
  NoParenZenkakuNum,
  /// 括弧なしローマ数字大文字
  NoParenZenkakuRomanUpper,
  /// 括弧なしローマ数字小文字
  NoParenZenkakuRomanLower,
  /// 括弧なし大文字
  NoParenZenkakuUpper,
  /// 括弧なし小文字
  NoParenZenkakuLower,
  /// 括弧あり漢数字
  ParenKansuji,
  /// 括弧ありイロハ
  ParenIrohaKatakana,
  /// 括弧ありいろは
  ParenIrohaHiragana,
  /// 括弧あり全角数字
  ParenZenkakuNum,
  /// 括弧ありローマ数字大文字
  ParenZenkakuRomanUpper,
  /// 括弧ありローマ数字小文字
  ParenZenkakuRomanLower,
  /// 括弧あり大文字
  ParenZenkakuUpper,
  /// 括弧あり小文字
  ParenZenkakuLower,
}

fn parse_line_contents(line: &str) -> LineContents {
  use ItemPattern::*;
  use LineContents::*;
  let re_caption = Regex::new("^（(?<caption>[^）]+)）$").unwrap();
  // TODO 枝番号に対応させる
  let re_article = Regex::new(r"^第((?<arabic_num>[0-9]+)|(?<zenkaku_num>[０-９]+)|(?<kansuji>[一二三四五六七八九十百千]+))(?<suffix>(編|章|節|款|目|条))([　\s]*)(?<text>(.+))$").unwrap();
  let re_paragraph = Regex::new(r"^(?<num>[０-９]+)([　\s]*)(?<text>(.+))$").unwrap();
  let re_item = Regex::new(r"^(（((?<paren_iroha_katakana>[ア-ン]+)|(?<paren_iroha_hiragana>[あ-ん]+)|(?<paren_kansuji>[一二三四五六七八九十百千]+)|(?<paren_zenkaku_num>[０-９]+)|(?<paren_zenkaku_upper>[Ａ-Ｚ]+)|(?<paren_zenkaku_lower>[ａ-ｚ]+))）|((?<no_paren_iroha_katakana>[ア-ン]+)|(?<no_paren_iroha_hiragana>[あ-ん]+)|(?<no_paren_kansuji>[一二三四五六七八九十百千]+)|(?<no_paren_zenkaku_num>[０-９]+)|(?<no_paren_zenkaku_upper>[Ａ-Ｚ]+)|(?<no_paren_zenkaku_lower>[ａ-ｚ]+)))([　\s]*)(?<text>(.+))$").unwrap();
  let re_is_roman = Regex::new(r"[ixvlcIXVLCｉｘｖｌｃＩＸＶＬＣ]+").unwrap();
  let re_suppl_provision =
    Regex::new(r"^附([　\s]*)則([　\s]*)(（(?<law_num>.+)）)?[^（）]*$").unwrap();
  if let Some(caps) = re_caption.captures(line) {
    Caption(caps["caption"].to_string())
  } else if let Some(caps) = re_article.captures(line) {
    let num = if let Some(arabic_num) = caps.name("arabic_num") {
      arabic_num.as_str().parse::<usize>().unwrap()
    } else if let Some(zenkaku_num) = caps.name("zenkaku_num") {
      parse_zenkaku_num(zenkaku_num.as_str())
    } else if let Some(kansuji) = caps.name("kansuji") {
      let kansuji = Kansuji::try_from(kansuji.as_str()).unwrap();
      let n: u128 = kansuji.into();
      n as usize
    } else {
      unreachable!()
    };
    let text = caps["text"].to_string();
    let suffix = &caps["suffix"];
    if suffix == "編" {
      Part(num, text)
    } else if suffix == "章" {
      Chapter(num, text)
    } else if suffix == "節" {
      Section(num, text)
    } else if suffix == "款" {
      Subsection(num, text)
    } else if suffix == "目" {
      Division(num, text)
    } else if suffix == "条" {
      Article(num, text)
    } else {
      unreachable!()
    }
  } else if let Some(caps) = re_paragraph.captures(line) {
    let num = parse_zenkaku_num(&caps["num"]);
    let text = caps["text"].to_string();
    Paragraph(num, text.to_string())
  } else if let Some(caps) = re_item.captures(line) {
    let (item_pattern, num) = if let Some(s) = caps.name("paren_iroha_katakana") {
      (ParenIrohaKatakana, parse_iroha_katakana(s.as_str()))
    } else if let Some(s) = caps.name("paren_iroha_hiragana") {
      (ParenIrohaHiragana, parse_iroha_hiragana(s.as_str()))
    } else if let Some(s) = caps.name("paren_kansuji") {
      let kansuji = Kansuji::try_from(s.as_str()).unwrap();
      let n: u128 = kansuji.into();
      (ParenKansuji, n as usize)
    } else if let Some(s) = caps.name("paren_zenkaku_num") {
      (ParenZenkakuNum, parse_zenkaku_num(s.as_str()))
    } else if let Some(s) = caps.name("paren_zenkaku_upper") {
      if re_is_roman.is_match(s.as_str()) {
        (ParenZenkakuRomanUpper, parse_roman(s.as_str()))
      } else {
        (ParenZenkakuUpper, parse_zenkaku_alphabet(s.as_str()))
      }
    } else if let Some(s) = caps.name("paren_zenkaku_lower") {
      if re_is_roman.is_match(s.as_str()) {
        (ParenZenkakuRomanLower, parse_roman(s.as_str()))
      } else {
        (ParenZenkakuLower, parse_zenkaku_alphabet(s.as_str()))
      }
    }
    // 括弧なし
    else if let Some(s) = caps.name("no_paren_iroha_katakana") {
      (NoParenIrohaKatakana, parse_iroha_katakana(s.as_str()))
    } else if let Some(s) = caps.name("no_paren_iroha_hiragana") {
      (NoParenIrohaHiragana, parse_iroha_hiragana(s.as_str()))
    } else if let Some(s) = caps.name("no_paren_kansuji") {
      let kansuji = Kansuji::try_from(s.as_str()).unwrap();
      let n: u128 = kansuji.into();
      (NoParenKansuji, n as usize)
    } else if let Some(s) = caps.name("no_paren_zenkaku_num") {
      (NoParenZenkakuNum, parse_zenkaku_num(s.as_str()))
    } else if let Some(s) = caps.name("no_paren_zenkaku_upper") {
      if re_is_roman.is_match(s.as_str()) {
        (NoParenZenkakuRomanUpper, parse_roman(s.as_str()))
      } else {
        (NoParenZenkakuUpper, parse_zenkaku_alphabet(s.as_str()))
      }
    } else if let Some(s) = caps.name("no_paren_zenkaku_lower") {
      if re_is_roman.is_match(s.as_str()) {
        (NoParenZenkakuRomanLower, parse_roman(s.as_str()))
      } else {
        (NoParenZenkakuLower, parse_zenkaku_alphabet(s.as_str()))
      }
    } else {
      unreachable!()
    };
    let text = caps["text"].to_string();
    Item(item_pattern, num, text)
  } else if let Some(caps) = re_suppl_provision.captures(line) {
    let law_num = caps.name("law_num").map(|m| m.as_str().to_string());
    SupplProvision(law_num)
  } else {
    Text(line.to_string())
  }
}

fn parse_zenkaku_num(str: &str) -> usize {
  str
    .replace('０', "0")
    .replace('１', "1")
    .replace('２', "2")
    .replace('３', "3")
    .replace('４', "4")
    .replace('５', "5")
    .replace('６', "6")
    .replace('７', "7")
    .replace('８', "8")
    .replace('９', "9")
    .parse::<usize>()
    .unwrap()
}

fn parse_iroha_katakana(str: &str) -> usize {
  match str {
    "イ" => 1,
    "ロ" => 2,
    "ハ" => 3,
    "ニ" => 4,
    "ホ" => 5,
    "ヘ" => 6,
    "ト" => 7,
    "チ" => 8,
    "リ" => 9,
    "ヌ" => 10,
    "ル" => 11,
    "ヲ" => 12,
    "ワ" => 13,
    "カ" => 14,
    "ヨ" => 15,
    "タ" => 16,
    "レ" => 17,
    "ソ" => 18,
    "ツ" => 19,
    "ネ" => 20,
    "ナ" => 21,
    "ラ" => 22,
    "ム" => 23,
    "ウ" => 24,
    "ヰ" => 25,
    "ノ" => 26,
    "オ" => 27,
    "ク" => 28,
    "ヤ" => 29,
    "マ" => 30,
    "ケ" => 31,
    "フ" => 32,
    "コ" => 33,
    "エ" => 34,
    "テ" => 35,
    "ア" => 36,
    "サ" => 37,
    "キ" => 38,
    "ユ" => 39,
    "メ" => 40,
    "ミ" => 41,
    "シ" => 42,
    "ヱ" => 43,
    "ヒ" => 44,
    "モ" => 45,
    "セ" => 46,
    "ス" => 47,
    _ => unreachable!(),
  }
}

fn parse_iroha_hiragana(str: &str) -> usize {
  match str {
    "い" => 1,
    "ろ" => 2,
    "は" => 3,
    "に" => 4,
    "ほ" => 5,
    "へ" => 6,
    "と" => 7,
    "ち" => 8,
    "り" => 9,
    "ぬ" => 10,
    "る" => 11,
    "を" => 12,
    "わ" => 13,
    "か" => 14,
    "よ" => 15,
    "た" => 16,
    "れ" => 17,
    "そ" => 18,
    "つ" => 19,
    "ね" => 20,
    "な" => 21,
    "ら" => 22,
    "む" => 23,
    "う" => 24,
    "ゐ" => 25,
    "の" => 26,
    "お" => 27,
    "く" => 28,
    "や" => 29,
    "ま" => 30,
    "け" => 31,
    "ふ" => 32,
    "こ" => 33,
    "え" => 34,
    "て" => 35,
    "あ" => 36,
    "さ" => 37,
    "き" => 38,
    "ゆ" => 39,
    "め" => 40,
    "み" => 41,
    "し" => 42,
    "ゑ" => 43,
    "ひ" => 44,
    "も" => 45,
    "せ" => 46,
    "す" => 47,
    _ => unreachable!(),
  }
}

fn parse_roman(str: &str) -> usize {
  roman::from(
    &str
      .replace('i', "I")
      .replace('x', "X")
      .replace('l', "L")
      .replace('c', "C")
      .replace('ｉ', "I")
      .replace('ｘ', "X")
      .replace('ｌ', "L")
      .replace('ｃ', "C")
      .replace('Ｉ', "I")
      .replace('Ｘ', "X")
      .replace('Ｌ', "L")
      .replace('Ｃ', "C"),
  )
  .unwrap() as usize
}

fn parse_zenkaku_alphabet(str: &str) -> usize {
  let s = str
    .chars()
    .map(|c| {
      let v = c as u32;
      let n =
    // 大文字
    if v < 0xFF41 {
      v - 0xFF20
    } else {
      v - 0xFF40
    };
      n.to_string()
    })
    .collect::<String>();
  s.parse::<usize>().unwrap()
}

#[test]
fn check_parse_zenkaku_alphabet_lower() {
  assert_eq!(parse_zenkaku_alphabet("ｂ"), 2)
}
#[test]
fn check_parse_zenkaku_alphabet_upper() {
  assert_eq!(parse_zenkaku_alphabet("Ｂ"), 2)
}
