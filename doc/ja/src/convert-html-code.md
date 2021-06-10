# HTMLタグを変換する

markdownファイル内にはHTMLタグを直接書くことができます。

ここではそのHTMLタグを処理するための方法を説明します。

ただし、mdbook-satysfiはmarkdownファイルを一旦HTMLに変換した後に処理をする方法を取っているため、意図しないタグの衝突が起こる可能性があることに充分注意して下さい。

book.toml内に以下のようなコードを書いてください。

```
[output.satysfi]
  [output.satysfi.html]
    [output.satysfi.html.hoge]
      command_name="fuga"
      children_type="inline"
      [[output.satysfi.html.hoge.attribute]]
        name = "src"
        type = "link option"
      [[output.satysfi.html.hoge.attribute]]
        name = "title"
        type = "inline"
```

`[output.satysfi.html.<tag-name>]`とすることで、そのタグが実際に書かれていた場合にその下に書いた設定が適用されます。

`div`タグと`span`タグの場合は`class`で指定した名前が使われます。

`command_name`はコマンドを出力する際にどうするかを設定します。デフォルトはタグ名です。

`children_type`は子要素がどうなるかを表しています。書かないと子要素は無視されます。
与えられるのは

- `inline`
- `block`
- `inline list`
- `block list`
- `inline code`
- `block code`

だけです。

`[[output.satysfi.html.<tag-name>.attribute]]`とすることで属性をSATySFiコマンドの引数に変換することができます。

属性はこのテーブルに追加した順に渡されます。

`name`は属性の名前です。

`type`は属性の値がどうなるかを表しています。`* option`のとき、その属性がなかった場合は`None`になります。`option`が付いていないでその属性がなかった場合はその部分は無視されます。

与えられるのは

- `string`
- `link`
- `int`
- `bool`
- `inline`
- `block`

とそれぞれの`option`です。

`link`とは相対パスのことで、画像タグの処理などに使うことを想定しています。与えられた相対パスをその原稿の書かれたmdファイルのパスと結合してSATySFiファイルに書き出します。
