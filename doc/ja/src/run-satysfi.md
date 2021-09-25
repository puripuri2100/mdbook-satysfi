# SATySFiを呼び出す

`[output.satysfi]`テーブルの中に`pdf = true`というコードを入れると、自動でSATySFiが呼び出され、出力したsatyファイルがコンパイルされてPDFファイルが生成されます。

デフォルトでは`pdf = false`で、SATySFiは呼び出されません。

## 高度な設定

```toml
[output.satysfi]
  [output.satysfi.pdf]
    # 各種設定
```

とすることで高度な設定を行うことができます。


### SATySFiの起動コマンドを変更する


以下のように`commands`というkeyに対してSATySFiを起動するためのコマンドを文字列か文字列のリストで与えることでSATySFiの起動コマンドを変更することができます。デフォルトは`satysfi`です。

```toml
[output.satysfi]
  [output.satysfi.pdf]
    commands = ["wsl", "satysfi"]
```

上のように、`["wsl", "satysfi"]`と与えると、SATySFiを起動するために

```
wsl satysfi main.saty
```

というコマンドが実行されます。


このままではLinuxでも`wsl`コマンドが呼ばれてしまうため、OSごとに切り替えることができるようになっています。

```toml
[output.satysfi]
  [output.satysfi.pdf]
    commands = {
      windows = ["wsl", "satysfi"],
      macos = ["satysfi"],
      linux = "satysfi",
      others = "satysfi"
    }
```

のようにすることでOSごとに呼び出しコマンドを切り替えることができます（この場合も、コマンド名として文字列または文字列のリストを与えます）。


現在、選択できるのは

- Windows
- MacOS
- Linux

の3種類のみで、それ以外のOSは`others`でひとまとめにされています。

省略した場合はデフォルトの`satysfi`が起動コマンドとして選択されますので、


```toml
[output.satysfi]
  [output.satysfi.pdf]
    commands = {windows = ["wsl", "satysfi"]}
```

と書けば「Windowsのときには`wsl satysfi <file name>`で起動させ、それ以外のOSでは`satysfi <file name>`で起動させると設定できます。



### SATySFiに与えるオプションの変更

SATySFiに与えるオプションを変更することができます。

- 変更するためにTOMLに与える時に使うキーの名前
- 対応して変更されるSATySFiのオプション
- キーに対応して与える値の種類
- デフォルトの値

の一覧のリストは以下のようになっています。


なお、`text-mode-configs`で文字列を与える場合は、

```toml
[output.satysfi]
  [output.satysfi.pdf]
    text-mode-cofings = "tex,latex"
```

のようにカンマ区切りで文字列を与えてください。

文字列のリストで与える場合はそのまま

```toml
[output.satysfi]
  [output.satysfi.pdf]
    text-mode-cofings = ["tex", "latex"]
```

のように与えてください。


また、`config-path`に与えるパスは「“{build-dir}/satysfi”フォルダ」からの相対パスにしてください。


`output-file-name`に与えるファイルのパスは`config-path`同様に、「“{build-dir}/satysfi”フォルダ」からの相対パスにしてください。


|キー|SATySFiのオプション|値|デフォルト|
|:-------:|:-------:|:----------:|:------------:|
|`is-bytecomp`|`--bytecomp`|真偽値|false|
|`is-type-check-only`|`--type-check-only`|真偽値|false|
|`is-debug-show-bbox`|`--debug-show-bbox`|真偽値|false|
|`is-debug-show-space`|`--debug-show-space`|真偽値|false|
|`is-debug-show-block-bbox`|`--debug-show-block-bbox`|真偽値|false|
|`is-debug-show-block-space`|`--debug-show-block-space`|真偽値|false|
|`is-debug-show-overfull`|`--debug-show-overfull`|真偽値|false|
|`is-full-path`|`--full-path`|真偽値|false|
|`is-show-fonts`|`--show-fonts`|真偽値|false|
|`is-no-default-config`|`--no-default-config`|真偽値|false|
|`config-path`|`--config`|文字列|オプションがつかない|
|`page-number-limit`|`--page-number-limt`|数値|オプションがつかない|
|`output-file-name`|`--output`|文字列|オプションがつかない|
|`text-mode-configs`|`--text-mode`|文字列・文字列のリスト|オプションがつかない|

