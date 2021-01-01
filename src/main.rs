extern crate mdbook;
extern crate pulldown_cmark;

use mdbook::config::Config;
use mdbook::renderer::RenderContext;
use mdbook::BookItem;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path;
//use std::process;

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

  let cfg = &ctx.config;
  let book = &cfg.book;

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

  f.write_all(
    format!(
      "@require: stdjabook
@require: annot
@require: itemize
@require: code
@require: vdecoset
@require: easytable/easytable

open EasyTableAlias


let add-fil ib = ib ++ inline-fil

let-block ctx +Chapter title bt =
  let title-ib = read-inline ctx title in
  let title-bb = line-break true false ctx (add-fil title-ib) in
  let main-bb = read-block ctx bt in
  main-bb


let-block ctx +heading n title =
  let font-size =
    match n with
    | 1 -> 25pt
    | 2 -> 20pt
    | 3 -> 18pt
    | _ -> 15pt
  in
  let ctx =
    ctx
    |> set-font-size font-size
    |> set-font Kana           (`ipaexg`    , 0.88, 0.)
    |> set-font HanIdeographic (`ipaexg`    , 0.88, 0.)
    |> set-font Latin          (`lmsans`    , 1.0 , 0.)
  in
  title
  |> read-inline ctx
  |> add-fil
  |> line-break true false ctx


let-block ctx +block-quote bt =
  let space = (20pt, 0pt, 0pt, 0pt) in
  let deco = VDecoSet.empty in
  block-frame-breakable ctx space deco (fun ctx -> read-block ctx bt)


let-block ctx +Separator = block-nil


let-block ctx +PartTitle title =
  let font-size = 25pt in
  let ctx =
    ctx
    |> set-font-size font-size
    |> set-font Kana           (`ipaexg`    , 0.88, 0.)
    |> set-font HanIdeographic (`ipaexg`    , 0.88, 0.)
    |> set-font Latin          (`lmsans`    , 1.0 , 0.)
  in
  title
  |> read-inline ctx
  |> add-fil
  |> line-break true false ctx


let-inline ctx \\strong it =
  let ctx =
    ctx
    |> set-font Kana           (`ipaexg`    , 0.88, 0.)
    |> set-font HanIdeographic (`ipaexg`    , 0.88, 0.)
    |> set-font Latin          (`lmsans`    , 1.0 , 0.)
  in
  read-inline ctx it


in

document (|
  title = {{{title}}};
  author = {{{author}}};
  show-toc = false;
  show-title = false;
|) '<",
      title = title,
      author = author
    )
    .as_bytes(),
  )
  .unwrap();

  ctx
    .book
    .iter()
    .for_each(|item| write_bookitme(&mut f, item, &root));
  f.write_all(b">").unwrap();
}

fn write_bookitme(f: &mut BufWriter<File>, item: &BookItem, root: &path::PathBuf) {
  let indent_str = "  ".to_string();
  match item {
    BookItem::Chapter(ch) => {
      let ch_name = ch.clone().name;
      f.write_all(
        format!(
          "{indent}+Chapter{{{name}}} <\n",
          indent = indent_str,
          name = md2satysfi::escape_inline_text(&ch_name)
        )
        .as_bytes(),
      )
      .unwrap();
      let path_opt = ch.clone().path;
      match path_opt {
        None => (),
        Some(path) => {
          let path = root.join(path);
          let s = md2satysfi::md_to_satysfi_code(ch.clone().content, &path).unwrap();
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
