extern crate mdbook;
extern crate pulldown_cmark;

use mdbook::renderer::RenderContext;
use mdbook::BookItem;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path;
//use std::process;

mod md2satysfi;

fn main() {
  let mut stdin = io::stdin();
  let ctx = RenderContext::from_json(&mut stdin).unwrap();

  let destination = &ctx.destination;
  let _ = fs::create_dir_all(&destination);
  let mut f = File::create(&destination.join("main.saty")).unwrap();

  let root = &ctx.source_dir();

  let s: String = ctx
    .book
    .iter()
    .map(|item| bookitme_to_string(item, &root, 1))
    .collect();

  writeln!(
    f,
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
  title = {{}};
  author = {{}};
  show-toc = false;
  show-title = false;
|) '<
{}
>
",
    s
  )
  .unwrap();
}

fn bookitme_to_string(item: &BookItem, root: &path::PathBuf, indent: usize) -> String {
  let indent_str = "  ".repeat(indent);
  match item {
    BookItem::Chapter(ch) => {
      let ch_name = ch.clone().name;
      let path_opt = ch.clone().path;
      let satysfi_code = match path_opt {
        None => String::new(),
        Some(path) => {
          let path = root.join(path);
          fs::read_to_string(&path)
            .map(|text| md2satysfi::make_satysfi_code(text, &path))
            .unwrap_or_else(|_| Ok(String::from("ファイルが見つからなかった")))
            .unwrap_or_else(|_| String::from("SATySFiのコード生成に失敗"))
        }
      };
      let sub_items = ch.clone().sub_items;
      let sub_item_str: String = sub_items
        .iter()
        .map(|item| bookitme_to_string(item, root, indent + 1))
        .collect();
      format!(
        "
{indent}+Chapter{{{name}}} <
{code}
{sub_item_str}
{indent}>\n",
        indent = indent_str,
        name = md2satysfi::escape_inline_text(&ch_name),
        code = satysfi_code,
        sub_item_str = sub_item_str
      )
    }
    BookItem::Separator => format!("{indent}+Separator;\n", indent = indent_str),
    BookItem::PartTitle(title) => {
      format!(
        "{indent}+PartTitle{{{title}}};\n",
        indent = indent_str,
        title = md2satysfi::escape_inline_text(title)
      )
    }
  }
}
