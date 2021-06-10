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

設定の中で使うことのできるフィールドと内容です：

- `is-bytecomp`：真偽値を与えます。`--bytecomp`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-type-check-only`：真偽値を与えます。`--type-check-only`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-debug-show-bbox`：真偽値を与えます。`--debug-show-bbox`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-debug-show-space`：真偽値を与えます。`--debug-show-space`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-debug-show-block-bbox`：真偽値を与えます。`--debug-show-block-bbox`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-debug-show-block-space`：真偽値を与えます。`--debug-show-block-space`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-debug-show-overfull`：真偽値を与えます。`--debug-debug-show-overfull`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-full-path`：真偽値を与えます。`--full-path`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-show-fonts`：真偽値を与えます。`--show-fonts`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `is-no-default-config`：真偽値を与えます。`--no-default-config`オプションを与えるかどうかを選択します。デフォルトはfalseです。
- `config-path`：`--config`オプションに与える文字列を与えます。絶対pathに変換された後に`--config`オプションに与えられます。デフォルトではオプションそのものが与えられません。
- `page-number-limit`：数値を与えます。`--page-number-limit`オプションに与える数値です。デフォルトではオプションそのものが与えられません。
- `output-file-name`：文字列を与えます。`--output`オプションに与える数値です。デフォルトではオプションそのものが与えられません。
- `text-mode-configs`：`--text-mode`オプションに与える文字列を与えます。デフォルトではオプションそのものが与えられません。

