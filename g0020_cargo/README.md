# Cargoを使う
## この例題について

Cargo実行環境について学習する。
- Ubuntu 24.04 (WSL)
- rustc 1.82.0
- cargo 1.82.0

Cargo はrustのビルド及び実行環境であり、プロジェクトの初期化の機能も持っている。

# 手順
```sh
cargo new hello_cargo
```
これで新しいディレクトリが作られその中にプロジェクトが作られる。プロジェクトは設定ファイルCargo.tomlを含んでいる。
```toml:hello_cargo/Cargo.toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]

```
また、srcサブディレクトリには初期ソースコードが生成される。
```rust:hello_cargo/src/main.rs
fn main() {
    println!("Hello, world!");
}

```
プログラムは以下のコマンドでビルドできる。

```sh
cd hello_cargo
cargo build
```

また、以下のコマンドで実行できる。
```sh
cd hello_cargo
cargo run
```

