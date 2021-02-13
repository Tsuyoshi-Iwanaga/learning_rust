# Learning Rust

## 1. 概要

### Rustの特徴

* コンパイラ言語 ガベージコレクタはない
* メモリアクセスは原則としてできないので安全性が高い
* マルチプラットフォーム
* cargoやrustfmtなどの優れたツール群が提供されている
* Cライブラリを使用できる
* Unicode対応、ソースコードのエンコーディングはUTF-8

### インストール

```
brew install rust
```

インストール後にはrustcコマンドが使えるようになる
(他にもcargo、rust-gdb、rust-gdbgui、rust-lldb、rustdocなど)

コマンドは/usr/local/binにシンボリックリンクが張られる

### コンパイル、実行

```shell
rustc hello.rs
```

```shell
./hello
```

### プロジェクトとビルド

プロジェクトの作成(Cargo.tomlやsrcディレクトリが生成される)

```shell
cargo new helloProject --bin
```

ビルド(target/debugフォルダが生成される)

```shell
cargo build
```

リリースビルド(target/releaseフォルダが生成される)

```shell
cargo build --release
```

ビルド実行のコマンド

```shell
cargo run
```

## 2.基礎

### 変数

```rust
fn main() {
  let age = 10;
  println!("age:{}", age);
}
```

### キーワード(予約語)

```
as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return Self self static struct super trait true type union unsafe use where while

abstract become box do final macro override priv try typeof unsized virtual yield
```

### データ型

```
bool 「ブール値」
char str 「キャラクタ/文字列スライス」
f32 f64 「浮動小数点型」
fn 「関数ポインタ」
i8 i16 i32 i64 i128 isize 「符号付き整数 0以上」
u8 u16 u32 u64 u128 usize 「符号なし整数 マイナスも可」
*const *mut 「ポインタ型(unsafe*)」
& 「参照型」
スライス「スライス型」
() 「"unit型"」
! 「"never型"」
tuple 「タプル」
[] 「配列」
```





