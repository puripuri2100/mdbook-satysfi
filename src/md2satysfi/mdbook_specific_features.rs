use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path;

// parse mdBook-specific features
// https://rust-lang.github.io/mdBook/format/mdbook.html

#[derive(Debug, Clone, PartialEq, Eq)]
enum TextType {
  Include(LinkType),
  RustDocInclude(LinkType),
  Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LinkType {
  path: path::PathBuf,
  range: FileRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FileRange {
  Name(String),
  Range(Option<usize>, Option<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StrWithAnchor {
  AnchorStart(String),
  AnchorEnd(String),
  Str(String),
}

pub fn parse_include_file(text: &str, file_path: &path::PathBuf) -> Result<String> {
  let text_type_list = parse_include_file_to_text_type_list(text);
  text_type_list
    .iter()
    .map(|text_type| text_type_to_string(text_type, file_path))
    .collect()
}

fn parse_include_file_to_text_type_list(text: &str) -> Vec<TextType> {
  let chars = text.chars().collect::<Vec<_>>();
  let mut s = String::new();
  let mut v = Vec::new();
  let mut pos = 0;
  while pos < chars.len() {
    match chars[pos] {
      '\\' => match chars.get(pos + 1) {
        None => {
          s.push('\\');
          break;
        }
        Some('{') => {
          pos += 1;
          let opt = parse_text_type_opt(&chars, pos);
          match opt {
            None => {
              s.push('{');
              pos += 1;
            }
            Some((_, new_pos)) => {
              let str = chars.iter().take(new_pos).skip(pos).collect::<String>();
              pos = new_pos;
              s.push('\\');
              s.push_str(&str)
            }
          }
        }
        Some(_) => {
          s.push('\\');
          pos += 1
        }
      },
      '{' => {
        let opt = parse_text_type_opt(&chars, pos);
        match opt {
          None => {
            s.push('{');
            pos += 1;
          }
          Some((text_type, new_pos)) => {
            pos = new_pos;
            if !s.is_empty() {
              v.push(TextType::Text(s))
            }
            s = String::new();
            v.push(text_type);
          }
        }
      }
      c => {
        pos += 1;
        s.push(c)
      }
    }
  }
  if !s.is_empty() {
    v.push(TextType::Text(s))
  }
  v
}

#[allow(unused_assignments)]
fn parse_text_type_opt(chars: &[char], pos: usize) -> Option<(TextType, usize)> {
  let mut pos = pos;
  let mut kind = String::new();
  let mut file_name = String::new();
  let mut range_opt = None;
  // parse
  pos += 1;
  match chars.get(pos) {
    Some('{') => pos += 1,
    _ => return None,
  }
  match chars.get(pos) {
    Some('#') => pos += 1,
    _ => return None,
  }
  while pos < chars.len() {
    if chars[pos].is_ascii_alphabetic() || chars[pos] == '_' {
      kind.push(chars[pos]);
      pos += 1;
    } else {
      break;
    }
  }
  while pos < chars.len() {
    if chars[pos].is_ascii_whitespace() {
      pos += 1
    } else {
      break;
    }
  }
  while pos < chars.len() {
    if chars[pos] != ':' && chars[pos] != '}' {
      file_name.push(chars[pos]);
      pos += 1;
    } else {
      break;
    }
  }
  match chars.get(pos) {
    Some(':') => pos += 1,
    Some('}') => {
      pos += 1;
      match chars.get(pos) {
        Some('}') => pos += 1,
        _ => return None,
      }
      range_opt = Some(FileRange::Range(None, None));
      match (kind.as_str(), range_opt) {
        ("include", Some(range)) | ("playground", Some(range)) => {
          let path = path::PathBuf::from(file_name);
          return Some((TextType::Include(LinkType { path, range }), pos));
        }
        ("rustdoc_include", Some(range)) => {
          let path = path::PathBuf::from(file_name);
          return Some((TextType::RustDocInclude(LinkType { path, range }), pos));
        }
        _ => return None,
      }
    }
    _ => return None,
  }
  match chars.get(pos) {
    Some(':') => {
      pos += 1;
      let (end_range, new_pos) = parse_digit_range(chars, pos)?;
      pos = new_pos;
      range_opt = Some(FileRange::Range(None, Some(end_range)))
    }
    Some(c) if c.is_ascii_digit() => {
      let (start_range, new_pos) = parse_digit_range(chars, pos)?;
      pos = new_pos;
      match chars.get(pos) {
        Some(':') => pos += 1,
        Some('}') => range_opt = Some(FileRange::Range(Some(start_range), None)),
        _ => return None,
      }
      let end_range_opt = parse_digit_range(chars, pos);
      match end_range_opt {
        None => range_opt = Some(FileRange::Range(Some(start_range), None)),
        Some((end_range, new_pos)) => {
          pos = new_pos;
          range_opt = Some(FileRange::Range(Some(start_range), Some(end_range)))
        }
      }
    }
    Some(c) if c.is_ascii_alphabetic() || *c == '_' || *c == '-' => {
      let (range_name, new_pos) = parse_digit_name(chars, pos)?;
      pos = new_pos;
      range_opt = Some(FileRange::Name(range_name))
    }
    _ => return None,
  }
  match chars.get(pos) {
    Some('}') => pos += 1,
    _ => return None,
  }
  match chars.get(pos) {
    Some('}') => pos += 1,
    _ => return None,
  }
  // return value
  match (kind.as_str(), range_opt) {
    ("include", Some(range)) | ("playground", Some(range)) => {
      let path = path::PathBuf::from(file_name);
      Some((TextType::Include(LinkType { path, range }), pos))
    }
    ("rustdoc_include", Some(range)) => {
      let path = path::PathBuf::from(file_name);
      Some((TextType::RustDocInclude(LinkType { path, range }), pos))
    }
    _ => None,
  }
}

fn parse_digit_range(chars: &[char], pos: usize) -> Option<(usize, usize)> {
  let mut pos = pos;
  let mut u_str = String::new();
  while pos < chars.len() {
    if chars[pos].is_ascii_digit() {
      u_str.push(chars[pos]);
      pos += 1;
    } else {
      break;
    }
  }
  let u = u_str.parse().ok()?;
  Some((u, pos))
}

fn parse_digit_name(chars: &[char], pos: usize) -> Option<(String, usize)> {
  let mut pos = pos;
  let mut str = String::new();
  while pos < chars.len() {
    let c = chars[pos];
    if (c.is_ascii_alphabetic() || c == '_' || c == '-') && c != '}' {
      str.push(chars[pos]);
      pos += 1;
    } else {
      break;
    }
  }
  Some((str, pos))
}

#[test]
fn check_parse_include_file_1() {
  assert_eq!(
    vec![TextType::Text("aabb".to_string())],
    parse_include_file_to_text_type_list("aabb")
  )
}

#[test]
fn check_parse_include_file_2() {
  assert_eq!(
    vec![TextType::Include(LinkType {
      path: path::PathBuf::from("file.rs"),
      range: FileRange::Range(None, None),
    }),],
    parse_include_file_to_text_type_list("{{#include file.rs}}")
  )
}

#[test]
fn check_parse_include_file_3() {
  assert_eq!(
    vec![TextType::Include(LinkType {
      path: path::PathBuf::from("file.rs"),
      range: FileRange::Range(Some(2), None),
    }),],
    parse_include_file_to_text_type_list(r"{{#include file.rs:2}}")
  )
}

#[test]
fn check_parse_include_file_4() {
  assert_eq!(
    vec![TextType::Include(LinkType {
      path: path::PathBuf::from("file.rs"),
      range: FileRange::Range(None, Some(10)),
    }),],
    parse_include_file_to_text_type_list(r"{{#include file.rs::10}}")
  )
}

#[test]
fn check_parse_include_file_5() {
  assert_eq!(
    vec![TextType::Include(LinkType {
      path: path::PathBuf::from("file.rs"),
      range: FileRange::Range(Some(2), None),
    }),],
    parse_include_file_to_text_type_list(r"{{#include file.rs:2:}}")
  )
}

#[test]
fn check_parse_include_file_6() {
  assert_eq!(
    vec![TextType::Include(LinkType {
      path: path::PathBuf::from("file.rs"),
      range: FileRange::Range(Some(2), Some(10)),
    }),],
    parse_include_file_to_text_type_list(r"{{#include file.rs:2:10}}")
  )
}

#[test]
fn check_parse_include_file_7() {
  assert_eq!(
    vec![
      TextType::Text("Here is a component:\n```rust,no_run,noplayground\n".to_string()),
      TextType::Include(LinkType {
        path: path::PathBuf::from("file.rs"),
        range: FileRange::Name("component".to_string()),
      }),
      TextType::Text("\n```".to_string()),
    ],
    parse_include_file_to_text_type_list(
      r"Here is a component:
```rust,no_run,noplayground
{{#include file.rs:component}}
```"
    )
  )
}

#[test]
fn check_parse_include_file_7_2() {
  assert_eq!(
    vec![TextType::Include(LinkType {
      path: path::PathBuf::from("file.rs"),
      range: FileRange::Name("component".to_string()),
    }),],
    parse_include_file_to_text_type_list(r"{{#include file.rs:component}}")
  )
}

#[test]
fn check_parse_include_file_8() {
  assert_eq!(
    vec![TextType::RustDocInclude(LinkType {
      path: path::PathBuf::from("file.rs"),
      range: FileRange::Range(Some(2), Some(10)),
    }),],
    parse_include_file_to_text_type_list(r"{{#rustdoc_include file.rs:2:10}}")
  )
}

#[test]
fn check_parse_include_file_9() {
  assert_eq!(
    vec![TextType::RustDocInclude(LinkType {
      path: path::PathBuf::from("file.rs"),
      range: FileRange::Range(None, None),
    }),],
    parse_include_file_to_text_type_list(r"{{#rustdoc_include file.rs}}")
  )
}

#[test]
fn check_parse_include_file_10() {
  assert_eq!(
    vec![TextType::Text("Lorem Ipsum is simply dummy text of the printing and typesetting industry.\nLorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.\nIt has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged.\nIt was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.".to_string())],
    parse_include_file_to_text_type_list(
r"Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.
It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged.
It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.")
  )
}

#[test]
fn check_parse_include_file_11() {
  assert_eq!(
    vec![TextType::Text("\\{{#include file.rs}}".to_string())],
    parse_include_file_to_text_type_list("\\{{#include file.rs}}")
  )
}

#[test]
fn check_parse_include_file_11_2() {
  assert_eq!(
    vec![TextType::Text(
      r#"\{{#include file1.rs:2}}
\{{#include file2.rs::10}}
\{{#include file3.rs:2:}}
\{{#include file4.rs:2:10}}"#
        .to_string()
    )],
    parse_include_file_to_text_type_list(
      r#"\{{#include file1.rs:2}}
\{{#include file2.rs::10}}
\{{#include file3.rs:2:}}
\{{#include file4.rs:2:10}}"#
    )
  )
}

#[test]
fn check_parse_include_file_11_3() {
  assert_eq!(
    vec![TextType::Text(
      r#"```
\{{#include file.rs}}
```"#
        .to_string()
    )],
    parse_include_file_to_text_type_list(
      r#"```
\{{#include file.rs}}
```"#
    )
  )
}

#[test]
fn check_parse_include_file_12() {
  assert_eq!(
    vec![TextType::Text("\\hoge".to_string())],
    parse_include_file_to_text_type_list("\\hoge")
  )
}

fn text_type_to_string(text_type: &TextType, file_path: &path::PathBuf) -> Result<String> {
  let s = match text_type {
    TextType::Text(str) => str.to_string(),
    TextType::Include(link_type) => {
      let path = file_path
        .parent()
        .with_context(|| "Cannot parent file path")?
        .join(link_type.clone().path);
      let text =
        fs::read_to_string(&path).with_context(|| format!("Cannote read file: {:?}", path))?;
      let text_lines_len = text.lines().count();
      let text_lines = text.lines();
      let text_with_range_name = make_text_with_range_anchor(&text);
      let range = link_type.clone().range;
      match range {
        FileRange::Name(name) => {
          let mut s = String::new();
          let mut b = false;
          for str_with_anchor in text_with_range_name.iter() {
            match str_with_anchor {
              StrWithAnchor::AnchorStart(anchor_name) => {
                if anchor_name == &name {
                  b = true
                }
              }
              StrWithAnchor::AnchorEnd(anchor_name) => {
                if anchor_name == &name {
                  b = false
                }
              }
              StrWithAnchor::Str(str) => {
                if b {
                  s.push_str(&format!("{}\n", str))
                }
              }
            }
          }
          s
        }
        FileRange::Range(start_opt, end_opt) => {
          let start = start_opt.unwrap_or(1);
          let end = end_opt.unwrap_or(text_lines_len);
          text_lines
            .enumerate()
            .filter(|(i, _)| start <= i + 1 && i < &end)
            .map(|(_, s)| format!("{}\n", s))
            .collect()
        }
      }
    }
    TextType::RustDocInclude(link_type) => {
      let path = file_path
        .parent()
        .with_context(|| "Cannot parent file path")?
        .join(link_type.clone().path);
      let text =
        fs::read_to_string(&path).with_context(|| format!("Cannote read file: {:?}", path))?;
      let text_with_range_name = make_text_with_range_anchor(&text);
      let range = link_type.clone().range;
      match range {
        FileRange::Name(name) => {
          let mut s = String::new();
          let mut b = false;
          for str_with_anchor in text_with_range_name.iter() {
            match str_with_anchor {
              StrWithAnchor::AnchorStart(anchor_name) => {
                if anchor_name == &name {
                  b = true
                }
              }
              StrWithAnchor::AnchorEnd(anchor_name) => {
                if anchor_name == &name {
                  b = false
                }
              }
              StrWithAnchor::Str(str) => {
                if b {
                  s.push_str(&format!("{}\n", str))
                } else {
                  s.push_str(&format!("#{}\n", str))
                }
              }
            }
          }
          s
        }
        FileRange::Range(start_opt, end_opt) => {
          let len = text.lines().count();
          let start = start_opt.unwrap_or(1);
          let end = end_opt.unwrap_or(len);
          text
            .lines()
            .enumerate()
            .map(|(i, s)| {
              if start <= i + 1 && i < end {
                format!("{}\n", s)
              } else {
                format!("#{}\n", s)
              }
            })
            .collect()
        }
      }
    }
  };
  Ok(s)
}

fn make_text_with_range_anchor(text: &str) -> Vec<StrWithAnchor> {
  let text_list = text.lines();
  text_list.map(|s| make_str_with_anchor(s)).collect()
}

fn make_str_with_anchor(s: &str) -> StrWithAnchor {
  let anchor_start_re =
    Regex::new(r".*ANCHOR:\s*(?P<start_name>[\w_-]+)[\W]*.*").expect("This is safe regex");
  let anchor_end_re =
    Regex::new(r".*ANCHOR_END:\s*(?P<end_name>[\w_-]+)[\W]*.*").expect("This is safe regex");
  let is_start = anchor_start_re.is_match(s);
  let is_end = anchor_end_re.is_match(s);
  match (is_start, is_end) {
    (false, false) => StrWithAnchor::Str(s.to_string()),
    (true, _) => StrWithAnchor::AnchorStart(anchor_start_re.replace(s, "$start_name").to_string()),
    (_, true) => StrWithAnchor::AnchorEnd(anchor_end_re.replace(s, "$end_name").to_string()),
  }
}

#[test]
fn check_make_text_with_range_anchor() {
  assert_eq!(
    vec![
      StrWithAnchor::AnchorStart("all".to_string()),
      StrWithAnchor::Str(String::new()),
      StrWithAnchor::AnchorStart("component".to_string()),
      StrWithAnchor::Str(r"    struct Paddle {".to_string()),
      StrWithAnchor::Str(r"        hello: f32,".to_string()),
      StrWithAnchor::Str(r"    }".to_string()),
      StrWithAnchor::AnchorEnd("component".to_string()),
      StrWithAnchor::Str(String::new()),
      StrWithAnchor::AnchorStart("system".to_string()),
      StrWithAnchor::Str(r"    impl System for MySystem { ... }".to_string()),
      StrWithAnchor::AnchorEnd("system".to_string()),
      StrWithAnchor::Str(String::new()),
      StrWithAnchor::AnchorEnd("all".to_string()),
    ],
    make_text_with_range_anchor(
      r"/* ANCHOR: all */

    // ANCHOR: component
    struct Paddle {
        hello: f32,
    }
    // ANCHOR_END: component

    ////////// ANCHOR: system
    impl System for MySystem { ... }
    ////////// ANCHOR_END: system

    /* ANCHOR_END: all */"
    )
  )
}

pub fn hiding_code_lines(text: &str) -> String {
  let len = text.lines().count();
  text
    .lines()
    .enumerate()
    .map(|(i, text)| {
      if i == len - 1 {
        text.to_string()
      } else {
        format!("{}\n", text)
      }
    })
    .filter(|text| !matches!(text.chars().next(), Some('#')))
    .collect()
}

#[test]
fn check_hiding_code_lines() {
  assert_eq!(
    "hoge
piyo#piyo"
      .to_string(),
    hiding_code_lines(
      "hoge
#fuga
piyo#piyo"
    )
  );
}
