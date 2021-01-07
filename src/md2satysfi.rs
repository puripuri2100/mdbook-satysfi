use pulldown_cmark::Alignment;
use pulldown_cmark::Event;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use regex::Regex;
use std::path;
use toml::map;

mod html2satysfi;
mod mdbook_specific_features;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TextMode {
  Block,
  Inline,
  List,
  Table,
  Code,
  Html,
  HtmlComment,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Mode {
  pub default: TextMode,
  pub now: TextMode,
  pub stack: Vec<TextMode>,
}

impl Mode {
  fn new(default_mode: TextMode) -> Self {
    Mode {
      default: default_mode.clone(),
      now: default_mode,
      stack: Vec::new(),
    }
  }

  fn now(&self) -> TextMode {
    self.clone().now
  }

  fn push(&mut self, mode: TextMode) -> Self {
    let mut stack = self.clone().stack;
    stack.push(self.clone().now);
    Mode {
      now: mode,
      stack,
      ..self.clone()
    }
  }

  fn pop(&mut self) -> Self {
    let default = self.clone().default;
    let mut stack = self.clone().stack;
    let now = stack.pop().unwrap_or_else(|| default.clone());
    Mode {
      default,
      now,
      stack,
    }
  }
}

#[test]
fn check_mode() {
  let mut stack = Mode::new(TextMode::Block);
  stack = stack.push(TextMode::Inline);
  stack = stack.push(TextMode::Block);
  stack = stack.push(TextMode::Inline);
  stack = stack.push(TextMode::Block);
  stack = stack.push(TextMode::Inline);
  assert_eq!(
    vec![
      TextMode::Block,
      TextMode::Inline,
      TextMode::Block,
      TextMode::Inline,
      TextMode::Block
    ],
    stack.stack
  );
  assert_eq!(TextMode::Inline, stack.now());
  stack = stack.pop();
  assert_eq!(TextMode::Block, stack.now());
}

pub fn md_to_satysfi_code(
  md_text: String,
  file_path: &path::PathBuf,
  ch_file_path: &path::PathBuf,
  html_cfg: &map::Map<String, toml::Value>,
) -> Result<String, ()> {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_FOOTNOTES);
  options.insert(Options::ENABLE_TASKLISTS);
  options.insert(Options::ENABLE_SMART_PUNCTUATION);
  let parser = Parser::new_ext(&md_text, options);
  parser_to_code(parser, file_path, ch_file_path, html_cfg)
}

fn parser_to_code(
  parser: Parser,
  file_path: &path::PathBuf,
  ch_file_path: &path::PathBuf,
  html_cfg: &map::Map<String, toml::Value>,
) -> Result<String, ()> {
  let mut s = String::new();
  let mut code_str = String::new();
  let mut html_str = String::new();
  let mut mode = Mode::new(TextMode::Block);
  for event in parser {
    match event {
      Event::Start(tag) => match tag {
        Tag::Paragraph => match mode.now() {
          TextMode::Html | TextMode::HtmlComment => {}
          _ => {
            s.push_str("+p {");
            mode = mode.push(TextMode::Inline)
          }
        },
        Tag::Heading(level) => {
          s.push_str(&format!("+heading ({level}) {{", level = level));
          mode = mode.push(TextMode::Inline);
        }
        Tag::BlockQuote => {
          s.push_str("+block-quote <\n");
          mode = mode.push(TextMode::Block);
        }
        Tag::CodeBlock(_code_block_kind) => {
          s.push_str("+code (");
          mode = mode.push(TextMode::Code);
        }
        Tag::List(_dep_opt) => {
          s.push_str("+listing {\n");
          mode = mode.push(TextMode::List);
        }
        Tag::Item => s.push_str("* "),
        Tag::FootnoteDefinition(_text) => {}
        Tag::Table(alignment_list) => {
          let alignment_text: String = alignment_list
            .iter()
            .map(|alignment| match alignment {
              Alignment::None => "c;",
              Alignment::Left => "l;",
              Alignment::Right => "r;",
              Alignment::Center => "c;",
            })
            .collect();
          s.push_str(&format!(
            "+p{{\\easytable [{alignment}] {{",
            alignment = alignment_text
          ));
          mode = mode.push(TextMode::Table);
        }
        Tag::TableHead => {}
        Tag::TableRow => s.push('\n'),
        Tag::TableCell => s.push('|'),
        Tag::Emphasis => {
          s.push_str("\\emph {");
          mode = mode.push(TextMode::Inline);
        }
        Tag::Strong => {
          s.push_str("\\strong {");
          mode = mode.push(TextMode::Inline);
        }
        Tag::Strikethrough => {}
        Tag::Link(_link_type, link, _title) => {
          s.push_str(&format!("\\href (``` {url} ```) {{", url = link,));
          mode = mode.push(TextMode::Inline);
        }
        Tag::Image(_link_type, link, _title) => {
          let path_str = format!(
            "{}/{}",
            ch_file_path.parent().unwrap().to_str().unwrap(),
            &link
          );
          s.push_str(&format!("\\img(``` {link} ```){{", link = path_str));
          mode = mode.push(TextMode::Inline);
        }
      },
      Event::End(tag) => match tag {
        Tag::Paragraph => match mode.now() {
          TextMode::Html | TextMode::HtmlComment => {}
          _ => {
            s.push_str("}\n");
            mode = mode.pop();
          }
        },
        Tag::Heading(_) => {
          s.push_str("}\n");
          mode = mode.pop();
        }
        Tag::BlockQuote => {
          s.push_str(">\n");
          mode = mode.pop();
        }
        Tag::CodeBlock(_) => {
          let code = mdbook_specific_features::hiding_code_lines(
            &mdbook_specific_features::parse_include_file(&code_str, file_path),
          );
          let n = count_accent_in_inline_text(&code);
          let raw = "`".repeat(n + 1);
          s.push_str(&format!("{raw}\n{code}\n{raw});\n", raw = raw, code = code));
          code_str = String::new();
          mode = mode.pop();
        }
        Tag::List(_) => {
          s.push_str("}\n");
          mode = mode.pop();
        }
        Tag::Item => s.push('\n'),
        Tag::FootnoteDefinition(_) => {}
        Tag::Table(_) => {
          s.push_str("|}}\n");
          mode = mode.pop();
        }
        Tag::TableHead => {}
        Tag::TableRow => {}
        Tag::TableCell => {}
        Tag::Emphasis => {
          s.push('}');
          mode = mode.pop();
        }
        Tag::Strong => {
          s.push('}');
          mode = mode.pop();
        }
        Tag::Strikethrough => {}
        Tag::Link(_, _, _) => {
          s.push('}');
          mode = mode.pop();
        }
        Tag::Image(_, _, _) => {
          s.push('}');
          mode = mode.pop();
        }
      },
      Event::Text(text) => {
        let now_mode = mode.now();
        match now_mode {
          TextMode::Code => {
            code_str.push_str(&text);
          }
          TextMode::Html => {
            html_str.push_str(&text);
          }
          TextMode::HtmlComment => {
            html_str.push_str(&text);
          }
          _ => {
            let t = escape_inline_text(&mdbook_specific_features::parse_include_file(
              &text, file_path,
            ));
            s.push_str(&t)
          }
        }
      }
      Event::Code(code) => match mode.now() {
        TextMode::Html | TextMode::HtmlComment => html_str.push_str(&code),
        _ => {
          let n = count_accent_in_inline_text(&code);
          let raw = "`".repeat(n + 1);
          s.push_str(&format!(
            "\\code({raw} {code} {raw});",
            raw = raw,
            code = code
          ));
        }
      },
      Event::Html(html_code) => {
        let start_tag_re =
          Regex::new("^((<[^!/]>|<[^!/][^<]*[^/]>).*|<[^!/]|<[^!/][^<>/]*)").unwrap();
        let end_tag_re = Regex::new(".*</[^>]*[^/]>").unwrap();
        let start_end_re =
          Regex::new("((<[^!/]>|<[^!/][^<]*[^/]>).*</[^>]*[^/]>)|<[^!/].*/>").unwrap();
        let one_line_comment_re = Regex::new("<!--[\\s\\S]*?-->").unwrap();
        let comment_start_re = Regex::new("<!--.*").unwrap();
        let comment_end_re = Regex::new(".*-->").unwrap();
        if start_end_re.is_match(&html_code) {
          match mode.now() {
            TextMode::Html | TextMode::HtmlComment => html_str.push_str(&html_code),
            TextMode::Block => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_code,
                html2satysfi::Mode::Block,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
            }
            TextMode::Code => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_code,
                html2satysfi::Mode::Code,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
            }
            _ => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_code,
                html2satysfi::Mode::Inline,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
            }
          }
        } else if start_tag_re.is_match(&html_code) {
          match mode.now() {
            TextMode::HtmlComment => html_str.push_str(&html_code),
            _ => {
              mode = mode.push(TextMode::Html);
              html_str.push_str(&html_code)
            }
          }
        } else if end_tag_re.is_match(&html_code) {
          match mode.now() {
            TextMode::HtmlComment => html_str.push_str(&html_code),
            _ => {
              mode = mode.pop();
            }
          };
          html_str.push_str(&html_code);
          match mode.now() {
            TextMode::Html | TextMode::HtmlComment => {}
            TextMode::Block => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_str,
                html2satysfi::Mode::Block,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
              html_str = String::new();
            }
            TextMode::Code => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_str,
                html2satysfi::Mode::Code,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
              html_str = String::new();
            }
            _ => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_str,
                html2satysfi::Mode::Inline,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
              html_str = String::new();
            }
          }
        } else if one_line_comment_re.is_match(&html_code) {
        } else if comment_start_re.is_match(&html_code) {
          mode = mode.push(TextMode::HtmlComment);
          html_str.push_str(&html_code);
        } else if comment_end_re.is_match(&html_code) {
          match mode.now() {
            TextMode::HtmlComment => {
              mode = mode.pop();
              html_str.push_str(&html_code);
            }
            TextMode::Html => {
              html_str.push_str(&html_code);
            }
            TextMode::Code => {
              code_str.push_str(&html_code);
            }
            _ => {}
          }
          match mode.now() {
            TextMode::Html | TextMode::HtmlComment => {}
            TextMode::Block => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_str,
                html2satysfi::Mode::Block,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
              html_str = String::new();
            }
            TextMode::Code => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_str,
                html2satysfi::Mode::Code,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
              html_str = String::new();
            }
            _ => {
              // end html code
              let satysfi_code = html2satysfi::html_to_satysfi_code(
                &html_str,
                html2satysfi::Mode::Inline,
                html_cfg,
                ch_file_path,
              );
              s.push_str(&satysfi_code);
              html_str = String::new();
            }
          }
        } else {
          match mode.now() {
            TextMode::Html => html_str.push_str(&html_code),
            TextMode::HtmlComment => html_str.push_str(&html_code),
            _ => {}
          }
        }
      }
      Event::FootnoteReference(footnote) => {
        s.push_str(&format!("\\footnote{{{footnote}}}", footnote = footnote));
      }
      Event::SoftBreak => {} //s.push(' '),
      Event::HardBreak => s.push('\n'),
      Event::Rule => s.push_str("\\rule;"),
      Event::TaskListMarker(bool) => {
        s.push_str(&format!("\\task-list-marker({bool});", bool = bool));
      }
    };
  }
  Ok(s)
}

pub fn escape_inline_text(text: &str) -> String {
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
