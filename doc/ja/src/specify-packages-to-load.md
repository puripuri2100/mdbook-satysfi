# 読み込むパッケージを指定する

book.tomlに以下のような内容を加えることで読み込むパッケージを追加することができます。

`div`タグや`span`タグで独自のタグを作るときに、それに対応するコマンドを用意したいときに便利です。


```
[output.satysfi]
require-packages = ["base/int"]
import-packages = ["local"]
```

`require-packages`ではstring arrayを与えます。指定したパッケージが`@require:`で読み込まれます。

`import-packages`では、指定したパッケージが`@imports:`で読み込まれます。

