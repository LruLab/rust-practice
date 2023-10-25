## Error Handling

### 要求事項

このcrateには以下の関数群は次のような処理を行うための関数群が含まれています.

- 環境変数からSecretの記述されたファイルのパスを取得する
  - ファイルにはユーザIDとパスワードのハッシュ値の組が記述されている
  - フォーマットはJSON形式であり, `user`, `hash`をkey, 文字列を値とするオブジェクトの配列が記述されている
- ファイルの内容を読み込まれ, 静的な変数として格納される
- verify関数は, 引数に与えられた文字列が, ユーザIDとパスワードのハッシュ値の組のいづれかと一致するかを検証する
  - この関数にはBasic認証のAuthorizationヘッダから取得した文字列が渡されることを想定している

処理には, 途中でエラーとなり処理が即時異常終了する可能性が多分に含まれています.
適切にエラー処理を実行できるようこの関数たちを完成させ, 異常終了する可能性を排除してください.


## 仕様

関数`verify`の実行に当たり, どのようなエラーが発生した場合にも, `VerifyError`としてエラーを返すようにしてください.
panicを起こしての異常終了は認められません.

エラーのマッピングとしては次のようなものが考えられます.

|状態|エラー|
|---|---|
|Basic認証のヘッダではない値が与えられた|`VerifyError::InvalidHeaderFormat`|
|Basic認証のヘッダではあるが, base64にてdecodeできない|`VerifyError::InvalidHeaderFormat`|
|Basic認証のヘッダではあるが, ユーザIDとパスワードの組が`:`で区切られていない|`VerifyError::InvalidHeaderFormat`|
|環境変数`CREDENTIALS_PATH`が設定されていない|`VerifyError::LoadFailed`|
|環境変数`CREDENTIALS_PATH`が示すパス上のファイルが読み込めない|`VerifyError::LoadFailed`|
|環境変数`CREDENTIALS_PATH`が示すパス上のファイルの形式が不正|`VerifyError::LoadFailed`|
|指定されたユーザIDが存在しない|`VerifyError::UserNotFound`|
|指定されたユーザIDが存在するが, パスワードが一致しない|`VerifyError::PasswordMismatch`|

## 詳細

ソースコード中に`unwrap`や`expect`等が記載されている部分は「エラーが発生する可能性があるがその制御を放棄する」という記述をしている部分です.
これらの記述をすべて除去できれば, この関数群は正常に動作するようになります.

### Result

異常終了せずにエラーを扱うためには, `Result`型を用いる必要があります.
この型は, `Ok`と`Err`の2つの列挙子を持ち, それぞれ正常終了と異常終了を表現します.

```rust
let result: Result<i32, String> = Ok(42);
let result: Result<i32, String> = Err("error".to_string());
```

### Error Trait

Errorとして振る舞うためのTraitが, 標準に用意されており, これを実装することでBackTrace等のエラー情報を保持できるようになります.
以前まではFailure等, 標準には含まれていない別のライブラリを別途利用する必要がありましたが, 現在は標準で提供されています.


実装対象: `std::error::Error` (今回必須とはしていませんし, チェックも特に入れていません)

### ? 演算子

`?` 演算子は, `Result`型の値を返す関数内でのみ使用可能な特殊な演算子です.
この演算子は`Result`型に対してのみ使用可能で, 次のような挙動を取ります.

- 値が`Ok`の場合は`Ok`の値返却し処理を継続
- 値が`Err`の場合はその`Err`を関数から返却
  - 返却する`Err`が明示している返り値の型でない場合, `?` 演算子は `From` Traitを用いて, `Err`の値を返り値に変換するよう試みます.

例えば以下のように記述した場合, 次のような挙動を取ります.

- 環境変数の読み込み時, 環境変数が存在しなかったり, Unicode的に不正な文字列だったりする場合には,　`std::env::var`からは`std::env::VarError`が返却される
- `?`演算子を用いることで, `VarError`が返却された場合は`From` Traitを用いて, `MyError`に変換した上で即時returnする.
- `std::env::var`が`Ok`を返却した場合は, `?`演算子は`Ok`の値を返却し, 処理を継続する.

```rust
#[derive(Debug)]
struct MyError;

impl From<std::env::VarError> for MyError {
    fn from(_: std::env::VarError) -> Self {
        MyError
    }
}

fn main() -> Result<(), MyError> {
    let path = std::env::var("CREDENTIALS_PATH")?;
    println!("{}", path);

    Ok(())
}
```

### チェック方法

`cargo test`を通過にてクリアとします.

```sh
cargo test
```

手動にて動作を検証する場合は, `src/main.rs`に検証したい処理を記述し, 次のコマンドを実行してください.

```sh
cargo run
```

`examples` ディレクトリには回答例があります.
どうしても手がかりがつかめない場合は参考にしてください.


## extra

### thiserror

このライブラリは, `derive`にて, `Error` Trait, `Display`Traitを実装するためのマクロを提供します.
更に `from`Attributesを利用することで, `From` Traitを実装することも可能です.

詳細はexamplesへ...


### anyhow

このライブラリは, ? 演算子を利用することを念頭に, `std::error::Error`を実装しているあらゆるエラー型からの`From`変換が可能なエラー型`anyhow::Error`を提供します.
これを利用することで, 複雑な`From`変換のロジックを記載することなく, 簡潔にエラーを扱うことができます.
(その一方で, エラーの中身詳細が伝搬しづらく, 詳細なエラーを取り扱いづらくなるデメリットもあります)

詳細はexamplesへ...