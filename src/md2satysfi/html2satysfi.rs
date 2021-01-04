use html_parser::{Dom, Node};
use std::collections::hash_map::HashMap;
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
  html_cfg: &map::Map<String, toml::Value>,
) -> String {
  let node_lst = Dom::parse(html_code).unwrap().children;
  node_lst
    .iter()
    .map(|node| node_to_satysfi_code(node, mode, html_cfg))
    .collect()
}

fn node_to_satysfi_code(
  node: &Node,
  mode: Mode,
  html_cfg: &map::Map<String, toml::Value>,
) -> String {
  match node {
    Node::Comment(_) => String::new(),
    Node::Text(str) => match mode {
      Mode::Inline => escape_inline_text(str),
      Mode::Block => format!("+p{{{}}}", escape_inline_text(str)),
      Mode::Code => str.to_string(),
    },
    Node::Element(element) => {
      let tag_name = &element.name.to_lowercase();
      let children = &element.children;
      let attributes = &element.attributes;
      let toml_data_opt = html_cfg.get(tag_name);
      match toml_data_opt {
        Some(toml_data) => {
          let command_name = toml_data
            .get("command-name")
            .map(|v| v.as_str())
            .flatten()
            .unwrap_or(tag_name);
          let children_type_opt = toml_data
            .get("children_type")
            .map(|v| v.as_str().map(|s| parse_children_type(s)))
            .flatten()
            .flatten();
          let attribute_cfg_lst = match toml_data.get("attribute").map(|v| v.as_array()).flatten() {
            Some(v) => v.clone(),
            None => Vec::new(),
          };
          let children_str = match children_type_opt {
            None => String::new(),
            Some(children_type) => match children_type {
              ChildrenType::BlockText => {
                let children_str = children
                  .iter()
                  .map(|node| node_to_satysfi_code(node, Mode::Block, html_cfg))
                  .collect::<String>();
                format!("<{}>", children_str)
              }
              ChildrenType::InlineText => {
                let children_str = children
                  .iter()
                  .map(|node| node_to_satysfi_code(node, Mode::Inline, html_cfg))
                  .collect::<String>();
                format!("{{{}}}", children_str)
              }
              ChildrenType::BlockCode => {
                let children_str = children
                  .iter()
                  .map(|node| node_to_satysfi_code(node, Mode::Code, html_cfg))
                  .collect::<String>();
                format!("({});", make_code(true, &children_str))
              }
              ChildrenType::InlineCode => {
                let children_str = children
                  .iter()
                  .map(|node| node_to_satysfi_code(node, Mode::Code, html_cfg))
                  .collect::<String>();
                format!("({});", make_code(false, &children_str))
              }
            },
          };
          let attributes_str = attribute_cfg_lst
            .iter()
            .map(|toml| {
              let attribute_name_opt = toml.get("toml").map(|v| v.as_str()).flatten();
              let attribute_type_opt = toml
                .get("type")
                .map(|v| v.as_str().map(|s| parse_attribute_type(s)))
                .flatten()
                .flatten();
              match (attribute_name_opt, attribute_type_opt) {
                (Some(attribute_name), Some(attribute_type)) => match mode {
                  Mode::Code => {
                    let attribute_value_opt = attributes.get(attribute_name).unwrap_or(&None);
                    match attribute_type {
                      AttributeTypeOrOption::Option(_) => match attribute_value_opt {
                        None => String::new(),
                        Some(attribute_value) => {
                          format!(r#" {}="{}""#, attribute_name, attribute_value)
                        }
                      },
                      AttributeTypeOrOption::Normal(_) => {
                        let attribute_value = attribute_value_opt.clone().unwrap();
                        format!(r#" {}="{}""#, attribute_name, attribute_value)
                      }
                    }
                  }
                  _ => {
                    let attribute_value_opt = attributes.get(attribute_name).unwrap_or(&None);
                    match attribute_type {
                      AttributeTypeOrOption::Option(attribute_type) => match attribute_value_opt {
                        None => "(None)".to_string(),
                        Some(attribute_value) => match attribute_type {
                          AttributeType::BlockText => format!(r#"(Some('<{}>))"#, attribute_value),
                          AttributeType::InlineText => {
                            format!(r#"(Some({{{}}}))"#, attribute_value)
                          }
                          AttributeType::String => {
                            format!(r#"(Some({}))"#, make_code(false, attribute_value))
                          }
                          AttributeType::Link => {
                            format!(r#"(Some({}))"#, make_code(false, attribute_value))
                          }
                        },
                      },
                      AttributeTypeOrOption::Normal(attribute_type) => match attribute_value_opt {
                        None => {
                          eprintln!(r#""{}" tag is not supported"#, tag_name);
                          String::new()
                        }
                        Some(attribute_value) => match attribute_type {
                          AttributeType::BlockText => {
                            format!(r#"('<{}>)"#, attribute_value)
                          }
                          AttributeType::InlineText => {
                            format!(r#"({{{}}})"#, attribute_value)
                          }
                          AttributeType::String => {
                            format!(r#"({})"#, make_code(false, &attribute_value))
                          }
                          AttributeType::Link => {
                            format!(r#"({})"#, make_code(false, &attribute_value))
                          }
                        },
                      },
                    }
                  }
                },
                _ => String::new(),
              }
            })
            .collect::<String>();
          match mode {
            Mode::Block => {
              format!(
                "+{command_name}{attributes_str}{children_str}",
                command_name = command_name,
                attributes_str = attributes_str,
                children_str = children_str
              )
            }
            Mode::Inline => {
              format!(
                "\\{command_name}{attributes_str}{children_str}",
                command_name = command_name,
                attributes_str = attributes_str,
                children_str = children_str
              )
            }
            Mode::Code => {
              format!(
                "<{tag_name} {attributes_str}>{children_str}</{tag_name}>",
                tag_name = tag_name,
                attributes_str = attributes_str,
                children_str = children_str
              )
            }
          }
        }
        None => {
          eprintln!(r#""{}" tag is not supported"#, tag_name);
          String::new()
        }
      }
    }
  }
}

#[test]
fn check_html_to_satysfi_code_1() {
  assert_eq!(
    r#"+p{this is a image.\img(` img/14-03.jpg `);\code(`` let p = `<p>x</p>` ``);}"#.to_owned(),
    html_to_satysfi_code(
      r#"<p> this is a image. <img src="img/14-03.jpg" class="center" /> <code>let p = `<p>x</p>`</code></p>"#,
      Mode::Block,
      &map::Map::new()
    )
  )
}

#[test]
fn check_html_to_satysfi_code_2() {
  assert_eq!(
    r#"\p{this is a image.\img(` img/14-03.jpg `);\code(`` let p = `<p>x</p>` ``);}"#.to_owned(),
    html_to_satysfi_code(
      r#"<p> this is a image. <img src="img/14-03.jpg" class="center" /> <code>let p = `<p>x</p>`</code></p>"#,
      Mode::Inline,
      &map::Map::new()
    )
  )
}

#[test]
fn check_html_to_satysfi_code_3() {
  assert_eq!(
    r#"\ruby{如何\rp{(}\rt{いか}\rp{)}}"#.to_string(),
    html_to_satysfi_code(
      r#"<ruby>如何<rp>(</rp><rt>いか</rt><rp>)</rp></ruby>"#,
      Mode::Inline,
      &map::Map::new()
    )
  )
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum ChildrenType {
  BlockText,
  InlineText,
  BlockCode,
  InlineCode,
}

fn parse_children_type(type_str: &str) -> Option<ChildrenType> {
  match type_str.to_string().to_lowercase().as_str() {
    "inline" => Some(ChildrenType::InlineText),
    "block" => Some(ChildrenType::BlockText),
    "inline-text" => Some(ChildrenType::InlineText),
    "block-text" => Some(ChildrenType::BlockText),
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
    _ => None,
  }
}

//fn tag_p_to_code(
//  _attributes: &HashMap<String, Option<String>>,
//  children: &[Node],
//  mode: &Mode,
//) -> String {
//  match mode {
//    Mode::Block => {
//      format!(
//        "+p{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Inline => {
//      format!(
//        "\\p{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Code => {
//      format!(
//        "<p>{}</p>",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Code))
//          .collect::<String>()
//      )
//    }
//  }
//}
//
//fn tag_code_to_code(
//  _attributes: &HashMap<String, Option<String>>,
//  children: &[Node],
//  mode: &Mode,
//) -> String {
//  match mode {
//    Mode::Block => {
//      format!(
//        "+code({});",
//        make_code(
//          true,
//          &children
//            .iter()
//            .map(|node| node_to_satysfi_code(node, Mode::Code))
//            .collect::<String>()
//        )
//      )
//    }
//    Mode::Inline => {
//      format!(
//        "\\code({});",
//        make_code(
//          false,
//          &children
//            .iter()
//            .map(|node| node_to_satysfi_code(node, Mode::Code))
//            .collect::<String>()
//        )
//      )
//    }
//    Mode::Code => {
//      format!(
//        "<code>{}</code>",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Code))
//          .collect::<String>()
//      )
//    }
//  }
//}
//
//fn tag_img_to_code(
//  attributes: &HashMap<String, Option<String>>,
//  _children: &[Node],
//  mode: &Mode,
//) -> String {
//  let src = attributes.get("src").unwrap().clone().unwrap();
//  match mode {
//    Mode::Block => {
//      format!("+img({});", make_code(false, &src))
//    }
//    Mode::Inline => {
//      format!("\\img({});", make_code(false, &src))
//    }
//    Mode::Code => {
//      format!("<img src=\"{}\"/>", src)
//    }
//  }
//}
//
//fn tag_span_to_code(
//  _attributes: &HashMap<String, Option<String>>,
//  children: &[Node],
//  mode: &Mode,
//) -> String {
//  match mode {
//    Mode::Block => {
//      format!(
//        "+span{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Inline => {
//      format!(
//        "\\span{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Code => {
//      format!(
//        "<span>{}</span>",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Code))
//          .collect::<String>()
//      )
//    }
//  }
//}
//
//fn tag_ruby_to_code(
//  _attributes: &HashMap<String, Option<String>>,
//  children: &[Node],
//  mode: &Mode,
//) -> String {
//  match mode {
//    Mode::Block => {
//      format!(
//        "+ruby{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Inline => {
//      format!(
//        "\\ruby{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Code => {
//      format!(
//        "<ruby>{}</ruby>",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Code))
//          .collect::<String>()
//      )
//    }
//  }
//}
//
//fn tag_rp_to_code(
//  _attributes: &HashMap<String, Option<String>>,
//  children: &[Node],
//  mode: &Mode,
//) -> String {
//  match mode {
//    Mode::Block => {
//      format!(
//        "+rp{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Inline => {
//      format!(
//        "\\rp{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Code => {
//      format!(
//        "<rp>{}</rp>",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Code))
//          .collect::<String>()
//      )
//    }
//  }
//}
//
//fn tag_rt_to_code(
//  _attributes: &HashMap<String, Option<String>>,
//  children: &[Node],
//  mode: &Mode,
//) -> String {
//  match mode {
//    Mode::Block => {
//      format!(
//        "+rt{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Inline => {
//      format!(
//        "\\rt{{{}}}",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Inline))
//          .collect::<String>()
//      )
//    }
//    Mode::Code => {
//      format!(
//        "<rt>{}</rt>",
//        children
//          .iter()
//          .map(|node| node_to_satysfi_code(node, Mode::Code))
//          .collect::<String>()
//      )
//    }
//  }
//}

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
  let raw = "`".repeat(i + 1);
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
