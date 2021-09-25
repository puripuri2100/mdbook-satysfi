use anyhow::{Context, Result};
use clap::App;
use mdbook::renderer::RenderContext;
use mdbook::BookItem;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use toml::map;

mod copy;
mod md2satysfi;
mod run_satysfi;

fn main() -> Result<()> {
  let app = App::new("mdbook-satysfi")
    .version("0.0.5")
    .author("Naoki Kaneko <puripuri2100@gmail.com>")
    .about("A mdbook backend for generating SATySFi documents");
  let _ = app.get_matches();

  let stdin = io::stdin();
  let mut stdin = BufReader::new(stdin);
  let ctx = RenderContext::from_json(&mut stdin)?;

  let destination = &ctx.destination;
  let _ = fs::create_dir_all(&destination);
  let f =
    File::create(&destination.join("main.saty")).with_context(|| "Cannot make 'main.saty'")?;
  let mut f = BufWriter::new(f);

  let source_dir = &ctx.source_dir();

  let src_dir = &ctx.root.join(&ctx.config.book.src);
  let build_dir = &ctx.root.join(&ctx.config.build.build_dir);
  copy::copy_files_except_ext(src_dir, &destination, Some(build_dir), &["md"])?;

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
  let pdf_toml_value_opt = &cfg.get("output.satysfi.pdf");
  let pdf_cfg_opt = match (
    pdf_toml_value_opt.map(|v| v.as_bool()).flatten(),
    pdf_toml_value_opt.map(|v| v.as_table()).flatten(),
  ) {
    (_, Some(t)) => Some(t.clone()),
    (Some(b), _) => {
      if b {
        Some(map::Map::new())
      } else {
        None
      }
    }
    _ => None,
  };

  let title = md2satysfi::html2satysfi::escape_inline_text(&book.clone().title.unwrap_or_default());
  let authors = &book.clone().authors;
  let authors = authors
    .iter()
    .map(|s| format!("{{{}}}; ", md2satysfi::html2satysfi::escape_inline_text(s)))
    .collect::<String>();
  let description_opt_str = match book.clone().description {
    None => "None".to_string(),
    Some(description) => format!(
      "Some({{{}}})",
      md2satysfi::html2satysfi::escape_inline_text(&description)
    ),
  };
  let language_opt_str = match book.clone().language {
    None => "None".to_string(),
    Some(language) => {
      let n = md2satysfi::html2satysfi::count_accent_in_inline_text(&language);
      let raw = "`".repeat(n + 1);
      format!(
        "Some({raw} {language} {raw})",
        raw = raw,
        language = language
      )
    }
  };

  let class_file_name = &satysfi_cfg
    .get("class-file-name")
    .map(|v| v.as_str())
    .flatten()
    .unwrap_or("class-mdbook-satysfi/mdbook-satysfi");

  let is_class_file_require = &satysfi_cfg
    .get("is-class-file-require")
    .map(|v| v.as_bool())
    .flatten()
    .unwrap_or(true);
  let class_file_import_type = if *is_class_file_require {
    "require"
  } else {
    "import"
  };

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
      "@{class_file_import_type}: {class_file_name}
{require_packages}
{import_packages}


document (|
  title = {{{title}}};
  authors = [{authors}];
  description = ({description_opt});
  language = ({language_opt});
|) '<",
      class_file_import_type = class_file_import_type,
      class_file_name = class_file_name,
      require_packages = require_packages_str,
      import_packages = import_packages_str,
      title = title,
      authors = authors,
      description_opt = description_opt_str,
      language_opt = language_opt_str,
    )
    .as_bytes(),
  )?;

  ctx
    .book
    .iter()
    .try_for_each(|item| write_bookitme(&mut f, item, &source_dir, &html_cfg))?;

  f.write_all(b"\n>\n")?;
  f.flush()?;
  if let Some(pdf_cfg) = pdf_cfg_opt {
    let msg = run_satysfi::run_satysfi(&destination, pdf_cfg)?;
    println!("{}", String::from_utf8(msg)?)
  }
  Ok(())
}

fn write_bookitme(
  f: &mut BufWriter<File>,
  item: &BookItem,
  root: &Path,
  html_cfg: &map::Map<String, toml::Value>,
) -> Result<()> {
  let indent_str = "  ".to_string();
  match item {
    BookItem::Chapter(ch) => {
      let ch_name = ch.clone().name;
      let parent_names = ch.clone().parent_names;
      let depth = parent_names.len();
      let numbers_opt = ch.clone().number;
      let numbers_str = match numbers_opt {
        None => "None".to_string(),
        Some(numbers) => {
          let mut s = String::new();
          for n in numbers.iter() {
            s.push_str(&format!("{};", n))
          }
          format!("Some([{}])", s)
        }
      };
      f.write_all(
        format!(
          "\n\n{indent}+Chapter ({numbers}) ({depth}) {{{name}}} <",
          indent = indent_str,
          numbers = numbers_str,
          depth = depth,
          name = md2satysfi::html2satysfi::escape_inline_text(&ch_name)
        )
        .as_bytes(),
      )?;
      let ch_file_path_opt = ch.clone().path;
      match ch_file_path_opt {
        None => (),
        Some(ch_file_path) => {
          let path = root.join(&ch_file_path);
          let s =
            md2satysfi::md_to_satysfi_code(ch.clone().content, &path, &ch_file_path, html_cfg)?;
          f.write_all(s.as_bytes())?
        }
      };
      f.write_all(format!("\n{}>", indent_str).as_bytes())?;
    }
    BookItem::Separator => {
      f.write_all(format!("\n\n{indent}+Separator;", indent = indent_str).as_bytes())?;
    }
    BookItem::PartTitle(title) => {
      f.write_all(
        format!(
          "\n\n{indent}+PartTitle{{{title}}}",
          indent = indent_str,
          title = md2satysfi::html2satysfi::escape_inline_text(title)
        )
        .as_bytes(),
      )?;
    }
  };
  Ok(())
}
