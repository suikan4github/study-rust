# study-rust
Rustの自習ノートです。私が個人利用のために作ったノートですが、Rustをこれから学習したい人の役にたてば幸いです。

このREADME.mdは「ですます調」で書いていますが、個人用ノートであるという理由から他のmarkdownページはすべて「である調」になっています。

Rust言語については以下のサイトを参考にしています。
- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)

# オンライン・コンパイラ
Rustにはプレイグラウンドと呼ばれるオンライン・コンパイラおよび実行環境が存在します。これらはRustを学ぶ上で大いに役立ちます。
代表的なものとして[Tour of Rust](https://tourofrust.com/)を挙げておきます。

なお、WSLで使用する場合はwsluをインストールしておいてください。
```sh
sudo apt install wslu
```
# 内容

- [0010](p0010_install/README.md) : Rustコンパイラのインストール
- [0020](p0020_cargo/README.md) : Cargoを使う
- [0030](p0030_comment/README.md) : コメント
- [0040](p0040_number_literal/README.md ) : 数値リテラル
- [0050](p0050_string_literal/README.md) : 文字列リテラル
- [0060](p0060_variable/README.md ) : 変数
- [0070](p0070_shadowing/README.md) : シャドーイング
- [0080](p0080_tuple/README.md) : タプル
- [0090](p0090_array/README.md) : 配列
- [0100](p0100-struct/README.md) : 構造体
- [0110](p0110-enum/README.md) : enum
- [0120](p0120-enum-bound/README.md) : 値を付加された列挙値
- [0130](p0130-reqwest-blocking/README.md) : URLをつかってオブジェクトを取得する
