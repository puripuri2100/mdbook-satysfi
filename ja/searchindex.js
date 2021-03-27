Object.assign(window.search, {"doc_urls":["mdbook-satysfi.html#mdbook-satysfi","mdbook-satysfi.html#実装","mdbook-satysfi.html#依存するもの","mdbook-satysfi.html#インストール方法","mdbook-satysfi.html#license","specify-packages-to-load.html#読み込むパッケージを指定する","convert-html-code.html#htmlタグを変換する"],"index":{"documentStore":{"docInfo":{"0":{"body":9,"breadcrumbs":4,"title":2},"1":{"body":3,"breadcrumbs":2,"title":0},"2":{"body":7,"breadcrumbs":2,"title":0},"3":{"body":5,"breadcrumbs":2,"title":0},"4":{"body":2,"breadcrumbs":3,"title":1},"5":{"body":13,"breadcrumbs":0,"title":0},"6":{"body":48,"breadcrumbs":2,"title":1}},"docs":{"0":{"body":"mdbook-satysfi はmarkdownファイルからHTMLファイルで構成された book を生成するRust製のソフトウェアである mdbook の拡張機能を提供するソフトウェアです。 mdbook-satysfiをインストールした状態でbook.tomlに[output.satysfi]という記述を追加してmdbook buildを実行すると、bookの内容と同じ内容のSATySFiのドキュメントファイルが生成されます。","breadcrumbs":"mdbook-satysfi » mdbook-satysfi","id":"0","title":"mdbook-satysfi"},"1":{"body":"実装はRustで行っています。 リポジトリは puripuri2100/mdbook-satysfi です。","breadcrumbs":"mdbook-satysfi » 実装","id":"1","title":"実装"},"2":{"body":"インストールにはRustとCargoを必要とします。 また、生成されたドキュメントファイルを処理するためには class-mdbook-satysfi というSATySFi用のライブラリのv0.2.0が必要であり、これのインストールには satyrographos が必要です。 当然のことながら SATySFi も必要です。","breadcrumbs":"mdbook-satysfi » 依存するもの","id":"2","title":"依存するもの"},"3":{"body":"RustとCargoをインストールした状態で cargo install mdbook-satysfi と行うことで最新版が手に入ります。","breadcrumbs":"mdbook-satysfi » インストール方法","id":"3","title":"インストール方法"},"4":{"body":"mdbook-satysfiはMITライセンスのもと公開されています。","breadcrumbs":"mdbook-satysfi » License","id":"4","title":"License"},"5":{"body":"book.tomlに以下のような内容を加えることで読み込むパッケージを追加することができます。 クラスファイルを指定すれば上書きすることも可能です。 [output.satysfi]\nrequire-packages = [\"base/int\"]\nimport-packages = [\"local\"] require-packagesではstring arrayを与えます。指定したパッケージが@require:で読み込まれます。 import-packagesでは、指定したパッケージが@imports:で読み込まれます。","breadcrumbs":"読み込むパッケージを指定する » 読み込むパッケージを指定する","id":"5","title":"読み込むパッケージを指定する"},"6":{"body":"markdownファイル内にはHTMLタグを直接書くことができます。 ここではそのHTMLタグを処理するための方法を説明します。 ただし、mdbook-satysfiはmarkdownファイルを一旦HTMLに変換した後に処理をする方法を取っているため、意図しないタグの衝突が起こる可能性があることに充分注意して下さい。 book.toml内に以下のようなコードを書いてください。 [output.satysfi] [output.satysfi.html] [output.satysfi.html.hoge] command_name=\"fuga\" children_type=\"inline\" [[output.satysfi.html.hoge.attribute]] name = \"src\" type = \"link option\" [[output.satysfi.html.hoge.attribute]] name = \"title\" type = \"inline\" [output.satysfi.html.<tag-name>]とすることで、そのタグが実際に書かれていた場合にその下に書いた設定が適用されます。 command_nameはコマンドを出力する際にどうするかを設定します。デフォルトはタグ名です。 children_typeは子要素がどうなるかを表しています。書かないと子要素は無視されます。 与えられるのは inline block inline list block list inline code block code だけです。 [[output.satysfi.html.<tag-name>.attribute]]とすることで属性をSATySFiコマンドの引数に変換することができます。 属性はこのテーブルに追加した順に渡されます。 nameは属性の名前です。 typeは属性の値がどうなるかを表しています。* optionのとき、その属性がなかった場合はNoneになります。optionが付いていないでその属性がなかった場合はその部分は無視されます。 与えられるのは string link int bool inline block とそれぞれのoptionです。 linkとは相対パスのことで、画像タグの処理などに使うことを想定しています。与えられた相対パスをその原稿の書かれたmdファイルのパスと結合してSATySFiファイルに書き出します。","breadcrumbs":"HTMLタグを変換する » HTMLタグを変換する","id":"6","title":"HTMLタグを変換する"}},"length":7,"save":true},"fields":["title","body","breadcrumbs"],"index":{"body":{"root":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"y":{"df":0,"docs":{},"を":{"df":0,"docs":{},"与":{"df":0,"docs":{},"え":{"df":0,"docs":{},"ま":{"df":0,"docs":{},"す":{"df":0,"docs":{},"。":{"df":0,"docs":{},"指":{"df":0,"docs":{},"定":{"df":0,"docs":{},"し":{"df":0,"docs":{},"た":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ッ":{"df":0,"docs":{},"ケ":{"df":0,"docs":{},"ー":{"df":0,"docs":{},"ジ":{"df":0,"docs":{},"が":{"@":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"q":{"df":0,"docs":{},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}}}}}},"df":0,"docs":{}}}},"b":{"a":{"df":0,"docs":{},"s":{"df":0,"docs":{},"e":{"/":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"c":{"df":0,"docs":{},"k":{"df":1,"docs":{"6":{"tf":2.0}}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{".":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":2,"docs":{"5":{"tf":1.0},"6":{"tf":1.0}}}}}}},"df":1,"docs":{"0":{"tf":1.0}}},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":0,"docs":{},"を":{"df":0,"docs":{},"実":{"df":0,"docs":{},"行":{"df":0,"docs":{},"す":{"df":0,"docs":{},"る":{"df":0,"docs":{},"と":{"df":0,"docs":{},"、":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":0,"docs":{},"の":{"df":0,"docs":{},"内":{"df":0,"docs":{},"容":{"df":0,"docs":{},"と":{"df":0,"docs":{},"同":{"df":0,"docs":{},"じ":{"df":0,"docs":{},"内":{"df":0,"docs":{},"容":{"df":0,"docs":{},"の":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}},"df":0,"docs":{}}}}}}}}},"df":0,"docs":{}}}}},"c":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"g":{"df":0,"docs":{},"o":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"df":0,"docs":{},"h":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"_":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"p":{"df":1,"docs":{"6":{"tf":1.0}},"e":{"=":{"\"":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{}}}},"l":{"a":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"2":{"tf":1.0}}}}},"df":0,"docs":{}},"o":{"d":{"df":0,"docs":{},"e":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"m":{"a":{"df":0,"docs":{},"n":{"d":{"_":{"df":0,"docs":{},"n":{"a":{"df":0,"docs":{},"m":{"df":1,"docs":{"6":{"tf":1.0}},"e":{"=":{"\"":{"df":0,"docs":{},"f":{"df":0,"docs":{},"u":{"df":0,"docs":{},"g":{"a":{"df":1,"docs":{"6":{"tf":1.0}}},"df":0,"docs":{}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}}}}},"n":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":1,"docs":{"6":{"tf":2.23606797749979}}}}},"s":{"df":0,"docs":{},"t":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"3":{"tf":1.0}}}},"df":0,"docs":{}}},"t":{"df":1,"docs":{"6":{"tf":1.0}}}}},"l":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"s":{"df":1,"docs":{"4":{"tf":1.0}}}}}},"df":0,"docs":{},"n":{"df":0,"docs":{},"k":{"df":1,"docs":{"6":{"tf":1.4142135623730951}},"と":{"df":0,"docs":{},"は":{"df":0,"docs":{},"相":{"df":0,"docs":{},"対":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ス":{"df":0,"docs":{},"の":{"df":0,"docs":{},"こ":{"df":0,"docs":{},"と":{"df":0,"docs":{},"で":{"df":0,"docs":{},"、":{"df":0,"docs":{},"画":{"df":0,"docs":{},"像":{"df":0,"docs":{},"タ":{"df":0,"docs":{},"グ":{"df":0,"docs":{},"の":{"df":0,"docs":{},"処":{"df":0,"docs":{},"理":{"df":0,"docs":{},"な":{"df":0,"docs":{},"ど":{"df":0,"docs":{},"に":{"df":0,"docs":{},"使":{"df":0,"docs":{},"う":{"df":0,"docs":{},"こ":{"df":0,"docs":{},"と":{"df":0,"docs":{},"を":{"df":0,"docs":{},"想":{"df":0,"docs":{},"定":{"df":0,"docs":{},"し":{"df":0,"docs":{},"て":{"df":0,"docs":{},"い":{"df":0,"docs":{},"ま":{"df":0,"docs":{},"す":{"df":0,"docs":{},"。":{"df":0,"docs":{},"与":{"df":0,"docs":{},"え":{"df":0,"docs":{},"ら":{"df":0,"docs":{},"れ":{"df":0,"docs":{},"た":{"df":0,"docs":{},"相":{"df":0,"docs":{},"対":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ス":{"df":0,"docs":{},"を":{"df":0,"docs":{},"そ":{"df":0,"docs":{},"の":{"df":0,"docs":{},"原":{"df":0,"docs":{},"稿":{"df":0,"docs":{},"の":{"df":0,"docs":{},"書":{"df":0,"docs":{},"か":{"df":0,"docs":{},"れ":{"df":0,"docs":{},"た":{"df":0,"docs":{},"m":{"d":{"df":0,"docs":{},"フ":{"df":0,"docs":{},"ァ":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ル":{"df":0,"docs":{},"の":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ス":{"df":0,"docs":{},"と":{"df":0,"docs":{},"結":{"df":0,"docs":{},"合":{"df":0,"docs":{},"し":{"df":0,"docs":{},"て":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}}},"o":{"c":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}}},"m":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"d":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":0,"docs":{},"n":{"df":0,"docs":{},"フ":{"df":0,"docs":{},"ァ":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ル":{"df":0,"docs":{},"か":{"df":0,"docs":{},"ら":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"内":{"df":0,"docs":{},"に":{"df":0,"docs":{},"は":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}}}}}}}}}}},"df":0,"docs":{}}}},"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":5,"docs":{"0":{"tf":2.0},"2":{"tf":1.0},"3":{"tf":1.0},"4":{"tf":1.0},"6":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"n":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{">":{".":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"]":{"]":{"df":0,"docs":{},"と":{"df":0,"docs":{},"す":{"df":0,"docs":{},"る":{"df":0,"docs":{},"こ":{"df":0,"docs":{},"と":{"df":0,"docs":{},"で":{"df":0,"docs":{},"属":{"df":0,"docs":{},"性":{"df":0,"docs":{},"を":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}}}}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":1,"docs":{"6":{"tf":2.0}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"6":{"tf":1.4142135623730951}},"の":{"df":0,"docs":{},"と":{"df":0,"docs":{},"き":{"df":0,"docs":{},"、":{"df":0,"docs":{},"そ":{"df":0,"docs":{},"の":{"df":0,"docs":{},"属":{"df":0,"docs":{},"性":{"df":0,"docs":{},"が":{"df":0,"docs":{},"な":{"df":0,"docs":{},"か":{"df":0,"docs":{},"っ":{"df":0,"docs":{},"た":{"df":0,"docs":{},"場":{"df":0,"docs":{},"合":{"df":0,"docs":{},"は":{"df":0,"docs":{},"n":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":0,"docs":{},"e":{"df":0,"docs":{},"に":{"df":0,"docs":{},"な":{"df":0,"docs":{},"り":{"df":0,"docs":{},"ま":{"df":0,"docs":{},"す":{"df":0,"docs":{},"。":{"df":0,"docs":{},"o":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"p":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{".":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{".":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{".":{"<":{"df":0,"docs":{},"t":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}},"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"g":{"df":1,"docs":{"6":{"tf":1.0}},"e":{".":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"df":1,"docs":{"6":{"tf":1.0}}}}}}},"df":2,"docs":{"5":{"tf":1.0},"6":{"tf":1.0}}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}}}},"p":{"a":{"c":{"df":0,"docs":{},"k":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"5":{"tf":1.4142135623730951}},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"で":{"df":0,"docs":{},"は":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.0}}}}},"、":{"df":0,"docs":{},"指":{"df":0,"docs":{},"定":{"df":0,"docs":{},"し":{"df":0,"docs":{},"た":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ッ":{"df":0,"docs":{},"ケ":{"df":0,"docs":{},"ー":{"df":0,"docs":{},"ジ":{"df":0,"docs":{},"が":{"@":{"df":0,"docs":{},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"2":{"1":{"0":{"0":{"/":{"df":0,"docs":{},"m":{"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"q":{"df":0,"docs":{},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}}}}},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}},"と":{"c":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"g":{"df":0,"docs":{},"o":{"df":2,"docs":{"2":{"tf":1.0},"3":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":1,"docs":{"2":{"tf":1.0}}}}}},"df":0,"docs":{}}}}},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":4,"docs":{"0":{"tf":1.4142135623730951},"1":{"tf":1.0},"2":{"tf":1.4142135623730951},"3":{"tf":1.0}},"は":{"df":0,"docs":{},"m":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"d":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":0,"docs":{},"n":{"df":0,"docs":{},"フ":{"df":0,"docs":{},"ァ":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ル":{"df":0,"docs":{},"を":{"df":0,"docs":{},"一":{"df":0,"docs":{},"旦":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}}}}}}}}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"4":{"tf":1.0}}}}}},"を":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ン":{"df":0,"docs":{},"ス":{"df":0,"docs":{},"ト":{"df":0,"docs":{},"ー":{"df":0,"docs":{},"ル":{"df":0,"docs":{},"し":{"df":0,"docs":{},"た":{"df":0,"docs":{},"状":{"df":0,"docs":{},"態":{"df":0,"docs":{},"で":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{".":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":0,"docs":{},"に":{"[":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"p":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{".":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"]":{"df":0,"docs":{},"と":{"df":0,"docs":{},"い":{"df":0,"docs":{},"う":{"df":0,"docs":{},"記":{"df":0,"docs":{},"述":{"df":0,"docs":{},"を":{"df":0,"docs":{},"追":{"df":0,"docs":{},"加":{"df":0,"docs":{},"し":{"df":0,"docs":{},"て":{"df":0,"docs":{},"m":{"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}}}}}},"df":0,"docs":{}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}}}}},"df":0,"docs":{}}}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{}}}}}}}}}}}}},"用":{"df":0,"docs":{},"の":{"df":0,"docs":{},"ラ":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ブ":{"df":0,"docs":{},"ラ":{"df":0,"docs":{},"リ":{"df":0,"docs":{},"の":{"df":0,"docs":{},"v":{"0":{".":{"2":{".":{"0":{"df":1,"docs":{"2":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}}}}}}}}},"df":0,"docs":{},"r":{"c":{"df":1,"docs":{"6":{"tf":1.0}}},"df":0,"docs":{}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}},"y":{"df":0,"docs":{},"p":{"df":0,"docs":{},"e":{"df":1,"docs":{"6":{"tf":1.7320508075688772}}}}}}}},"breadcrumbs":{"root":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"y":{"df":0,"docs":{},"を":{"df":0,"docs":{},"与":{"df":0,"docs":{},"え":{"df":0,"docs":{},"ま":{"df":0,"docs":{},"す":{"df":0,"docs":{},"。":{"df":0,"docs":{},"指":{"df":0,"docs":{},"定":{"df":0,"docs":{},"し":{"df":0,"docs":{},"た":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ッ":{"df":0,"docs":{},"ケ":{"df":0,"docs":{},"ー":{"df":0,"docs":{},"ジ":{"df":0,"docs":{},"が":{"@":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"q":{"df":0,"docs":{},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}}}}}},"df":0,"docs":{}}}},"b":{"a":{"df":0,"docs":{},"s":{"df":0,"docs":{},"e":{"/":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"l":{"df":0,"docs":{},"o":{"c":{"df":0,"docs":{},"k":{"df":1,"docs":{"6":{"tf":2.0}}}},"df":0,"docs":{}}},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{".":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":2,"docs":{"5":{"tf":1.0},"6":{"tf":1.0}}}}}}},"df":1,"docs":{"0":{"tf":1.0}}},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":0,"docs":{},"を":{"df":0,"docs":{},"実":{"df":0,"docs":{},"行":{"df":0,"docs":{},"す":{"df":0,"docs":{},"る":{"df":0,"docs":{},"と":{"df":0,"docs":{},"、":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":0,"docs":{},"の":{"df":0,"docs":{},"内":{"df":0,"docs":{},"容":{"df":0,"docs":{},"と":{"df":0,"docs":{},"同":{"df":0,"docs":{},"じ":{"df":0,"docs":{},"内":{"df":0,"docs":{},"容":{"df":0,"docs":{},"の":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}},"df":0,"docs":{}}}}}}}}},"df":0,"docs":{}}}}},"c":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"g":{"df":0,"docs":{},"o":{"df":1,"docs":{"3":{"tf":1.0}}}}}},"df":0,"docs":{},"h":{"df":0,"docs":{},"i":{"df":0,"docs":{},"l":{"d":{"df":0,"docs":{},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"_":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"p":{"df":1,"docs":{"6":{"tf":1.0}},"e":{"=":{"\"":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{}}}},"l":{"a":{"df":0,"docs":{},"s":{"df":0,"docs":{},"s":{"df":1,"docs":{"2":{"tf":1.0}}}}},"df":0,"docs":{}},"o":{"d":{"df":0,"docs":{},"e":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}},"df":0,"docs":{},"m":{"df":0,"docs":{},"m":{"a":{"df":0,"docs":{},"n":{"d":{"_":{"df":0,"docs":{},"n":{"a":{"df":0,"docs":{},"m":{"df":1,"docs":{"6":{"tf":1.0}},"e":{"=":{"\"":{"df":0,"docs":{},"f":{"df":0,"docs":{},"u":{"df":0,"docs":{},"g":{"a":{"df":1,"docs":{"6":{"tf":1.0}}},"df":0,"docs":{}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}}}}},"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":2.0}}}}}},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}}}}},"n":{"df":0,"docs":{},"l":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":1,"docs":{"6":{"tf":2.23606797749979}}}}},"s":{"df":0,"docs":{},"t":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"3":{"tf":1.0}}}},"df":0,"docs":{}}},"t":{"df":1,"docs":{"6":{"tf":1.0}}}}},"l":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"s":{"df":1,"docs":{"4":{"tf":1.4142135623730951}}}}}},"df":0,"docs":{},"n":{"df":0,"docs":{},"k":{"df":1,"docs":{"6":{"tf":1.4142135623730951}},"と":{"df":0,"docs":{},"は":{"df":0,"docs":{},"相":{"df":0,"docs":{},"対":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ス":{"df":0,"docs":{},"の":{"df":0,"docs":{},"こ":{"df":0,"docs":{},"と":{"df":0,"docs":{},"で":{"df":0,"docs":{},"、":{"df":0,"docs":{},"画":{"df":0,"docs":{},"像":{"df":0,"docs":{},"タ":{"df":0,"docs":{},"グ":{"df":0,"docs":{},"の":{"df":0,"docs":{},"処":{"df":0,"docs":{},"理":{"df":0,"docs":{},"な":{"df":0,"docs":{},"ど":{"df":0,"docs":{},"に":{"df":0,"docs":{},"使":{"df":0,"docs":{},"う":{"df":0,"docs":{},"こ":{"df":0,"docs":{},"と":{"df":0,"docs":{},"を":{"df":0,"docs":{},"想":{"df":0,"docs":{},"定":{"df":0,"docs":{},"し":{"df":0,"docs":{},"て":{"df":0,"docs":{},"い":{"df":0,"docs":{},"ま":{"df":0,"docs":{},"す":{"df":0,"docs":{},"。":{"df":0,"docs":{},"与":{"df":0,"docs":{},"え":{"df":0,"docs":{},"ら":{"df":0,"docs":{},"れ":{"df":0,"docs":{},"た":{"df":0,"docs":{},"相":{"df":0,"docs":{},"対":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ス":{"df":0,"docs":{},"を":{"df":0,"docs":{},"そ":{"df":0,"docs":{},"の":{"df":0,"docs":{},"原":{"df":0,"docs":{},"稿":{"df":0,"docs":{},"の":{"df":0,"docs":{},"書":{"df":0,"docs":{},"か":{"df":0,"docs":{},"れ":{"df":0,"docs":{},"た":{"df":0,"docs":{},"m":{"d":{"df":0,"docs":{},"フ":{"df":0,"docs":{},"ァ":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ル":{"df":0,"docs":{},"の":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ス":{"df":0,"docs":{},"と":{"df":0,"docs":{},"結":{"df":0,"docs":{},"合":{"df":0,"docs":{},"し":{"df":0,"docs":{},"て":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}},"s":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}}},"o":{"c":{"a":{"df":0,"docs":{},"l":{"df":1,"docs":{"5":{"tf":1.0}}}},"df":0,"docs":{}},"df":0,"docs":{}}},"m":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"d":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":0,"docs":{},"n":{"df":0,"docs":{},"フ":{"df":0,"docs":{},"ァ":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ル":{"df":0,"docs":{},"か":{"df":0,"docs":{},"ら":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"内":{"df":0,"docs":{},"に":{"df":0,"docs":{},"は":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}}}}}}}}}}},"df":0,"docs":{}}}},"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":6,"docs":{"0":{"tf":2.449489742783178},"1":{"tf":1.0},"2":{"tf":1.4142135623730951},"3":{"tf":1.4142135623730951},"4":{"tf":1.4142135623730951},"6":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"n":{"a":{"df":0,"docs":{},"m":{"df":0,"docs":{},"e":{">":{".":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"e":{"]":{"]":{"df":0,"docs":{},"と":{"df":0,"docs":{},"す":{"df":0,"docs":{},"る":{"df":0,"docs":{},"こ":{"df":0,"docs":{},"と":{"df":0,"docs":{},"で":{"df":0,"docs":{},"属":{"df":0,"docs":{},"性":{"df":0,"docs":{},"を":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}}}}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":1,"docs":{"6":{"tf":2.0}}}}},"df":0,"docs":{}},"o":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":1,"docs":{"6":{"tf":1.4142135623730951}},"の":{"df":0,"docs":{},"と":{"df":0,"docs":{},"き":{"df":0,"docs":{},"、":{"df":0,"docs":{},"そ":{"df":0,"docs":{},"の":{"df":0,"docs":{},"属":{"df":0,"docs":{},"性":{"df":0,"docs":{},"が":{"df":0,"docs":{},"な":{"df":0,"docs":{},"か":{"df":0,"docs":{},"っ":{"df":0,"docs":{},"た":{"df":0,"docs":{},"場":{"df":0,"docs":{},"合":{"df":0,"docs":{},"は":{"df":0,"docs":{},"n":{"df":0,"docs":{},"o":{"df":0,"docs":{},"n":{"df":0,"docs":{},"e":{"df":0,"docs":{},"に":{"df":0,"docs":{},"な":{"df":0,"docs":{},"り":{"df":0,"docs":{},"ま":{"df":0,"docs":{},"す":{"df":0,"docs":{},"。":{"df":0,"docs":{},"o":{"df":0,"docs":{},"p":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"p":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{".":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{".":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{".":{"<":{"df":0,"docs":{},"t":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}},"df":0,"docs":{}}},"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":0,"docs":{},"g":{"df":1,"docs":{"6":{"tf":1.0}},"e":{".":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"b":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":1,"docs":{"6":{"tf":1.4142135623730951}}}}},"df":0,"docs":{}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"df":1,"docs":{"6":{"tf":1.0}}}}}}},"df":2,"docs":{"5":{"tf":1.0},"6":{"tf":1.0}}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}}}},"p":{"a":{"c":{"df":0,"docs":{},"k":{"a":{"df":0,"docs":{},"g":{"df":1,"docs":{"5":{"tf":1.4142135623730951}},"e":{"df":0,"docs":{},"s":{"df":0,"docs":{},"で":{"df":0,"docs":{},"は":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.0}}}}},"、":{"df":0,"docs":{},"指":{"df":0,"docs":{},"定":{"df":0,"docs":{},"し":{"df":0,"docs":{},"た":{"df":0,"docs":{},"パ":{"df":0,"docs":{},"ッ":{"df":0,"docs":{},"ケ":{"df":0,"docs":{},"ー":{"df":0,"docs":{},"ジ":{"df":0,"docs":{},"が":{"@":{"df":0,"docs":{},"i":{"df":0,"docs":{},"m":{"df":0,"docs":{},"p":{"df":0,"docs":{},"o":{"df":0,"docs":{},"r":{"df":0,"docs":{},"t":{"df":1,"docs":{"5":{"tf":1.0}}}}}}}}},"df":0,"docs":{}}}}}}}}}}}}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"p":{"df":0,"docs":{},"u":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"2":{"1":{"0":{"0":{"/":{"df":0,"docs":{},"m":{"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"1":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}},"r":{"df":0,"docs":{},"e":{"df":0,"docs":{},"q":{"df":0,"docs":{},"u":{"df":0,"docs":{},"i":{"df":0,"docs":{},"r":{"df":1,"docs":{"5":{"tf":1.4142135623730951}}}}}}},"u":{"df":0,"docs":{},"s":{"df":0,"docs":{},"t":{"df":2,"docs":{"0":{"tf":1.0},"1":{"tf":1.0}},"と":{"c":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"g":{"df":0,"docs":{},"o":{"df":2,"docs":{"2":{"tf":1.0},"3":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"r":{"df":0,"docs":{},"o":{"df":0,"docs":{},"g":{"df":0,"docs":{},"r":{"a":{"df":0,"docs":{},"p":{"df":0,"docs":{},"h":{"df":0,"docs":{},"o":{"df":1,"docs":{"2":{"tf":1.0}}}}}},"df":0,"docs":{}}}}},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":5,"docs":{"0":{"tf":2.0},"1":{"tf":1.4142135623730951},"2":{"tf":1.7320508075688772},"3":{"tf":1.4142135623730951},"4":{"tf":1.0}},"は":{"df":0,"docs":{},"m":{"a":{"df":0,"docs":{},"r":{"df":0,"docs":{},"k":{"d":{"df":0,"docs":{},"o":{"df":0,"docs":{},"w":{"df":0,"docs":{},"n":{"df":0,"docs":{},"フ":{"df":0,"docs":{},"ァ":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ル":{"df":0,"docs":{},"を":{"df":0,"docs":{},"一":{"df":0,"docs":{},"旦":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}}}}}}}}}}},"df":0,"docs":{}}}},"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":1,"docs":{"4":{"tf":1.0}}}}}},"を":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ン":{"df":0,"docs":{},"ス":{"df":0,"docs":{},"ト":{"df":0,"docs":{},"ー":{"df":0,"docs":{},"ル":{"df":0,"docs":{},"し":{"df":0,"docs":{},"た":{"df":0,"docs":{},"状":{"df":0,"docs":{},"態":{"df":0,"docs":{},"で":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{".":{"df":0,"docs":{},"t":{"df":0,"docs":{},"o":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":0,"docs":{},"に":{"[":{"df":0,"docs":{},"o":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{"df":0,"docs":{},"p":{"df":0,"docs":{},"u":{"df":0,"docs":{},"t":{".":{"df":0,"docs":{},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"]":{"df":0,"docs":{},"と":{"df":0,"docs":{},"い":{"df":0,"docs":{},"う":{"df":0,"docs":{},"記":{"df":0,"docs":{},"述":{"df":0,"docs":{},"を":{"df":0,"docs":{},"追":{"df":0,"docs":{},"加":{"df":0,"docs":{},"し":{"df":0,"docs":{},"て":{"df":0,"docs":{},"m":{"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}}}}}},"df":0,"docs":{}}}}}}},"df":0,"docs":{}}},"df":0,"docs":{}}}}}}}},"df":0,"docs":{}}}}}}},"df":0,"docs":{}}}}},"df":0,"docs":{}}}}}}}}}}}}},"用":{"df":0,"docs":{},"の":{"df":0,"docs":{},"ラ":{"df":0,"docs":{},"イ":{"df":0,"docs":{},"ブ":{"df":0,"docs":{},"ラ":{"df":0,"docs":{},"リ":{"df":0,"docs":{},"の":{"df":0,"docs":{},"v":{"0":{".":{"2":{".":{"0":{"df":1,"docs":{"2":{"tf":1.0}}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}},"df":0,"docs":{}}}}}}}}}}}}}}}},"df":0,"docs":{},"r":{"c":{"df":1,"docs":{"6":{"tf":1.0}}},"df":0,"docs":{}},"t":{"df":0,"docs":{},"r":{"df":0,"docs":{},"i":{"df":0,"docs":{},"n":{"df":0,"docs":{},"g":{"df":1,"docs":{"6":{"tf":1.0}}}}}}}},"t":{"df":0,"docs":{},"i":{"df":0,"docs":{},"t":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}},"y":{"df":0,"docs":{},"p":{"df":0,"docs":{},"e":{"df":1,"docs":{"6":{"tf":1.7320508075688772}}}}}}}},"title":{"root":{"df":0,"docs":{},"h":{"df":0,"docs":{},"t":{"df":0,"docs":{},"m":{"df":0,"docs":{},"l":{"df":1,"docs":{"6":{"tf":1.0}}}}}},"l":{"df":0,"docs":{},"i":{"c":{"df":0,"docs":{},"e":{"df":0,"docs":{},"n":{"df":0,"docs":{},"s":{"df":1,"docs":{"4":{"tf":1.0}}}}}},"df":0,"docs":{}}},"m":{"d":{"b":{"df":0,"docs":{},"o":{"df":0,"docs":{},"o":{"df":0,"docs":{},"k":{"df":1,"docs":{"0":{"tf":1.0}}}}}},"df":0,"docs":{}},"df":0,"docs":{}},"s":{"a":{"df":0,"docs":{},"t":{"df":0,"docs":{},"y":{"df":0,"docs":{},"s":{"df":0,"docs":{},"f":{"df":0,"docs":{},"i":{"df":1,"docs":{"0":{"tf":1.0}}}}}}}},"df":0,"docs":{}}}}},"lang":"English","pipeline":["trimmer","stopWordFilter","stemmer"],"ref":"id","version":"0.9.5"},"results_options":{"limit_results":30,"teaser_word_count":30},"search_options":{"bool":"OR","expand":true,"fields":{"body":{"boost":1},"breadcrumbs":{"boost":1},"title":{"boost":2}}}});