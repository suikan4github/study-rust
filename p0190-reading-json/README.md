# JSONファイルの読み込み
## この例題について
この例題では、Rustの`serde_json`クレートを使用してJSONファイルを読み込む方法を示す。
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

### Cargo.toml
Cargo.tomlファイルは、プロジェクトの依存関係を管理するために使用される。このプロジェクトでは`serde`と`serde_json`の2つのクレートを依存関係として追加している。

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```
### main.rs
`serde`クレートを使用してJSONデータをRustの構造体にデシリアライズする。以下のコードは、JSONファイルを読み込み、データを表示する。

読み込むJSONファイルは`series.json`で、以下のよう構造になっている。
```json
{
    "id": 121,
    "title": "カルチャーラジオ　漢詩をよむ",
    "radio_broadcast": "R2",
    ...,
    "episodes": [
        {
            "program_title": "カルチャーラジオ 漢詩をよむ 中国 古都の詩(うた)(11)",
            "stream_url": "https://vod-stream.nhk.jp/radioondemand/r/YVW32ZL75Z/s/stream_YVW32ZL75Z_051955dcb4b75dabc3cbe85f76e79b2f/index.m3u8",
            ...,
        },
        ...,
    ],
   
}
```
これをRustの構造体にデシリアライズするために、まずエピソード構造体を定義する。
```rust
// エピソード構造体を定義する
#[derive(Serialize, Deserialize, Debug)]
//#[allow(non_snake_case)]
struct Episode {
    id: u32,
    program_title: String,
    onair_date: String,
    stream_url: String,
    program_sub_title: String,
}
```
エピソード構造体は、`#[derive(Serialize, Deserialize, Debug)]`アトリビュートを使用して、`serde`クレートのシリアライズとデシリアライズを可能にする。

次に、シリーズ構造体を定義する。
```rust
// シリーズ構造体を定義する
#[derive(Serialize, Deserialize, Debug)]
//#[allow(non_snake_case)]
struct Series {
    id: u32,
    title: String,
    episodes: Vec<Episode>,
}
```
シリーズ構造体も同様に、`#[derive(Serialize, Deserialize, Debug)]`アトリビュートを使用して、`serde`クレートのシリアライズとデシリアライズを可能にする。

デシリアライズは1行で済む。
```rust    
    // JSONをデシリアライズする
    let series: Series = serde_json::from_str(&file_content).expect("JSON was not well-formatted");
```
ここで、series変数には、JSONファイルの内容がデシリアライズされた`Series`構造体のインスタンスが格納される。

構造体のフィールド名とJSONのキー名が一致している必要がある。もし一致しない場合は、`#[serde(rename = "key_name")]`アトリビュートを使用して、フィールド名をJSONのキー名に合わせることができる。
