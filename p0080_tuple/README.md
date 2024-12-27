# タプル
## この例題について

タプルについて学習する。
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
### タプル
"()"で型名の列をくるむとタプル型になる。また、"()"で値の列をくるむとタプル型の値になる。
```rust
    // Declaration of tuple
    let _a: (u32, f64, &str) = (24, 3.141592, "Hello, world");
```

### 要素へのアクセス
タプル変数名の後ろに"."およびインデックスを続けると、個別要素にアクセスできる。インデックスは0から始まる。
```rust
    // Element access
    println!("_a.0 is {}", _a.0);

```

### 分割代入
let文で"()"でくるんだ変数列を宣言すると、タプル型の要素をそれぞれの変数に代入できる。
```rust
    // Destructuring assignment
    let (_x, _y, _z) = _a;

    println!("_x is {}", _x);
    println!("_y is {}", _y);
    println!("_z is {}", _z);
```

### 違う型でのシャドーイング
シャドーイングは同じ型に限らない。

```rust 
    let foo = "Hello";
    let foo = foo.len();
    println!("foo is {}", foo);
```
