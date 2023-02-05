# クラスファイルを自作する

mdbook-satysfiによって出力されたファイルを処理するためのクラスファイルを自作することができます。


## 必要なコマンドなど

```
(|
  title : inline-text;
  authors : inline-text list;
  description : inline-text option;
  language : string option;
|)
```

というレコード型を一つ目の引数に、`block-text`を二つ目の引数に持ち、`document`型を返す、`document`という名前の関数を提供する必要があります。

`language`フィールドに入力される値は``Some(`ja`)``や``Some(`en`)``といったものです。

実装する必要があるコマンドは以下の通りです。

- `+Chapter: [(int list) option; int; inline-text; block-text]`：章を表すコマンドです。引数はそれぞれ「何番目の章の子や孫になっているのか」・「深さ」・「タイトル」・「中身」です。
- `+PartTitle: [inline-text]`：目次に出力する部の分け目です。引数はタイトルです。
- `+Separator: []`：目次に出力するラインを表します。引数はありません。
- `\strong: [inline-text]`：強調を表します。
- `\emph: [inline-text]`：強調を表します。
- `\strike: [inline-text]`：打ち消し線を表します。
- `+block-quote: [block-text]`：引用を表します。
- `\block-quote: [block-text]`：引用を表します。
- `+heading: [int; inline-text]`：節のタイトルを表します。一つ目の引数は深さです。
- `+p: [inline-text]`：段落を表します。
- `\p: [inline-text]`：段落を表します。
- `+code: [string]`：インラインコードを表します。
- `\code: [string]`：インラインコードを表します。
- `+code-block: [string option; string option; string]`：ブロックコードを表します。一つ目の引数は指定された言語を表し、二つ目の引数はカラーテーマを表します。ここに与えられる値は`book.toml`に`color-theme = "theme name"`と表記されると変更可能です。
- `\code-block: [string option; string option; string]`：同上
- `\href: [string; inline-text]`：リンクを表します。
- `+img: [string; inline-text]`：画像挿入を表します。二つ目の引数はキャプションです。
- `\img: [string; inline-text]`：画像挿入を表します。二つ目の引数はキャプションです。
- `\footnote: [string]`：脚注を出力します。引数はkeyです。
- `+add-footnote: [string; inline-text]`：脚注の中身を登録します。一つ目の引数はkeyで、二つ目の引数が中身です。
- `+rule: []`：線を出力します。
- `\task-list-marker: [bool]`：`true`のときはチェック印付きの四角を出力し、`false`のときは空の四角を出力します。`\item`コマンドの引数内に書かれます。
- `\item: [inline-text]`：箇条書きの際に使用します。
- `\lisgint: [inline-text list]`：箇条書きです。
- `+lisgint: [inline-text list]`：箇条書きです。
- `\enumerate: [int; inline-text list]`：数字付きの箇条書きです。一つ目の引数は「数字がどこから始まるか」を表します。
- `+enumerate: [int; inline-text list]`：数字付きの箇条書きです。一つ目の引数は「数字がどこから始まるか」を表します。
- `+table: [block-text]`：表を出力します。
- `+thead: [block-text]`：表のヘッダーです。
- `+tbody: [block-text]`：表の本体です。
- `+tr: [inline-text list]`：表の横方向を指定します。
- `\th: [string option; inline-text]`：ヘッダーの中で表の一セルを表すコマンドです。一つ目の引数は位置を表します。中央ぞろえの時は``Some(`center`)``・左揃えの時は``Some(`left`)``・右揃えの時は``Some(`right`)``が与えられます。
- `\td: [string option; inline-text]`：`\th`のボディ版です。


## その他のコマンドについて


mdbook-satysfiはマークダウンをHTMLに変換してからSATySFiコードに変換をする形を取っています。そのため、原理的にはHTMLタグ全てに対応するコマンドが必要です。ですが、それらに対応するかどうかはクラスファイル作成者に任されます。

また、`div`タグと`span`タグでは`class`で指定した名前がコマンド名として使われる仕様なため、それらが使用されているマークダウンファイルを処理したい場合はそれに対応するコマンドを実装する必要があるかもしれません。


# クラスファイルの変更の仕方

`[output.satysfi]`以下に

```toml
class-file-name = "class-file-folder/class-file-name"
is-class-file-require = true
```

のように記述します。

`class-file-name`にはクラスファイルの名前を文字列型で入力します。`class-file-name`フィールドが無い、もしくは文字列型ではない場合は`class-mdbook-satysfi/mdbook-satysfi`が使用されます。

`is-class-file-require`には「クラスファイルの読み込みが`@require`かどうか」を真偽値で入力します。`true`の場合は`@require`で読み込まれ、`false`の場合は`@import`で読み込まれます。
デフォルト値は`true`です。

