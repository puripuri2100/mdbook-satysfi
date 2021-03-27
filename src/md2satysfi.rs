use anyhow::Result;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::path::Path;
use toml::map;

pub mod html2satysfi;
mod mdbook_specific_features;

pub fn md_to_satysfi_code(
  md_text: String,
  file_path: &Path,
  ch_file_path: &Path,
  html_cfg: &map::Map<String, toml::Value>,
) -> Result<String> {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_FOOTNOTES);
  options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_TASKLISTS);
  options.insert(Options::ENABLE_SMART_PUNCTUATION);
  let parser = Parser::new_ext(&md_text, options).map(|event| match event {
    Event::SoftBreak => Event::Text(" ".into()),
    Event::FootnoteReference(tag) => {
      let html_tag = format!(r#"<span class="footnote-reference" tag="{}" />"#, tag);
      Event::Html(html_tag.into())
    }
    Event::Start(Tag::FootnoteDefinition(tag)) => {
      let html_tag = format!(r#"<span class="footnote-definition" tag="{}">"#, tag);
      Event::Html(html_tag.into())
    }
    Event::End(Tag::FootnoteDefinition(_)) => Event::Html("</span>".into()),
    Event::TaskListMarker(bool) => {
      let html_tag = format!(
        r#"<span class="task-list-marker" checked="{bool}" />"#,
        bool = bool
      );
      Event::Html(html_tag.into())
    }
    Event::Start(Tag::CodeBlock(_)) => Event::Html(r#"<div class="code-block">"#.into()),
    Event::End(Tag::CodeBlock(_)) => Event::Html(r#"</div>"#.into()),
    Event::Start(Tag::List(Some(n))) => {
      let html_tag = format!(r#"<ol start="{}">"#, n);
      Event::Html(html_tag.into())
    }
    Event::End(Tag::List(Some(_))) => Event::Html(r#"</ol>"#.into()),
    _ => event,
  });
  let mut html_code = String::new();
  html::push_html(&mut html_code, parser);
  let satysfi_code = html2satysfi::html_to_satysfi_code(
    &html_code,
    html2satysfi::Mode::Block,
    file_path,
    html_cfg,
    ch_file_path,
  )?;
  Ok(satysfi_code)
}
