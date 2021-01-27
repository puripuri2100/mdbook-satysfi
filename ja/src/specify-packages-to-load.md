# 読み込むパッケージを指定する

book.tomlに以下のような内容を加えることで読み込むパッケージを追加することができます。

クラスファイルを指定すれば上書きすることも可能です。

```
[output.satysfi]
require-packages = ["base/int"]
import-packages = ["local"]
```

`require-packages`ではstring arrayを与えます。指定したパッケージが`@require:`で読み込まれます。

`import-packages`では、指定したパッケージが`@imports:`で読み込まれます。

