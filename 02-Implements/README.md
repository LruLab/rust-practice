## Implementation

### 要求事項

Rustではただ構造体を定義しただけでは, その構造体に対して何も操作を行うことができません.
次の構造体`Slime`, `TamedSlime`に対し, 次のふるまいを実装してください
  
```rust
pub struct Slime {
    pub hp: u32,
    pub name: String,
}

pub struct TamedSlime {
    pub hp: u32,
    pub name: String,
}
```

### 仕様

「対象Trait」と記載されているTraitをそれぞれ実装してください.
実装方法は問いません.

#### モンスターの定義

モンスターは, それぞれ最大HPと攻撃力を持ちます.
`Slime`の最大HPは70, 攻撃力は3であり, これは`TamedSlime`にも共通です.

対象Trait: `crate::Monster`

<details>
<summary>Details</summary>

RustのTraitは他の言語のInterfaceとは異なり, Trait自体に実装を載せたり, Traitとして定数値の定義を要求したり, 特定の条件を満たす型を指定させたりすることができます.
今回の場合は定数値要求のため, `MAX_HP`と`POWER`という定数値を定義することで, `Monster`Traitを実装したものとみなすことができます.

```rust
impl Monster for Slime {
  /// ???
}
```

</details>

--- 

#### モンスターのデフォルト生成

`Slime`は特に指定のない場合は`"スライム"`という名前で生成されることとします.
HPの初期値は, 各モンスターの`MAX_HP`として定義されている値を使用します.
この名称, HP初期値のルールは`TamedSlime`にも共通です.

対象Trait: `std::default::Default`

<details>
<summary>Details</summary>

Rustのデフォルト値実装は, `Default` Traitを実装することで可能になります.

```rust
impl Default for Slime {
  /// ???
}
```

これを定義しておくことで, 任意の`Slime`構造体のインスタンスが必要な箇所において`Default::default()`と記述するだけでこのデフォルト値を呼び出すことができるようになります.

```rust
let s: Slime = Default::default();
```

なお構造体の全Fieldが`Default`Traitを実装している場合, `derive`を使用して, 該当構造体のデフォルト値を各Fieldのデフォルト値を`default()`で充填した物とするよう自動実装することができます.

今回の場合は `name`として`"スライム"`がデフォルトで入るよう要求があるため, `derive`を使用することはできません.
`String`のデフォルト値は空文字列です.
</details>

--- 

#### モンスターの攻撃

モンスターは敵の場合攻撃を仕掛け, 味方の場合は的に攻撃を仕掛けることができます.
攻撃した際のダメージは, 攻撃力によって決定されます.
このダメージ量を返す`attack`メソッドを実装してください.

また, 特定の攻撃を受けたときにHPが減少するように, `damage`メソッドを実装してください.
このとき, HPは0未満にならないようにしてください.

対象Trait: `crate::Battle`

<details>
<summary>Details</summary>

ごく普通のTrait実装です.
指定されたメソッドを実装するのみで完了します.

もし, `現在のHPを参照する`, `HP残量を変更する`といった振る舞いが実装されていることを要求できるのであれば, 実装を一つにまとめることもできるかもしれません.

</details>

---

#### 仲間になったモンスター

仲間になったモンスターには名前をつけることができます.
この名前を設定する`set_name`メソッドを実装してください.

対象Trait: `crate::Tamed`

<details>
<summary>Details</summary>

ごく普通のTrait実装Part 2です.

</details>

---

#### デバッグ出力

どんなモンスターでも画面上に情報が出せなければ話になりません.
各モンスターを`dbg!`や`println!`マクロで出力可能な状態にしてください.
出力はいかなる形式でも構いません.

対象Trait: `std::fmt::Debug`

<details>
<summary>Details</summary>

Rustで新しく定義した構造体は, どのように出力を行ったら良いのかの指定が行われていないため, そもそも内容を出力しようとすると, コンパイルエラーになります.
出力される方式の指定を行うためには2種類の標準Traitがあり, そのどちらかを実装する必要があります.

`std::fmt::Display` Traitを実装すると, `{}`で出力する方式を指定できます.
`std::fmt::Debug` Traitを実装すると, `{:?}`で出力する方式を指定でき, `dbg!`マクロに直接渡すことができるようになります.

一般に`Debug` Traitには簡素な出力を, `Display` Traitには読みやすさに重点を置いた出力を指定します.
今回の仕様においては, `dbg!`を使用する指定があるため, `Debug` Traitを実装してください.

```rust
impl std::fmt::Debug for Slime {
  /// ???
}
```

あるいは, `derive`を使用することで, `Debug` Traitを自動実装させることもできます.

</details>

---

#### コンストラクタ

各モンスターを生成するためのコンストラクタを実装してください.
コンストラクタは, 二つ名を持つモンスターへの対応のため, 名前を引数に取り初期化に使用します.
HPの初期値は, 各モンスターの`MAX_HP`として定義されている値を使用します.

<details>
<summary>Details</summary>

Rustの構造体は, 初期時点では関数としてのコンストラクタを持たず, インスタンスを生成するためにはブラケットで囲った上で, 直接全フィールドに値を指定するような記述をする必要があります.

```rust
let slime = Slime {
  hp: 70,
  name: String::from("スライム"),
};
```

このような記述を行っていくと, 次のような問題点に遭遇します.

- Fieldが追加されたとき, 上記の初期化を行っているすべての箇所で該当Fieldの記述を追加する必要がある
  - ライブラリの場合, 該当のライブラリを使用しているすべての箇所で修正が必要となる
- PrivateなFieldが追加されたとき, 初期化を行う上記記述がすべてエラーとなりブラケット記述での初期化ができなくなる
  - ライブラリの場合, Privateを変更しただけなのに破壊的な影響が生じる
- 全Fieldを指定する必要があるため, Fieldが増えると記述量が増える
  - RustにおいてはOptionalなFieldであっても`None`を記述する必要がある
- 直接保持したい型で指定されるため, Fieldに対して事前に値の検証をかけておくことができない

これらの問題を解決するために, 一般的にはコンストラクタを実装し, コンストラクタでは必須のField等限られた値のみを受け取り, 検証を実施した上でインスタンスを作成します.
Fieldが多い場合には`Builder`パターンを用いることで解決します.

```rust
let slime = Slime::new(String::from("スライム"));
```

コンストラクタについては特にTraitが存在しているものではないため, 自分で実装する必要があります.
慣習的にメソッド名として`new`を使用します.

```rust
impl Slime {
  fn new(name: String) -> Self {
    // ???
  }
}
```

なおブラケット記述での初期化は, PrivateなFieldを持つ構造体に対しては使用できませんが, PublicなFieldのみを持つ構造体については, コンストラクタを実装した場合であっても記述を継続して利用できてしまいます.

外部からの利用時にはブラケット記述での初期化を行えないようにするためには, `#[non_exhaustive]`Attributesを付与することにより, 「今後フィールドが追加される可能性の考慮の強制」を行うことができ, 結果的にブラケット記述での初期化を行えなくすることができます.

```rust
#[non_exhaustive]
pub struct Slime {
  /// ???
}
```

</details>

---

---

#### 仲間にする / 逃がす

`Slime`は仲間になると`TamedSlime`に変化します.
`Slime`はHPが`MAX_HP`の半分以下であるとき, 仲間にすることができます.
また, `TamedSlime`はいつでも逃がして`Slime`に戻すことができます.

これらの変換を実装してください.
なお, 仲間にしようとしたがHPが`MAX_HP`の半分以上である場合は, 仲間にすることができません.
この場合, `String`型でエラーを返してください.

対象Trait: `std::convert::From`, `std::convert::Into`, `std::convert::TryFrom`, `std::convert::TryInto`

<details>
<summary>Details</summary>

特定の型から特定の型への変換を可能にするには, `From`, `Into`, `TryFrom`, `TryInto` Traitを実装します.
`From`と`Into`はエラーの発生する余地のない変換を行う場合に使用し, `TryFrom`と`TryInto`はエラーの発生する可能性のある変換を行う場合に使用します.

例えば, `i32`型から`i64`型への変換は, 32bitから64bitへの変換であり, エラーの発生する余地がないため, `From`と`Into`が実装されています.
逆に`i64`型から`i32`型への変換は, 64bitから32bitへの変換であり, 元の数値が32bitに入らない値となっていた場合は正常に変換を完了できず, エラーの発生する可能性があるため, `TryFrom`と`TryInto`が実装されています.

```rust
// i32 -> i64
let x = 1i32;
let y: i64 = x.into();

// i64 -> i32
let x = 1i64;
let y: i32 = x.try_into().unwrap();
```

`Slime`型から`TamedSlime`型への変換は, HP状態によってはエラーの発生する可能性があるため, `TryFrom`と`TryInto`を実装します.
`TamedSlime`型のインスタンスから`Slime`型への変換は, エラーの発生する余地がないため, `From`と`Into`を実装します.

```rust
impl From<TamedSlime> for Slime {
  /// ???
}

impl TryFrom<Slime> for TamedSlime {
  /// TryFrom Traitはエラー型の定義が要求されます
  type Error = String;

  /// ???
}
```

なお, `Into` Traitは `From` Traitが実装されることで自動実装され, `TryInto` Traitは `TryFrom` Traitが実装されることで自動実装されます.
そのため, `From`Traitを実装した後に`Into`Traitを実装しようとすると, 重複定義にてエラーになります.
特に`Into`, `TryInto`のみを実装する明確な意図がない場合には, `From`, `TryFrom`のみを実装することを推奨します.

</details>

---

#### 比較

`Slime`と`TamedSlime`は, 残りのHP量に比例して大きくなります.
そのサイズに応じて, 比較が行えるようにしてください.
`Slime`と`TamedSlime`の比較は考慮しなくて構いません.

対象Trait: `std::cmp::PartialEq`, `std::cmp::PartialOrd`, `std::cmp::Eq`, `std::cmp::Ord`

<details>
<summary>Details</summary>

比較を可能とするには, 等号(`==`), 不等号(`!=`)については`PartialEq` Trait, 大小比較(`>`, `<`, `>=`, `<=`)については`PartialOrd` Traitを実装します.

```rust
impl PartialEq for Slime {
  /// ???
}

impl PartialOrd for Slime {
  /// ???
}
```

これらの上位概念として, `Eq`, `Ord` Traitがありますが, これらはそれぞれ「同値関係」「全順序」を表すTraitであり, それぞれ実装対象の構造体や型が数学的に該当する条件を満たしている場合にのみ実装可能なTraitになります.
例えば, `f64`型はその値として`NaN`を持つことができ, これは`NaN == NaN`, `NaN <= NaN`が成立せず, 反射性を持たないことから同値関係, 全順序どちらの条件も満たせず, `f64`型に対しては`Eq`, `Ord` Traitを実装することができません.

`Eq`や`Ord`が実装されている場合, その型の任意のインスタンスについて, 同一判定, 比較判定が可能になることになるため, Sortや最大値取得などのそのインスタンスの集合に対する一般的な操作を行う関数が利用できるようになります.
例えば`Iterator`に実装されている`max`関数は, その内部に保持している型に`Ord` Traitが実装されている場合に限り利用することができます.

```rust
struct Num(i32);

fn main() {
    // i32にはOrdが実装されているため, max関数が利用できる
    let mx = vec![1, 2, 3].iter().max();

    // NumにはOrdが実装されていないため, max関数が利用できない (Compile Error)
    let mx = vec![Num(1), Num(2), Num(3)].iter().max();
}
```

なおこれらのTraitには依存関係があり, `Eq` Traitを実装するには`PartialEq` Traitを実装する必要があり, `Ord` Traitを実装するには`PartialOrd` Trait, `Eq`Traitを実装する必要があります.

またこれらのTraitは, `derive`を使用することで自動実装させることができます.
今回の場合, 比較条件はHP残量 => 名称辞書順となっているため, `derive`の自動実装が適合します.

</details>

---

#### 分身

モンスターは分身することができます.
分身したモンスターは, 分身元のモンスターと同じ名前, HPを持ち, 独立にダメージを受け, 行動することができます.

対象Trait: `std::clone::Clone`

<details>
<summary>Details</summary>

Rustのコピーは`Clone`Trait, `Copy`Traitを実装することで可能になります.
`Clone`TraitはDeep Copy, `Copy`TraitはShallow Copyを行うことを表します.

```rust
impl Clone for Slime {
  /// ???
}
```

`Copy`TraitはShallow Copyを実装するため, フィールドに参照を持っているような構造体の場合は, 「参照をコピーして使いまわす」という挙動をとってしまい, 所有権上の問題が生じるため実装すること自体ができなくなっています.
今回の場合String型は内部にヒープ領域に格納しているデータへの参照を持つため, `Copy`Traitを実装することはできません.

</details>

---

### チェック方法

`cargo test`を通過にてクリアとします.

```sh
cargo test
```

今回のケースは「コンパイルが無事通過し, テスト処理が実行できること」がテストケースになっています.
`fixtures`ディレクトリ内にRustコードがありますが, こちらはcargo run等では実行されません.
手動にて動作を検証する場合は, `src/main.rs`に検証したい処理を記述し, 次のコマンドを実行してください.

```sh
cargo run
```

`examples` ディレクトリには回答例があります.
どうしても手がかりがつかめない場合は参考にしてください.
