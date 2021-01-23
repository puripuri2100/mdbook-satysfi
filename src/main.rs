extern crate mdbook;
extern crate pulldown_cmark;

use mdbook::renderer::RenderContext;
use mdbook::BookItem;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path;
use toml::map;

mod copy;
mod md2satysfi;

fn main() {
  let stdin = io::stdin();
  let mut stdin = BufReader::new(stdin);
  let ctx = RenderContext::from_json(&mut stdin).unwrap();

  let destination = &ctx.destination;
  let _ = fs::create_dir_all(&destination);
  let f = File::create(&destination.join("main.saty")).unwrap();
  let mut f = BufWriter::new(f);

  let root = &ctx.source_dir();

  let src_dir = &ctx.root.join(&ctx.config.book.src);
  let build_dir = &ctx.root.join(&ctx.config.build.build_dir);
  copy::copy_files_except_ext(src_dir, &destination, Some(build_dir), &["md"]);

  let cfg = &ctx.config;
  let book = &cfg.book;
  let satysfi_cfg = match &cfg
    .get("output.satysfi")
    .map(|toml| toml.as_table())
    .flatten()
  {
    None => map::Map::new(),
    Some(map) => (*map).clone(),
  };
  let html_cfg = match &cfg
    .get("output.satysfi.html")
    .map(|toml| toml.as_table())
    .flatten()
  {
    None => map::Map::new(),
    Some(map) => (*map).clone(),
  };

  let title = &book.clone().title.unwrap_or_default();
  let authors = &book.authors;
  let len = authors.len();
  let author = authors
    .iter()
    .enumerate()
    .map(|(i, s)| {
      if i == 0 {
        s.to_string()
      } else if i == len - 1 {
        format!("and {}", s)
      } else {
        format!(", {}", s)
      }
    })
    .collect::<String>();

  let require_packages_str = &satysfi_cfg
    .get("require-packages")
    .map(|v| {
      let s_opt = {
        v.as_array().map(|lst| {
          let new = lst
            .iter()
            .map(|v| v.as_str())
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap_or_default())
            .collect::<Vec<_>>();
          if new.len() < lst.len() {
            eprintln!(r#""output.satysfi.require-packages" require type "string array""#);
            String::new()
          } else {
            new.iter().map(|s| format!("@require: {}\n", s)).collect()
          }
        })
      };
      s_opt.unwrap_or_else(|| {
        eprintln!(r#""output.satysfi.require-packages" require type "string array""#);
        String::new()
      })
    })
    .unwrap_or_default();
  let import_packages_str = &satysfi_cfg
    .get("import-packages")
    .map(|v| {
      let s_opt = {
        v.as_array().map(|lst| {
          let new = lst
            .iter()
            .map(|v| v.as_str())
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap_or_default())
            .collect::<Vec<_>>();
          if new.len() < lst.len() {
            eprintln!(r#""output.satysfi.import-packages" require type "string array""#);
            String::new()
          } else {
            new.iter().map(|s| format!("@import: {}\n", s)).collect()
          }
        })
      };
      s_opt.unwrap_or_else(|| {
        eprintln!(r#""output.satysfi.import-packages" require type "string array""#);
        String::new()
      })
    })
    .unwrap_or_default();

  f.write_all(
    format!(
      "%@require: class-mdbook-satysfi/mdbook-satysfi
@import: ../../../src/mdbook-satysfi
{require_packages}
{import_packages}


document (|
  title = {{{title}}};
  author = {{{author}}};
|) '<",
      require_packages = require_packages_str,
      import_packages = import_packages_str,
      title = title,
      author = author
    )
    .as_bytes(),
  )
  .unwrap();

  ctx
    .book
    .iter()
    .for_each(|item| write_bookitme(&mut f, item, &root, &html_cfg));
  f.write_all(b">\n").unwrap();
}

fn write_bookitme(
  f: &mut BufWriter<File>,
  item: &BookItem,
  root: &path::PathBuf,
  html_cfg: &map::Map<String, toml::Value>,
) {
  let indent_str = "  ".to_string();
  match item {
    BookItem::Chapter(ch) => {
      let ch_name = ch.clone().name;
      let parent_names = ch.clone().parent_names;
      let depth = parent_names.len();
      f.write_all(
        format!(
          "{indent}+Chapter ({depth}) {{{name}}} <\n",
          indent = indent_str,
          depth = depth,
          name = md2satysfi::escape_inline_text(&ch_name)
        )
        .as_bytes(),
      )
      .unwrap();
      let ch_file_path_opt = ch.clone().path;
      match ch_file_path_opt {
        None => (),
        Some(ch_file_path) => {
          let path = root.join(&ch_file_path);
          let s =
            md2satysfi::md_to_satysfi_code(ch.clone().content, &path, &ch_file_path, html_cfg)
              .unwrap();
          f.write_all(s.as_bytes()).unwrap()
        }
      };
      f.write_all(format!("{}>\n", indent_str).as_bytes())
        .unwrap();
    }
    BookItem::Separator => {
      f.write_all(format!("{indent}+Separator;\n", indent = indent_str).as_bytes())
        .unwrap();
    }
    BookItem::PartTitle(title) => {
      f.write_all(
        format!(
          "{indent}+PartTitle{{{title}}};\n",
          indent = indent_str,
          title = md2satysfi::escape_inline_text(title)
        )
        .as_bytes(),
      )
      .unwrap();
    }
  }
}
