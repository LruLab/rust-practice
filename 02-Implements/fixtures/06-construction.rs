//! check **Constructor**
//!
//! 各モンスターを生成するためのコンストラクタを実装してください.
//! コンストラクタは, 二つ名を持つモンスターへの対応のため, 名前を引数に取り初期化に使用します.
//! HPの初期値は, 各モンスターの`MAX_HP`として定義されている値を使用します.

use implements::{Monster, Slime, TamedSlime};

fn main() {
    let slime = Slime::new("さすらいのはぐれスライム".to_string());
    assert_eq!(slime.name, "さすらいのはぐれスライム", "Slime's name is wrong");
    assert_eq!(slime.hp, 70, "Slime's HP is wrong");

    let tamed = TamedSlime::new("従属スライムナイト".to_string());
    assert_eq!(tamed.name, "従属スライムナイト", "TamedSlime's name is wrong");
    assert_eq!(tamed.hp, 70, "TamedSlime's HP is wrong");
}
