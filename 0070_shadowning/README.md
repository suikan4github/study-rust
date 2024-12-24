# 変数
## この例題について

シャドーイングについて学習する。
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
### シャドーイング
let文で宣言した変数は、let文で再宣言できる。元の変数は「隠される」のでアクセスできない。
```rust
    let foo = "Hello";
    println!("foo is {}", foo);
    // foo = "World"; // not allowed.
    let foo = "World"; // shadowing by redeclaration. Allowed.
    println!("foo is {}", foo);
```

### スコープ
ネストしたスコープ内でシャドーイングすると、スコープを出たときに隠された変数へ再度アクセスできる。
```rust
    let foo = "World"; // shadowing by redeclaration. Allowed.
    println!("foo is {}", foo);

    {
        let foo = "from Rust"; // shadowing from nested scope. Allowed.
        println!("foo is {}", foo);
    }
    
    println!("foo is {}", foo); // The variable in the nested scope is gone.
```

### 違う型でのシャドーイング
シャドーイングは同じ型に限らない。

```rust 
    let foo = "Hello";
    let foo = foo.len();
    println!("foo is {}", foo);
```
