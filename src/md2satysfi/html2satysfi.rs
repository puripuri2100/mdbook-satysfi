use html_parser::{Dom, Node};
use std::collections::hash_map::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Mode {
  Block,
  Inline,
  Code,
}

pub fn html_to_satysfi_code(html_code: &str) -> String {
  let node_lst = Dom::parse(html_code).unwrap().children;
  node_lst
    .iter()
    .map(|node| node_to_satysfi_code(node, Mode::Block))
    .collect()
}

fn node_to_satysfi_code(node: &Node, mode: Mode) -> String {
  match node {
    Node::Comment(_) => String::new(),
    Node::Text(str) => match mode {
      Mode::Inline => escape_inline_text(str),
      Mode::Block => format!("+p{{{}}}", escape_inline_text(str)),
      Mode::Code => make_code(false, str),
    },
    Node::Element(element) => {
      let tag_name = &element.name.to_lowercase();
      let attributes = &element.attributes;
      let children = &element.children;
      match tag_name.as_str() {
        "p" => tag_p_to_code(&attributes, &children, &mode),
        "code" => tag_code_to_code(&attributes, &children, &mode),
        "img" => tag_img_to_code(&attributes, &children, &mode),
        _ => {
          eprintln!(r#""{}" tag is not supported"#, tag_name);
          String::new()
        }
      }
    }
  }
}

#[test]
fn name() {
  assert_eq!(
    "".to_owned(),
    html_to_satysfi_code(
      r#"<img alt="Rendered documentation for the `art` crate that lists the `kinds` and `utils` modules" src="img/trpl14-03.png" class="center" />"#
    )
  )
}

fn tag_p_to_code(
  _attributes: &HashMap<String, Option<String>>,
  children: &Vec<Node>,
  mode: &Mode,
) -> String {
  match mode {
    Mode::Block => {
      format!(
        "+p{{{}}}",
        children
          .iter()
          .map(|node| node_to_satysfi_code(node, Mode::Inline))
          .collect::<String>()
      )
    }
    Mode::Inline => {
      format!(
        "\\p{{{}}}",
        children
          .iter()
          .map(|node| node_to_satysfi_code(node, Mode::Inline))
          .collect::<String>()
      )
    }
    Mode::Code => {
      format!(
        "<p>{}</p>",
        children
          .iter()
          .map(|node| node_to_satysfi_code(node, Mode::Code))
          .collect::<String>()
      )
    }
  }
}

fn tag_code_to_code(
  _attributes: &HashMap<String, Option<String>>,
  children: &Vec<Node>,
  mode: &Mode,
) -> String {
  match mode {
    Mode::Block => {
      format!(
        "+code({});",
        make_code(
          true,
          &children
            .iter()
            .map(|node| node_to_satysfi_code(node, Mode::Code))
            .collect::<String>()
        )
      )
    }
    Mode::Inline => {
      format!(
        "\\code({});",
        make_code(
          false,
          &children
            .iter()
            .map(|node| node_to_satysfi_code(node, Mode::Code))
            .collect::<String>()
        )
      )
    }
    Mode::Code => {
      format!(
        "<code>{}</code>",
        children
          .iter()
          .map(|node| node_to_satysfi_code(node, Mode::Code))
          .collect::<String>()
      )
    }
  }
}

fn tag_img_to_code(
  attributes: &HashMap<String, Option<String>>,
  _children: &Vec<Node>,
  mode: &Mode,
) -> String {
  let src = attributes.get("src").unwrap().clone().unwrap();
  match mode {
    Mode::Block => {
      format!("+img({});", make_code(true, &src))
    }
    Mode::Inline => {
      format!("\\img({});", make_code(true, &src))
    }
    Mode::Code => {
      format!("<img src=\"{}\"/>", src)
    }
  }
}

fn escape_inline_text(text: &str) -> String {
  text
    .replace("\\", "\\\\")
    .replace("{", "\\{")
    .replace("}", "\\}")
    .replace("<", "\\<")
    .replace(">", "\\>")
    .replace("%", "\\%")
    .replace("$", "\\$")
    .replace("#", "\\#")
    .replace(";", "\\;")
    .replace("|", "\\|")
    .replace("*", "\\*")
    .replace("@", "\\@")
}

fn make_code(is_block: bool, code: &str) -> String {
  let i = count_accent_in_inline_text(code);
  let raw = "`".repeat(i);
  if is_block {
    format!("{raw}\n{code}\n{raw}", code = code, raw = raw)
  } else {
    format!("{raw} {code} {raw}", code = code, raw = raw)
  }
}

fn count_accent_in_inline_text(text: &str) -> usize {
  let chars: Vec<char> = text.chars().collect();
  let mut count = 0;
  let mut n = 0;
  for c in chars.iter() {
    if c == &'`' {
      n += 1
    } else if n > count {
      count = n;
      n = 0;
    }
  }
  if n > count {
    count = n;
  }
  count
}
