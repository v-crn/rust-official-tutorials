# Hello, World

[Hello, World! - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch01-02-hello-world.html)

```sh
mkdir ./hello_world
cd hello_world
```

main.rs を作成、コンパイル、実行。

```sh
# Compile
rustc main.rs

# Run
./main
```

- Rust はAOTコンパイル(ahead-of-time; 予め)言語
- 簡単なプログラムなら `rustc` でコンパイルするだけで十分だが、プロジェクトが肥大化してくると管理が難しくなってくる。そこで次に紹介する Cargo ツールを利用する
