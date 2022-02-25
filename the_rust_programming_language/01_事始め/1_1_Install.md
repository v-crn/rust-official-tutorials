# インストール

[インストール - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch01-01-installation.html)

## Linux と macOS に rustup をインストールする

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Windows に rustup をインストールする

[Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install)

## 更新とアンインストール

```sh
# 更新
rustup update

# アンインストール
rustup self uninstall
```

## トラブルシューティング

```sh
# Rustが正常にインストールされているか確かめる
rustc --version
```

## ローカルのドキュメンテーション

Rust をインストールすると、ローカルにドキュメンテーションが保存される。ブラウザでそのドキュメンテーションを開くには `rustup doc` を実行する。

```sh
rustup doc
```
