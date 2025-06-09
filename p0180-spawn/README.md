# サブプロセスの起動
## この例題について

サブプロセスを起動する方法を学ぶ。
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
サブプロセスの起動には、std::processモジュールを使う。サブプロセスは、コマンドラインで実行できるプログラムを指す。

### 起動
起動には`Command`構造体を使用する。`Command::new()`メソッドで実行するコマンドを指定し、必要な引数を`arg()`メソッドで追加する。
```rust
    // サブプロセスを起動する
    let mut child = Command::new("/usr/bin/lsblk")
        .arg("-f")
        .spawn()
        .expect("lsblk should run successfully");

```
起動したサブプロセスは、`Child`型のインスタンスとして返される。このインスタンスを使って、サブプロセスの標準入出力を操作したり、終了を待つことができる。
### 同期
```rust
    // サブプロセスの終了を待つ
    let _ = child
        .wait()
        .expect("Child process should exit successfully");
```
