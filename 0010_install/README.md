# Rustコンパイラのインストール
## この例題について

Rustコンパイラについて学習する。
- Ubuntu 24.04 (WSL)
- rustc 1.82.0

# 手順
## インストール
コマンドラインから以下のプログラムを実行する。root権限は不要
```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
インストールが終わったらshellを再起動し、以下のコマンドを実行する。
```sh
rustc --version
```
バージョン番号が表示されたらインストールは成功である。なお、コンパイラは~/.cargo/binにインストールされている。

## アップデート
一度インストールしたコンパイラは以下のコマンドでアップデートできる。
```sh
rustup update
```

## アンインストール
Rustコンパイラとrustupをアンインストールするときには以下のコマンドを実行する。
```sh
rustup self uninstall
```

# Hello, World
以下のプログラムをファイルに書いて保存する。
この例題ではすでにmain.rsに保存済みである。
```rust:main.rs
fn main() {
    println!("Hello, World!");
}

```
コマンドラインから以下のプログラムを実行する。

```sh
rustc main.rs
./main
```