# URLをつかってオブジェクトを取得する
## この例題について
この例題は、`reqwest`クレートを使って、指定したURLからオブジェクトを取得する方法を説明する。
- Ubuntu 24.04 (WSL)
- rustc 1.86.0
- cargo 1.86.0
- reqwest 0.12.18

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
    let response: String = reqwest::blocking::get(url)
        .expect("It could get the response from the URL")
        .text()
        .expect("It could be converted to text");
```
このコードは変数`response`に、指定したURLから取得したレスポンスのテキストを格納する。
