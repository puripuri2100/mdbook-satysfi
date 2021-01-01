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

pub fn parse_include_file(text: &str, file_path: &path::PathBuf) -> String {
  let text_type_list = parse_include_file_to_text_type_list(text);
  text_type_list
    .iter()
    .map(|text_type| text_type_to_string(text_type, file_path))
    .collect()
}

fn parse_include_file_to_text_type_list(text: &str) -> Vec<TextType> {
  let text_bits = text.as_bytes().to_vec();
  let re = Regex::new(
    r"\{\{#(?P<field_name_1>[a-zA-Z_-]+)\s+(?P<file_name_1>[^:\s]+)\}\}|\{\{#(?P<field_name_2>[a-zA-Z_-]+)\s+(?P<file_name_2>[^:\s]+):(?P<start_opt>[0-9]*):(?P<end_opt>[0-9]*)\}\}|\{\{#(?P<field_name_3>[a-zA-Z_-]+)\s+(?P<file_name_3>[^:\s]+):(?P<range_name>[\w_-]+)\}\}").unwrap();
  let match_range_list: Vec<(usize, usize)> = re
    .find_iter(text)
    .map(|mat| (mat.start(), mat.end()))
    .collect();
  let mut v = Vec::new();
  let mut pos = 0;
  for (start, end) in match_range_list.iter() {
    let mut text_v = Vec::new();
    for t in text_bits.iter().take(*start).skip(pos) {
      text_v.push(*t)
    }
    let text = String::from_utf8(text_v).unwrap();
    if pos != *start {
      v.push(TextType::Text(text))
    };
    pos = end + 1;
    let mut match_v = Vec::new();
    for t in text_bits.iter().take(*end).skip(*start) {
      match_v.push(*t)
    }
    let text = String::from_utf8(match_v).unwrap();
    let caps = re.captures(&text).unwrap();
    match (
      caps.name("field_name_1"),
      caps.name("field_name_2"),
      caps.name("field_name_3"),
    ) {
      (Some(field_name), _, _) => {
        let file_name = caps.name("file_name_1").unwrap().as_str();
        let path = path::PathBuf::from(file_name);
        let range = FileRange::Range(None, None);
        match field_name.as_str() {
          "include" | "playground" => v.push(TextType::Include(LinkType { path, range })),
          "rustdoc_include" => v.push(TextType::RustDocInclude(LinkType { path, range })),
          _ => v.push(TextType::Text(text.to_string())),
        }
      }
      (_, Some(field_name), _) => {
        let file_name = caps.name("file_name_2").unwrap().as_str();
        let path = path::PathBuf::from(file_name);
        let start_opt = caps
          .name("start_opt")
          .map(|s| s.as_str().parse().ok())
          .flatten();
        let end_opt = caps
          .name("end_opt")
          .map(|s| s.as_str().parse().ok())
          .flatten();
        let range = FileRange::Range(start_opt, end_opt);
        match field_name.as_str() {
          "include" | "playground" => v.push(TextType::Include(LinkType { path, range })),
          "rustdoc_include" => v.push(TextType::RustDocInclude(LinkType { path, range })),
          _ => v.push(TextType::Text(text.to_string())),
        }
      }
      (_, _, Some(field_name)) => {
        let file_name = caps.name("file_name_3").unwrap().as_str();
        let path = path::PathBuf::from(file_name);
        let range = match caps.name("range_name").unwrap().as_str().parse::<usize>() {
          Ok(u) => FileRange::Range(Some(u), None),
          Err(_) => FileRange::Name(caps.name("range_name").unwrap().as_str().to_string()),
        };
        match field_name.as_str() {
          "include" | "playground" => v.push(TextType::Include(LinkType { path, range })),
          "rustdoc_include" => v.push(TextType::RustDocInclude(LinkType { path, range })),
          _ => v.push(TextType::Text(text.to_string())),
        }
      }
      _ => unimplemented!(),
    }
  }
  if pos < text_bits.len() {
    let mut text_v = Vec::new();
    for t in text_bits.iter().skip(pos) {
      text_v.push(*t)
    }
    let text = String::from_utf8(text_v).unwrap();
    v.push(TextType::Text(text));
  }
  v
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
    })],
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
      TextType::Text("```".to_string()),
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

fn text_type_to_string(text_type: &TextType, file_path: &path::PathBuf) -> String {
  match text_type {
    TextType::Text(str) => format!("{}\n", str),
    TextType::Include(link_type) => {
      let path = file_path.parent().unwrap().join(link_type.clone().path);
      let text = fs::read_to_string(&path).unwrap();
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
      let path = file_path.parent().unwrap().join(link_type.clone().path);
      let text = fs::read_to_string(&path).unwrap();
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
  }
}

fn make_text_with_range_anchor(text: &str) -> Vec<StrWithAnchor> {
  let text_list = text.lines();
  text_list.map(|s| make_str_with_anchor(s)).collect()
}

fn make_str_with_anchor(s: &str) -> StrWithAnchor {
  let anchor_start_re = Regex::new(r".*ANCHOR:\s*(?P<start_name>[\w_-]+)[\W]*.*").unwrap();
  let anchor_end_re = Regex::new(r".*ANCHOR_END:\s*(?P<end_name>[\w_-]+)[\W]*.*").unwrap();
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
