# mdbook-satysfi

[![crates.io][crates-badge]][crates]
[![Build Status][ci-badge]][ci]
[![source badge][source-badge]][source]
[![license badge][license-badge]][license]

[crates]: https://crates.io/crates/mdbook-satysfi
[crates-badge]: https://img.shields.io/crates/v/mdbook-satysfi
[ci]: https://github.com/puripuri2100/mdbook-satysfi/actions?query=workflow%3ARust%20CI
[ci-badge]: https://github.com/puripuri2100/mdbook-satysfi/workflows/Rust%20CI/badge.svg?branch=master
[source]: https://github.com/puripuri2100/mdbook-satysfi
[source-badge]: https://img.shields.io/badge/source-github-blue
[license]: https://github.com/puripuri2100/mdbook-satysfi/blob/master/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue


- **[Documentation(ja)](https://puripuri2100.github.io/mdbook-satysfi/ja/)**
- **[Documentation PDF(ja)](https://puripuri2100.github.io/mdbook-satysfi/ja/ja.pdf)**


## Status of Rust Bookshelf

note: The PDF file was generated by manually runnig the `satysfi` command.


- ✅ compiles successfully
- 🍊 compiles but requires a few manual fixes
- ❌ compilation fails/not yet attempted

| Source                              | Generate SATySFi  |  Generate PDF  | Online Version          |
| :---------------------------------- | :---------------: | :------------: | :---------------------- |
|[Rust Programming Language][rust-src]|       ✅         |       🍊       | [HTML][rust-html]       |
|[Mdbook User Guide][mdbook-src]      |       ✅         |       ✅       |[HTML][mdbook-html]      |
|[Rust By Example][example-src]       |       ✅         |       🍊       |[HTML][example-html]     |
|[Edition Guide][edition-src]         |       ✅         |       🍊       |[HTML][edition-html]     |
|[Rustc Book][rustc-src]              |       ✅         |       ❌       |[HTML][rustc-html]       |
|[Cargo Book][cargo-src]              |       ✅         |       🍊       |[HTML][cargo-html]       |
|[Rustdoc Book][rustdoc-src]          |       ✅         |       🍊       |[HTML][rustdoc-html]     |
|[Rust Reference][reference-src]      |       ✅         |       ❌       |[HTML][reference-html]   |
|[Rustonomicon][rustonomicon-src]     |       ✅         |       🍊       |[HTML][rustonomicon-html]|
|[Embedded Rust Book][embedded-src]   |       ✅         |       🍊       |[HTML][embedded-html]    |


[rust-src]: https://github.com/rust-lang/book
[rust-html]: https://doc.rust-lang.org/book/

[mdbook-src]: https://github.com/rust-lang/mdBook/tree/master/guide
[mdbook-html]: https://rust-lang-nursery.github.io/mdBook/

[example-src]: https://github.com/rust-lang/rust-by-example
[example-html]: https://doc.rust-lang.org/stable/rust-by-example/

[edition-src]: https://github.com/rust-lang-nursery/edition-guide
[edition-html]: https://doc.rust-lang.org/edition-guide/index.html

[rustc-src]: https://github.com/rust-lang/rustc-guide
[rustc-html]: https://doc.rust-lang.org/rustc/index.html

[cargo-src]: https://github.com/rust-lang/cargo/tree/master/src/doc
[cargo-html]: https://doc.rust-lang.org/cargo/index.html

[rustdoc-src]: https://github.com/rust-lang/rust/tree/master/src/doc/rustdoc
[rustdoc-html]: https://doc.rust-lang.org/rustdoc/index.html

[reference-src]: https://github.com/rust-lang-nursery/reference
[reference-html]: https://doc.rust-lang.org/reference/index.html

[rustonomicon-src]: https://github.com/rust-lang-nursery/nomicon
[rustonomicon-html]: https://doc.rust-lang.org/nomicon/index.html

[embedded-src]: https://github.com/rust-embedded/book
[embedded-html]: https://rust-embedded.github.io/book/


## Installation

### Requirements

- [Rust](https://www.rust-lang.org/)
- [mdbook](https://github.com/rust-lang-nursery/mdBook)

### Cargo install + Configuration

```sh
cargo install mdbook-satysfi
```

Add the following `toml` configuration to `book.toml`.

```toml
[output.satysfi]
```

The next `mdbook build` command will produce SATySFi file in the `book/satysfi/` directory.

## Uninstallation

To uninstall `mdbook-satysfi`, enter the following in a shell:

```sh
cargo uninstall mdbook-satysfi
```

Then delete the `[output.satysfi]` configuration in `book.toml`:

```diff
- [output.satysfi]
```

## Build PDF file

### Requirements

- [opam](https://opam.ocaml.org/) 2
    - See <https://opam.ocaml.org/doc/Install.html>.
- OCaml 4.11.0 (installed by opam)
- [Satyrographos](https://github.com/na4zagin3/satyrographos)
- [SATySFi](https://github.com/gfngfn/SATySFi) 0.0.5 (installed by Satyrographos)
- [satysfi-class-mdbook-satysfi](https://github.com/puripuri2100/satysfi-class-mdbook-satysfi) 0.2.0 (installed by Satyrographos)

Please run:

```sh
satysfi book/satysfi/main.saty
```


## Primary Dependencies

`mdbook-satysfi` is built upon some really wonderful projects, including:

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark): Parses the markdown source AST.
- [html_parser](https://github.com/mathiversen/html-parser): Parsing the html tag.

---

(c) 2021 Naoki Kaneko (a.k.a. "puripuri2100")
