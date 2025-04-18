# 列挙型
## この例題について

構造体について学習する。
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
### 型定義
予約語`enum`に続いて型名と、`{}`  でくるんだ要素を列挙すると、列挙型を宣言できる。

```rust
enum Flag {
    Yellow,
    RedCross,
    Red,
    Checker,
}

```
列挙型の変数が取ることのできる値は、列挙された要素のいずれから一つだけである。

### 列挙型リテラル
列挙型の要素のスコープは、型の中だけである。したがって、リテラルを書くときには`型名::要素名`という形式にする。

```rust
    let flag: Flag = Flag::Red;
```

### match文
列挙型の値に応じて処理をするときは、match文を利用できる。これはC言語のswtich-case文と同じである。
```
match文 ::= "match" 式 "{" アームの列 "}"
アームの列 ::= アーム { アームの列 }
アーム ::= 値 "=>" 式 ","
```

match文は式の値を評価し、一致する値を左辺に持つアームを選んで右辺を実行する。match文の値は実行した式の値である。

