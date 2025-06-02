# URLをつかってオブジェクトを非同期取得する
## この例題について
この例題は、`reqwest`クレートを使って、指定したURLからオブジェクトを取得する方法を説明する。
- Ubuntu 24.04 (WSL)
- rustc 1.86.0
- cargo 1.86.0
- reqwest 0.12.18
- tokio v1.45.1

プロジェクトはCargoで作った。

## 例題実行手順
例題を実行するにはプロジェクトの中で以下のコマンドを実行する。
```sh
cargo run
```
## 内容
`reqwest`はCargo.tomlに以下のように記述してインストールする。
```toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking"] }
```
中心になるのは次の1文である。
```rust
    // 通信によりヘッダが帰ってくるのを待つ
    // ここで初めてレスポンスを得る。ただし、レスポンス・ボディは未着の可能性がある。 
    let response_struct = request_future
        .await // ヘッダの到着を待つ
        .expect("It could get the response from the URL");
    
    // レスポンスをテキストに変換する。
    // text()が返すのは、非同期処理のためのFuture構造体。
    let text_conversion_future = response_struct.text(); 

    // 変換が終了するのを待つ。
    let response: String = text_conversion_future
        .await  // 文字列変換の完了を待つ
        .expect("It could be converted to text");

```
このコードは変数`response`に、指定したURLから取得したレスポンスのテキストを格納する。

.get()や.text()などのメソッドは、非同期処理のためのFuture構造体を返す。この構造体に対して.awaitを呼び出すことで、非同期処理の完了を待つことができる。
