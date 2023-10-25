## Rust 練習帳

Rustの簡易実装問題集です.

## Workspaceの構成

このリポジトリは, [Cargo Workspace](https://doc.rust-jp.rs/book-ja/ch14-03-cargo-workspaces.html)で構成されています.
各実装問題は独立したCrateとして用意されており, それぞれについて正しく実装した場合には通過可能なテストが用意されています.
それぞれ実際に実装してみて, どのようになるのかを確認してみてください.

実装方法がわからない場合は, `examples`ディレクトリに回答例が用意されています.
また, `cargo run --example <対象回答例>`にて実行結果を確認することができます.

## 実行想定環境

- Rust 1.73.0+ (Edition 2021)

## 実装問題

### 01 Fizz Buzz

単純なFizzBuzzです.

テーマ: 基礎 / 基本制御フロー

### 02 Implement Traits

Rustには共通の振る舞いを定義する`Trait`が存在します.
指定の構造体に対して`Trait`を実装し, 適切な振る舞いが可能な状態にしてください.

単元: Trait 実装 / 構造体 / 列挙型

### 03 Error Handling

Rustでは他の言語にあるような大域脱出的なエラーハンドリングを行う機能は基本的に利用しません.
`panic`を雑に投げて諦めることをやめ, `Result`を用いたエラー伝搬を行い, main関数までエラーを運んでいくことができる状態にしてください.

単元: Error / Result / ? Operator / panic!

### 04 Async/Await (準備中)

Rustでは並列処理においてもその安全性を保証する仕組みが提供されています.
指定の関数に対して適切に実装を行い, 並列処理実行可能な状態にしてください.

単元: Async/Await / Future / Arc / Mutex / lifetime

### 05 Macro (準備中)

Rustには自由自在に自身の定めた記法を利用することのできるmacroという機構が存在します.
また, 特定の構造体やメソッド等に対して, その定義記述の構文解析木を駆使して, 任意のコードを自動実装するproc_macroという機構が存在します...

単元: macro / proc_macro / proc_macro_derive

## library

### x1 Database (準備中)

ライブラリ[diesel](https://diesel.rs/)を用いて, 指定のデータベースに対してクエリを発行し, データを取得してください.

単元: diesel / データベース / クエリ

### x2 Web Server (準備中)

ライブラリ[actix-web](https://actix.rs/)を用いて, 指定のWebサーバーを実装してください.

単元: actix-web / Webサーバー / ルーティング


