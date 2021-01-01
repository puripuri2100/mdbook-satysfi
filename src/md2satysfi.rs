use pulldown_cmark::Alignment;
use pulldown_cmark::Event;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use std::path;

mod mdbook_specific_features;

#[derive(Debug, Clone)]
enum TextMode {
  Block,
  Inline,
  List,
  Table,
  Code,
}

pub fn md_to_satysfi_code(md_text: String, file_path: &path::PathBuf) -> Result<String, ()> {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_FOOTNOTES);
  options.insert(Options::ENABLE_TASKLISTS);
  options.insert(Options::ENABLE_SMART_PUNCTUATION);
  let parser = Parser::new_ext(&md_text, options);
  parser_to_code(parser, file_path)
}

fn parser_to_code(parser: Parser, file_path: &path::PathBuf) -> Result<String, ()> {
  let mut s = String::new();
  let mut code_str = String::new();
  let mut stack = vec![TextMode::Block];
  for event in parser {
    match event {
      Event::Start(tag) => match tag {
        Tag::Paragraph => {
          s.push_str("+p {");
          stack.push(TextMode::Inline);
        }
        Tag::Heading(level) => {
          s.push_str(&format!("+heading ({level}) {{", level = level));
          stack.push(TextMode::Inline);
        }
        Tag::BlockQuote => {
          s.push_str("+block-quote <\n");
          stack.push(TextMode::Block);
        }
        Tag::CodeBlock(_code_block_kind) => {
          s.push_str("+code (");
          stack.push(TextMode::Code);
        }
        Tag::List(_dep_opt) => {
          s.push_str("+listing {\n");
          stack.push(TextMode::List);
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
          stack.push(TextMode::Table);
        }
        Tag::TableHead => {}
        Tag::TableRow => s.push('\n'),
        Tag::TableCell => s.push('|'),
        Tag::Emphasis => {
          s.push_str("\\emph {");
          stack.push(TextMode::Inline);
        }
        Tag::Strong => {
          s.push_str("\\strong {");
          stack.push(TextMode::Inline);
        }
        Tag::Strikethrough => {}
        Tag::Link(_link_type, link, _title) => {
          s.push_str(&format!("\\href (``` {url} ```) {{", url = link,));
          stack.push(TextMode::Inline);
        }
        Tag::Image(_link_type, _link, _title) => {
          //s.push_str(&format!(
          //  "\\href(``` {link} ```){{{title}}}{{",
          //  link = link,
          //  title = title
          //));
          stack.push(TextMode::Inline);
        }
      },
      Event::End(tag) => match tag {
        Tag::Paragraph => {
          s.push_str("}\n");
          stack.pop();
        }
        Tag::Heading(_) => {
          s.push_str("}\n");
          stack.pop();
        }
        Tag::BlockQuote => {
          s.push_str(">\n");
          stack.pop();
        }
        Tag::CodeBlock(_) => {
          let code = mdbook_specific_features::hiding_code_lines(
            &mdbook_specific_features::parse_include_file(&code_str, file_path),
          );
          let n = count_accent_in_inline_text(&code);
          let raw = "`".repeat(n + 1);
          s.push_str(&format!("{raw}\n{code}\n{raw});\n", raw = raw, code = code));
          code_str = String::new();
          stack.pop();
        }
        Tag::List(_) => {
          s.push_str("}\n");
          stack.pop();
        }
        Tag::Item => s.push('\n'),
        Tag::FootnoteDefinition(_) => {}
        Tag::Table(_) => {
          s.push_str("|}}\n");
          stack.pop();
        }
        Tag::TableHead => {}
        Tag::TableRow => {}
        Tag::TableCell => {}
        Tag::Emphasis => {
          s.push('}');
          stack.pop();
        }
        Tag::Strong => {
          s.push('}');
          stack.pop();
        }
        Tag::Strikethrough => {}
        Tag::Link(_, _, _) => {
          s.push('}');
          stack.pop();
        }
        Tag::Image(_, _, _) => {
          //s.push_str("}");
          stack.pop();
        }
      },
      Event::Text(text) => {
        let now_mode_opt = stack.pop();
        let t = match now_mode_opt {
          Some(TextMode::Code) => {
            code_str.push_str(&text);
            String::new()
          }
          _ => escape_inline_text(&mdbook_specific_features::parse_include_file(
            &text, file_path,
          )),
        };
        s.push_str(&t);
        stack.push(now_mode_opt.unwrap_or(TextMode::Code))
      }
      Event::Code(code) => {
        let n = count_accent_in_inline_text(&code);
        let raw = "`".repeat(n + 1);
        s.push_str(&format!(
          "\\code({raw} {code} {raw});",
          raw = raw,
          code = code
        ));
      }
      Event::Html(html_code) => {} //s.push_str("\\<html code\\>"),
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
