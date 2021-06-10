# mdbook-satysfi

**mdbook-satysfi**はmarkdownファイルからHTMLファイルで構成された*book*を生成するRust製のソフトウェアである[mdbook](https://rust-lang.github.io/mdBook/index.html)の拡張機能を提供するソフトウェアです。

mdbook-satysfiをインストールした状態でbook.tomlに`[output.satysfi]`という記述を追加して`mdbook build`を実行すると、bookの内容と同じ内容のSATySFiのドキュメントファイルが生成されます。

## 実装

実装はRustで行っています。

リポジトリは[puripuri2100/mdbook-satysfi](https://github.com/puripuri2100/mdbook-satysfi)です。

## 依存するもの

インストールにはRustとCargoを必要とします。

また、生成されたドキュメントファイルを処理するためには[class-mdbook-satysfi](https://github.com/puripuri2100/satysfi-class-mdbook-satysfi)というSATySFi用のライブラリのv0.2.0が必要であり、これのインストールには[satyrographos](https://github.com/na4zagin3/satyrographos)が必要です。
当然のことながら[SATySFi](https://github.com/gfngfn/SATySFi)も必要です。

## インストール方法

RustとCargoをインストールした状態で

```
cargo install mdbook-satysfi
```

と行うことで最新版が手に入ります。

## License

mdbook-satysfiはMITライセンスのもと公開されています。
