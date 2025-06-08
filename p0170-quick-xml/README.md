# quick-xmlの使い方
## この例題について
XMLを高速にパースするためのライブラリである`quick-xml`の使い方を学ぶ。
- Ubuntu 24.04 (WSL)
- rustc 1.86.0
- cargo 1.86.0

プロジェクトはCargoで作った。

## 例題実行手順
例題を実行するにはプロジェクトの中で以下のコマンドを実行する。
```sh
cargo run
```
## 内容
quick-xmlを使って、XMLファイルをパースし、特定の要素のテキストを抽出する。quick-xmlのドキュメントは[こちら](https://docs.rs/quick-xml/latest/quick_xml/)。

quick-xmlはpull型のXMLパーサーで、イベント駆動型である。XMLの各要素をイベントとして受け取り、必要な情報を抽出する。
### コード

イベントの取得は以下のように行う。readerは`Reader`型のインスタンスで、XMLを読み込むために使用する。bufはバッファで、イベントを読み込むために使用される。
```rust
match reader.read_event_into(&mut buf) {
```

イベントの種類は`Event`型で、以下のようにマッチングして処理を行う。Startイベントは要素の開始を示し、Textイベントは要素のテキストを示す。Endイベントは要素の終了を示す。

この例代ではXMLファイルを木構造として扱い、特定の要素のテキストを抽出する。具体的には、地域が"tokyo"のときに、r2hlsのURLを抽出する。

要素の名前(<data>や<areajp>など)は、イベントの`name()`メソッドを使用して取得できる。このプログラムはそれらの名前をコンテキストと呼ぶ。

新しい要素が開始されると、現在のコンテキストをスタックに保存し、新しいコンテキストに切り替える。要素のテキストは、`Text`イベントで取得できる。CDATAセクションは、`CData`イベントで取得できる。
```rust
Ok(Event::Start(ref e)) => {
    context_stack.push(context.clone());
    context = String::from_utf8_lossy(e.name().as_ref()).to_string();
}
```
`Text`イベントでは、要素のテキストを取得し、特定のコンテキストに応じて処理を行う。この時、テキストはXMLエスケープされている可能性があるため、`unescape()`メソッドを使用してデコードする。
```rust
Ok(Event::Text(e)) => {
    let text = e.unescape().unwrap().to_string();
    if context == "areajp" {
        println!("地域: {}", text);
    } else if context == "area" {
        area = text;
    }
}
```
CDATAセクションは、`CData`イベントで取得し、テキストと同様に処理を行う。CDATAセクションは、XMLの特殊文字を含むことができるため、`String::from_utf8_lossy()`を使用してUTF-8に変換する。
```rust

Ok(Event::CData(e)) => {
    let text = String::from_utf8_lossy(&e).to_ascii_lowercase().to_string();
    if context == "r1hls" {
        r1_url = text;
    } else if context == "r2hls" {
        r2_url = text;
    } else if context == "fmhls" {
        fm_url = text;
    }
}
```
`End`イベントでは、現在のコンテキストをスタックからポップし、前のコンテキストに戻る。<data>要素の終了時に、地域が"tokyo"の場合はr2hlsのURLを出力する。
```rust
Ok(Event::End(_)) => {
    if context == "data" {
        if area == "tokyo" {
            result = r2_url.clone();
        }
        r1_url.clear();
        r2_url.clear();
        fm_url.clear();
        area.clear();
    }
    context = context_stack.pop().unwrap_or_default();
}
```