# Rustの学習メモ

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





