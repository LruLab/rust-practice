//! # Trait実装試験
//!
//! 次の構造体`Slime`, `TamedSlime`に対し, 次のふるまいを実装してください
//! ()内は実装を期待するトレイトを表します.
//!
//! ## モンスターの定義 (Monster)
//!
//! モンスターは, それぞれ最大HPと攻撃力を持ちます.
//! `Slime`の最大HPは70, 攻撃力は3であり, これは`TamedSlime`にも共通です.
//!
//! ## モンスターのデフォルト生成 (Default)
//!
//! `Slime`は`スライム`という名前がデフォルトとなります.
//! 特に指定がない場合, この名前を使用して生成されるようにしてください.
//! HPの初期値は, 各モンスターの`MAX_HP`として定義されている値を使用します.
//! この名称, HP初期値のルールは`TamedSlime`にも共通です.
//!
//! ## モンスターの攻撃 (Battle)
//!
//! モンスターは敵の場合攻撃を仕掛け, 味方の場合は的に攻撃を仕掛けることができます.
//! 攻撃した際のダメージは, 攻撃力によって決定されます.
//! このダメージ量を返す`attack`メソッドを実装してください.
//!
//! また, 特定の攻撃を受けたときにHPが減少するように, `damage`メソッドを実装してください.
//! このとき, HPは0未満にならないようにしてください.
//!
//! ## 仲間になったモンスター (Tamed)
//!
//! 仲間になったモンスターには名前をつけることができます.
//! この名前を設定する`set_name`メソッドを実装してください.
//!
//! ## デバッグ出力(Debug)
//!
//! どんなモンスターでも画面上に情報が出せなければ話になりません.
//! 各モンスターを`dbg!`や`println!`マクロで出力可能な状態にしてください.
//! 出力はいかなる形式でも構いません.
//!
//! ## コンストラクタ (`new`)
//!
//! 各モンスターを生成するためのコンストラクタを実装してください.
//! コンストラクタは, 二つ名を持つモンスターへの対応のため, 名前を引数に取り初期化に使用します.
//! HPの初期値は, 各モンスターの`MAX_HP`として定義されている値を使用します.
//!
//! ## 仲間にする / 逃がす (From/Into, TryFrom/TryInto)
//!
//! `Slime`は仲間になると`TamedSlime`に変化します.
//! `Slime`はHPが`MAX_HP`の半分以下であるとき, 仲間にすることができます.
//! また, `TamedSlime`はいつでも逃がして`Slime`に戻すことができます.
//!
//! これらの変換を実装してください.
//! なお, 仲間にしようとしたがHPが`MAX_HP`の半分以上である場合は, 仲間にすることができません.
//! この場合, `String`型でエラーを返してください.
//!
//! ## 比較(Eq, PartialEq, Ord, PartialOrd)
//!
//! `Slime`と`TamedSlime`は, 残りのHP量に比例して大きくなります.
//! そのサイズに応じて, 比較が行えるようにしてください.
//!
//! サイズが同じ場合, 名前辞書順にて比較し, 辞書順の早いほうが小さいもの(a<z)としてください.
//! `Slime`と`TamedSlime`の比較は考慮しなくて構いません.
//!
//! ## 分身 (Clone)
//!
//! モンスターは分身することができます.
//! 分身したモンスターは, 分身元のモンスターと同じ名前, HPを持ち, 独立にダメージを受け, 行動することができます.
//!

/* WRITE YOUR CODE ANYWHERE */

use std::fmt::Debug;

pub trait Monster {
    const MAX_HP: u32;
    const POWER: u32;
}

pub trait Battle: Monster {
    fn attack(&self) -> u32;
    fn damage(&mut self, damage: u32);
}

pub trait Tamed: Monster {
    fn set_name(&mut self, name: String);
}

pub struct Slime {
    pub hp: u32,
    pub name: String,
}

pub struct TamedSlime {
    pub hp: u32,
    pub name: String,
}
