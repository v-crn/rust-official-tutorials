# Hello, Cargo

[Hello, Cargo! - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html)

- Cargo は、Rust のビルドシステム兼パッケージマネージャ
- Cargo を使用する利点
  - 使用しているOSに関わらず、同じコマンドが使用できる

## Cargo でプロジェクトを作成する

```sh
cargo new hello_cargo --bin
cd hello_cargo
```

Cargo.toml が自動的に作成される。

- TOML: Tom's Obvious, Minimal Language; トムの明確な最小限の言語
- `[dependencies]` は、プロジェクトの依存を列挙するためのセクションの最初の行
- Rust では、パッケージのコードは「クレート」として参照される

src/main.rs も自動生成される。

- Cargo は、ソースファイルが src ディレクトリに存在することを期待する
- Cargo はプロジェクトを体系化する手助けをしてくれる

## Cargo プロジェクトをビルドし、実行する

### プロジェクトのビルド / cargo build

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

- Cargo はビルドの結果をコードと同じディレクトリに保存するのではなく、Cargoはtarget/debug ディレクトリに格納する
  - 実行ファイル target/debug/hello_cargo が作成される
- プロジェクト直下に Cargo.lock が作成される
- Cargo.lock はプロジェクトの依存のバージョンを追跡する
- Cargo.lock を手動で変更する必要はない

### コンパイルと実行 / cargo run

`cargo run` でコンパイルと実行ファイルの実行をまとめて行うことができる。

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/hello_cargo`
Hello, world!
```

### コンパイルが通るかチェック / cargo check

`cargo check` でコンパイルできることを確認できる。

```console
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

- `cargo build` は時間が掛かることがあるが、`cargo check` はより高速
- プログラム作成の途中段階でよく使うことになる

## リリースビルド

プロジェクトをリリースする準備ができたら、`cargo build --release` で最適化とコンパイルを行うことができる。

```console
❯ cargo build --release
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished release [optimized] target(s) in 0.85s
```

- 最適化を行うと
  - Rust コードの実行が速くなる
    - 実行時間のベンチマークはこの release 版を使って行う
  - コンパイル所要時間が伸びる

## 習慣としての Cargo

- 複数のクレートからなる複雑なプロジェクトでは、Cargo にビルドを調整してもらうことに大きな利便性がある
- Cargo について詳しく: [Introduction - The Cargo Book](https://doc.rust-lang.org/cargo/)
