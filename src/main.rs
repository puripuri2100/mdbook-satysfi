extern crate mdbook;
extern crate pulldown_cmark;

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

  f.write(
    b"@require: stdjabook
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
  title = {};
  author = {};
  show-toc = false;
  show-title = false;
|) '<",
  )
  .unwrap();

  ctx
    .book
    .iter()
    .for_each(|item| write_bookitme(&mut f, item, &root, 1));
  f.write(b">").unwrap();
}

fn write_bookitme(
  f: &mut BufWriter<File>,
  item: &BookItem,
  root: &path::PathBuf,
  indent: usize,
) -> () {
  let indent_str = "  ".repeat(indent);
  match item {
    BookItem::Chapter(ch) => {
      let ch_name = ch.clone().name;
      f.write(
        format!(
          "{indent}+Chapter{{{name}}} <",
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
          let file_text = fs::read_to_string(&path).expect("ファイル読み込みに失敗した");
          md2satysfi::write_satysfi_code(f, file_text, &path).expect("コード生成に失敗した")
        }
      };
      //f.write(satysfi_code.as_bytes()).unwrap();
      let sub_items = ch.clone().sub_items;
      sub_items
        .iter()
        .for_each(|item| write_bookitme(f, item, root, indent + 1));
      f.write(format!("{}>\n", indent_str).as_bytes()).unwrap();
    }
    BookItem::Separator => {
      f.write(format!("{indent}+Separator;\n", indent = indent_str).as_bytes())
        .unwrap();
    }
    BookItem::PartTitle(title) => {
      f.write(
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
