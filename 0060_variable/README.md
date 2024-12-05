# 変数
## この例題について

変数について学習する。
- Ubuntu 24.04 (WSL)
- rustc 1.82.0
- cargo 1.82.0

プロジェクトはCargoで作った

## 例題実行手順
例題を実行するにはプロジェクトの中で以下のコマンドを実行する。
```sh
cargo run
```
## 内容
### let文
変数はlet文で宣言する。型注釈をつけることもできる。
```rust
let foo = 3.14f64; // 変数の型は初期値から推論される。
let bar: f32 = 1.41421356; // 変数宣言には型注釈できる。
```
let文で宣言した変数はimmutableであり、再代入できない。constと似ているが、constは値であり、let文で宣言した変数には実体がある。

### let mut文
let mut文で宣言した変数には値を再代入できる。
```rust
let mut baz = "The quick brown fox"; // mutableな変数には再代入できる。
baz = "The rain in Spain";
```