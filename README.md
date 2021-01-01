# mdbook-satysfi

[![Build Status][ci-badge]][ci]
[![source badge][source-badge]][source]
[![license badge][license-badge]][license]

[ci]: https://github.com/puripuri2100/mdbook-satysfi/actions?query=workflow%3ARust%20CI
[ci-badge]: https://github.com/puripuri2100/mdbook-satysfi/workflows/Rust%20CI/badge.svg?branch=master
[source]: https://github.com/puripuri2100/mdbook-satysfi
[source-badge]: https://img.shields.io/badge/source-github-blue
[license]: https://github.com/puripuri2100/mdbook-satysfi/blob/master/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue

## Installation

### Requirements

- [Rust](https://www.rust-lang.org/)
- [mdbook](https://github.com/rust-lang-nursery/mdBook)

### Cargo install + Configuration

```sh
cargo install --git "https://github.com/puripuri2100/mdbook-satysfi.git"
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

## Primary Dependencies

`mdbook-satysfi` is built upon some really wonderful projects, including:

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark): Parses the markdown source AST.
- [html_parser](https://github.com/mathiversen/html-parser): Parsing the html tag.

---

(c) 2021 Naoki Kaneko (a.k.a. "puripuri2100")
