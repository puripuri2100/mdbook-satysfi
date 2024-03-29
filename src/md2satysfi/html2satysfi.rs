use super::mdbook_specific_features;
use anyhow::{Context, Result};
use html_parser::{Dom, Node};
use std::path::Path;
use toml::map;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Mode {
  Block,
  Inline,
  Code,
}

pub fn html_to_satysfi_code(
  html_code: &str,
  mode: Mode,
  file_path: &Path,
  html_cfg: &map::Map<String, toml::Value>,
  ch_file_path: &Path,
) -> Result<String> {
  let html = Dom::parse(html_code).with_context(|| "Missing Parse HTML tag")?;
  let node_lst = html.children;
  let s = node_lst
    .iter()
    .map(|node| node_to_satysfi_code(node, mode, file_path, html_cfg, ch_file_path, 2))
    .collect();
  s
}

fn node_to_satysfi_code(
  node: &Node,
  mode: Mode,
  file_path: &Path,
  html_cfg: &map::Map<String, toml::Value>,
  ch_file_path: &Path,
  indent: usize,
) -> Result<String> {
  match node {
    Node::Comment(_) => Ok(String::new()),
    Node::Text(str) => match mode {
      Mode::Inline => Ok(escape_inline_text(
        &mdbook_specific_features::parse_include_file(str, file_path)?,
      )),
      Mode::Block => Ok(format!(
        "+p{{{}}}",
        escape_inline_text(&mdbook_specific_features::parse_include_file(
          str, file_path
        )?)
      )),
      Mode::Code => Ok(str.to_string()),
    },
    Node::Element(element) => {
      let tag_name =
        if element.name.to_lowercase() == *"div" || element.name.to_lowercase() == *"span" {
          element
            .classes
            .get(0)
            .unwrap_or(&element.name.to_lowercase())
            .clone()
        } else {
          element.name.to_lowercase()
        };
      let tag_name = &tag_name;
      let children = &element.children;
      let attributes = &element.attributes;
      let default_cfg = make_default_config();
      let toml_data_opt = match html_cfg.get(tag_name) {
        Some(data) => Some(data),
        None => default_cfg.get(tag_name),
      };
      match toml_data_opt {
        Some(toml_data) => {
          let command_name = toml_data
            .get("command_name")
            .and_then(|v| v.as_str())
            .unwrap_or(tag_name);
          let children_type_opt = toml_data
            .get("children_type")
            .and_then(|v| v.as_str().and_then(parse_children_type));
          let attribute_cfg_lst = match toml_data.get("attribute").and_then(|v| v.as_array()) {
            Some(v) => v.clone(),
            None => Vec::new(),
          };
          let children_str = match children_type_opt {
            None => ";".to_string(),
            Some(children_type) => match children_type {
              ChildrenType::BlockText => {
                let children_str = children
                  .iter()
                  .map(|node| match mode {
                    Mode::Code => node_to_satysfi_code(
                      node,
                      Mode::Code,
                      file_path,
                      html_cfg,
                      ch_file_path,
                      indent + 1,
                    ),
                    _ => node_to_satysfi_code(
                      node,
                      Mode::Block,
                      file_path,
                      html_cfg,
                      ch_file_path,
                      indent + 1,
                    ),
                  })
                  .collect::<Result<String>>()?;
                match mode {
                  Mode::Code => children_str,
                  _ => {
                    let indent_str = "  ".repeat(indent);
                    format!("<{children_str}\n{indent_str}>")
                  }
                }
              }
              ChildrenType::InlineText => {
                let children_str = children
                  .iter()
                  .map(|node| match mode {
                    Mode::Code => node_to_satysfi_code(
                      node,
                      Mode::Code,
                      file_path,
                      html_cfg,
                      ch_file_path,
                      indent + 1,
                    ),
                    _ => node_to_satysfi_code(
                      node,
                      Mode::Inline,
                      file_path,
                      html_cfg,
                      ch_file_path,
                      indent + 1,
                    ),
                  })
                  .collect::<Result<String>>()?;
                match mode {
                  Mode::Code => children_str,
                  _ => format!("{{{children_str}}}"),
                }
              }
              ChildrenType::BlockCode => {
                let children_str = children
                  .iter()
                  .map(|node| {
                    node_to_satysfi_code(
                      node,
                      Mode::Code,
                      file_path,
                      html_cfg,
                      ch_file_path,
                      indent + 1,
                    )
                  })
                  .collect::<Result<String>>()?;
                match mode {
                  Mode::Code => children_str,
                  _ => format!("({});", make_code(true, &escape_code(&children_str))),
                }
              }
              ChildrenType::InlineCode => {
                let children_str = children
                  .iter()
                  .map(|node| {
                    node_to_satysfi_code(
                      node,
                      Mode::Code,
                      file_path,
                      html_cfg,
                      ch_file_path,
                      indent + 1,
                    )
                  })
                  .collect::<Result<String>>()?;
                match mode {
                  Mode::Code => children_str,
                  _ => format!("({});", &make_code(false, &escape_code(&children_str))),
                }
              }
              ChildrenType::BlockList => {
                let children_iter = children.iter().map(|node| match mode {
                  Mode::Code => node_to_satysfi_code(
                    node,
                    Mode::Code,
                    file_path,
                    html_cfg,
                    ch_file_path,
                    indent + 1,
                  ),
                  _ => node_to_satysfi_code(
                    node,
                    Mode::Block,
                    file_path,
                    html_cfg,
                    ch_file_path,
                    indent + 1,
                  ),
                });
                let indent_str = "  ".repeat(indent + 1);
                let mut children_str = String::new();
                for children_value in children_iter {
                  children_str.push_str(&format!("\n{indent_str}('<{}>);", children_value?))
                }
                match mode {
                  Mode::Code => children_str,
                  _ => {
                    let indent_str = "  ".repeat(indent);
                    format!("[{children_str}\n{indent_str}];")
                  }
                }
              }
              ChildrenType::InlineList => {
                let children_iter = children.iter().map(|node| match mode {
                  Mode::Code => node_to_satysfi_code(
                    node,
                    Mode::Code,
                    file_path,
                    html_cfg,
                    ch_file_path,
                    indent + 1,
                  ),
                  _ => node_to_satysfi_code(
                    node,
                    Mode::Inline,
                    file_path,
                    html_cfg,
                    ch_file_path,
                    indent + 1,
                  ),
                });
                let indent_str = "  ".repeat(indent + 1);
                let mut children_str = String::new();
                for children_value in children_iter {
                  children_str.push_str(&format!("\n{indent_str}({{{}}});", children_value?))
                }
                match mode {
                  Mode::Code => children_str,
                  _ => {
                    let indent_str = "  ".repeat(indent);
                    format!("[{children_str}\n{indent_str}];")
                  }
                }
              }
            },
          };
          let attributes_str_opt = attribute_cfg_lst
            .iter()
            .map(|toml| {
              let attribute_name_opt = toml.get("name").and_then(|v| v.as_str());
              let attribute_type_opt = toml
                .get("type")
                .and_then(|v| v.as_str().and_then(parse_attribute_type));
              match (attribute_name_opt, attribute_type_opt) {
                (Some(attribute_name), Some(attribute_type)) => match mode {
                  Mode::Code => {
                    let attribute_value_opt = attributes.get(attribute_name).unwrap_or(&None);
                    match attribute_type {
                      AttributeTypeOrOption::Option(_) => match attribute_value_opt {
                        None => Some(String::new()),
                        Some(attribute_value) => {
                          Some(format!(r#" {attribute_name}="{attribute_value}""#))
                        }
                      },
                      AttributeTypeOrOption::Normal(_) => attribute_value_opt
                        .clone()
                        .map(|attribute_value| format!(r#" {attribute_name}="{attribute_value}""#)),
                    }
                  }
                  _ => {
                    let attribute_value_opt = attributes.get(attribute_name).unwrap_or(&None);
                    match attribute_type {
                      AttributeTypeOrOption::Option(attribute_type) => match attribute_value_opt {
                        None => Some("(None)".to_string()),
                        Some(attribute_value) => match attribute_type {
                          AttributeType::BlockText => {
                            Some(format!(r#"(Some('<{attribute_value}>))"#))
                          }
                          AttributeType::InlineText => {
                            Some(format!(r#"(Some({{{attribute_value}}}))"#))
                          }
                          AttributeType::String => {
                            Some(format!(r#"(Some({}))"#, make_code(false, attribute_value)))
                          }
                          AttributeType::Int => attribute_value
                            .parse::<isize>()
                            .ok()
                            .map(|isize| format!(r#"(Some({isize}))"#)),
                          AttributeType::Bool => attribute_value
                            .parse::<bool>()
                            .ok()
                            .map(|bool| format!(r#"(Some({bool}))"#)),
                          AttributeType::Link => ch_file_path
                            .parent()
                            .and_then(|path| path.to_str())
                            .map(|path| {
                              format!(
                                r#"(Some({}))"#,
                                make_code(false, &format!("{path}/{attribute_value}"))
                              )
                            }),
                        },
                      },
                      AttributeTypeOrOption::Normal(attribute_type) => match attribute_value_opt {
                        None => {
                          eprintln!(r#"Not enough attributes required: "{tag_name}""#);
                          None
                        }
                        Some(attribute_value) => match attribute_type {
                          AttributeType::BlockText => Some(format!(r#"('<{attribute_value}>)"#)),
                          AttributeType::InlineText => Some(format!(r#"({{{attribute_value}}})"#)),
                          AttributeType::String => {
                            Some(format!(r#"({})"#, make_code(false, attribute_value)))
                          }
                          AttributeType::Int => attribute_value
                            .parse::<isize>()
                            .ok()
                            .map(|isize| format!(r#"({isize})"#)),
                          AttributeType::Bool => attribute_value
                            .parse::<bool>()
                            .ok()
                            .map(|bool| format!(r#"({bool})"#)),
                          AttributeType::Link => ch_file_path
                            .parent()
                            .and_then(|path| path.to_str())
                            .map(|path| {
                              if tag_name == "img" {
                                String::new()
                              } else {
                                format!(
                                  r#"({})"#,
                                  make_code(false, &format!("{path}/{attribute_value}"))
                                )
                              }
                            }),
                        },
                      },
                    }
                  }
                },
                _ => Some(String::new()),
              }
            })
            .collect::<Option<String>>();
          match mode {
            Mode::Block => {
              let s = if let Some(attributes_str) = attributes_str_opt {
                let indent_str = "  ".repeat(indent);
                format!("\n{indent_str}+{command_name}{attributes_str}{children_str}")
              } else {
                String::new()
              };
              Ok(s)
            }
            Mode::Inline => {
              let s = if let Some(attributes_str) = attributes_str_opt {
                format!(" \\{command_name}{attributes_str}{children_str} ")
              } else {
                String::new()
              };
              Ok(s)
            }
            Mode::Code => {
              let s = if let Some(attributes_str) = attributes_str_opt {
                format!("<{tag_name}{attributes_str}>{children_str}</{tag_name}>")
              } else {
                String::new()
              };
              Ok(s)
            }
          }
        }
        None => {
          eprintln!(r#""{tag_name}" tag is not supported"#);
          Ok(String::new())
        }
      }
    }
  }
}

#[test]
fn check_html_to_satysfi_code_1() {
  assert_eq!(
    "\n    +p{this is a image. \\code(`` let p = `<p>x</p>` ``); }".to_owned(),
    html_to_satysfi_code(
      r#"<p> this is a image. <code>let p = `<p>x</p>`</code></p>"#,
      Mode::Block,
      &Path::new("ch1/hoge.md"),
      &map::Map::new(),
      &Path::new("ch1/hoge.md"),
    )
    .unwrap_or_default()
  )
}

#[test]
fn check_html_to_satysfi_code_2() {
  assert_eq!(
    r#" \p{this is a image. \code(`` let p = `<p>x</p>` ``); } "#.to_owned(),
    html_to_satysfi_code(
      r#"<p> this is a image.<code>let p = `<p>x</p>`</code></p>"#,
      Mode::Inline,
      &Path::new("ch1/hoge.md"),
      &map::Map::new(),
      &Path::new("ch1/hoge.md")
    )
    .unwrap_or_default()
  )
}

#[test]
fn check_html_to_satysfi_code_3() {
  assert_eq!(
    r#" \ruby{如何 \rp{(}  \rt{いか}  \rp{)} } "#.to_string(),
    html_to_satysfi_code(
      r#"<ruby>如何<rp>(</rp><rt>いか</rt><rp>)</rp></ruby>"#,
      Mode::Inline,
      &Path::new("ch1/hoge.md"),
      &map::Map::new(),
      &Path::new("ch1/hoge.md")
    )
    .unwrap_or_default()
  )
}

#[test]
fn check_html_to_satysfi_code_4() {
  assert_eq!(
    "\n    +code-block(None)(None)(`\n\\{{#include file.rs}}\n`);".to_string(),
    html_to_satysfi_code(
      r#"<div class="code-block">\{{#include file.rs}}</div>"#,
      Mode::Block,
      &Path::new("ch1/hoge.md"),
      &map::Map::new(),
      &Path::new("ch1/hoge.md")
    )
    .unwrap_or_default()
  )
}
#[test]
fn check_html_to_satysfi_code_4_2() {
  assert_eq!(
    "\n    +code-block(Some(` satysfi `))(None)(`\n\\{{#include file.rs}}\n`);".to_string(),
    html_to_satysfi_code(
      r#"<div class="code-block" lang="satysfi">\{{#include file.rs}}</div>"#,
      Mode::Block,
      &Path::new("ch1/hoge.md"),
      &map::Map::new(),
      &Path::new("ch1/hoge.md")
    )
    .unwrap_or_default()
  )
}

#[test]
fn check_break_html_to_satysfi_code() {
  assert_eq!(
    "".to_string(),
    html_to_satysfi_code(
      r#"<img src="path"/>"#,
      Mode::Block,
      &Path::new("ch1/hoge.md"),
      &map::Map::new(),
      &Path::new("ch1/hoge.md")
    )
    .unwrap_or_default()
  )
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum ChildrenType {
  BlockText,
  InlineText,
  BlockCode,
  InlineCode,
  BlockList,
  InlineList,
}

fn parse_children_type(type_str: &str) -> Option<ChildrenType> {
  match type_str.to_string().to_lowercase().as_str() {
    "inline" => Some(ChildrenType::InlineText),
    "block" => Some(ChildrenType::BlockText),
    "inline-text" => Some(ChildrenType::InlineText),
    "block-text" => Some(ChildrenType::BlockText),
    "inline list" => Some(ChildrenType::InlineList),
    "block list" => Some(ChildrenType::BlockList),
    "inline-text list" => Some(ChildrenType::InlineList),
    "block-text list" => Some(ChildrenType::BlockList),
    "inline code" => Some(ChildrenType::InlineCode),
    "block code" => Some(ChildrenType::BlockCode),
    _ => None,
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum AttributeType {
  InlineText,
  BlockText,
  Link,
  String,
  Int,
  Bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum AttributeTypeOrOption {
  Option(AttributeType),
  Normal(AttributeType),
}

fn parse_attribute_type(type_str: &str) -> Option<AttributeTypeOrOption> {
  match type_str.to_string().to_lowercase().as_str() {
    "inline" => Some(AttributeTypeOrOption::Normal(AttributeType::InlineText)),
    "inline option" => Some(AttributeTypeOrOption::Option(AttributeType::InlineText)),
    "inline-text" => Some(AttributeTypeOrOption::Normal(AttributeType::InlineText)),
    "inline-text option" => Some(AttributeTypeOrOption::Option(AttributeType::InlineText)),
    "block" => Some(AttributeTypeOrOption::Normal(AttributeType::BlockText)),
    "block option" => Some(AttributeTypeOrOption::Option(AttributeType::BlockText)),
    "block-text" => Some(AttributeTypeOrOption::Normal(AttributeType::BlockText)),
    "block-text option" => Some(AttributeTypeOrOption::Option(AttributeType::BlockText)),
    "link" => Some(AttributeTypeOrOption::Normal(AttributeType::Link)),
    "link option" => Some(AttributeTypeOrOption::Option(AttributeType::Link)),
    "string" => Some(AttributeTypeOrOption::Normal(AttributeType::String)),
    "string option" => Some(AttributeTypeOrOption::Option(AttributeType::String)),
    "int" => Some(AttributeTypeOrOption::Normal(AttributeType::Int)),
    "int option" => Some(AttributeTypeOrOption::Option(AttributeType::Int)),
    "bool" => Some(AttributeTypeOrOption::Normal(AttributeType::Bool)),
    "bool option" => Some(AttributeTypeOrOption::Option(AttributeType::Bool)),
    _ => None,
  }
}

const DEFAULT_HTML_CONFIG: &str = r#"
[p]
  command_name="p"
  children_type="inline"
[code]
  command_name="code"
  children_type="inline code"
[blockquote]
  command_name="block-quote"
  children_type="block"
[em]
  command_name="emph"
  children_type="inline"
[strong]
  command_name="strong"
  children_type="inline"
[hr]
  command_name="rule"
[ul]
  command_name="listing"
  children_type="inline list"
[ol]
  command_name="enumerate"
  children_type="inline list"
  [[ol.attribute]]
  "name" = "start"
  "type" = "int"
[li]
  command_name="item"
  children_type="inline"
[h1]
  command_name="heading(1)"
  children_type="inline"
[h2]
  command_name="heading(2)"
  children_type="inline"
[h3]
  command_name="heading(3)"
  children_type="inline"
[h4]
  command_name="heading(4)"
  children_type="inline"
[h5]
  command_name="heading(5)"
  children_type="inline"
[h6]
  command_name="heading(6)"
  children_type="inline"
[code-block]
  command_name="code-block"
  children_type="block code"
  [[code-block.attribute]]
  "name" = "lang"
  "type" = "string option"
  [[code-block.attribute]]
  "name" = "theme"
  "type" = "string option"
[a]
  command_name="href"
  children_type="inline"
  [[a.attribute]]
  "name" = "href"
  "type" = "string"
[task-list-marker]
  command_name="task-list-marker"
  [[task-list-marker.attribute]]
  "name" = "checked"
  "type" = "bool"
[footnote-reference]
  command_name="footnote"
  [[footnote-reference.attribute]]
  "name" = "tag"
  "type" = "string"
[footnote-definition]
  command_name="add-footnote"
  children_type="inline"
  [[footnote-definition.attribute]]
  "name" = "tag"
  "type" = "string"
[ruby]
  command_name="ruby"
  children_type="inline"
[rp]
  command_name="rp"
  children_type="inline"
[rt]
  command_name="rt"
  children_type="inline"
[sup]
  command_name="sup"
  children_type="inline"
[sub]
  command_name="sub"
  children_type="inline"
[small]
  command_name="small"
  children_type="inline"
[s]
  command_name="stroke"
  children_type="inline"
[cite]
  command_name="cite"
  children_type="inline"
[q]
  command_name="q"
  children_type="inline"
[dfn]
  command_name="dfn"
  children_type="inline"
[abbr]
  command_name="abbr"
  children_type="inline"
  [[abbr.attribute]]
  "name" = "title"
  "type" = "inline option"
[img]
  command_name="img"
  [[img.attribute]]
  "name" = "src"
  "type" = "link"
  [[img.attribute]]
  "name" = "alt"
  "type" = "inline"
[del]
  command_name="strike"
  children_type="inline"
[table]
  command_name="table"
  children_type="block"
[thead]
  command_name="thead"
  children_type="block"
[tbody]
  command_name="tbody"
  children_type="block list"
[tr]
  command_name="tr"
  children_type="inline list"
[th]
  command_name="th"
  children_type="inline"
  [[th.attribute]]
  "name" = "align"
  "type" = "string option"
[td]
  command_name="td"
  children_type="inline"
  [[td.attribute]]
  "name" = "align"
  "type" = "string option"
"#;

fn make_default_config() -> map::Map<String, toml::Value> {
  DEFAULT_HTML_CONFIG
    .parse::<toml::Value>()
    .unwrap()
    .as_table()
    .unwrap()
    .clone()
}

pub fn escape_code(text: &str) -> String {
  text
    .replace("&amp;", "&")
    .replace("&lt;", "<")
    .replace("&gt;", ">")
    .replace("&quot;", "\"")
}

pub fn escape_inline_text(text: &str) -> String {
  text
    .replace('\\', "\\\\")
    .replace("&amp;", "&")
    .replace("&lt;", "<")
    .replace("&gt;", ">")
    .replace('{', "\\{")
    .replace('}', "\\}")
    .replace('<', "\\<")
    .replace('>', "\\>")
    .replace('%', "\\%")
    .replace('$', "\\$")
    .replace('#', "\\#")
    .replace(';', "\\;")
    .replace('|', "\\|")
    .replace('*', "\\*")
    .replace('@', "\\@")
    .replace('`', "\\`")
}

fn make_code(is_block: bool, code_str: &str) -> String {
  let i = count_accent_in_inline_text(code_str);
  let raw = "`".repeat(i + 1);
  let code = mdbook_specific_features::hiding_code_lines(code_str);
  if is_block {
    format!("{raw}\n{code}\n{raw}")
  } else {
    format!("{raw} {code} {raw}")
  }
}

pub fn count_accent_in_inline_text(text: &str) -> usize {
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

#[test]
fn check_count_accent_in_inline_text() {
  assert_eq!(3, count_accent_in_inline_text("aa``bb```c``d`"))
}

#[test]
fn check_count_accent_in_inline_text2() {
  assert_eq!(1, count_accent_in_inline_text("`"))
}
