//! check **Default** implementation
//!
//! `Slime`は`スライム`という名前がデフォルトとなります.
//! 特に指定がない場合, この名前を使用して生成されるようにしてください.
//! HPの初期値は, 各モンスターの`MAX_HP`として定義されている値を使用します.
//! この名称, HP初期値のルールは`TamedSlime`にも共通です.

use implements::{Slime, TamedSlime};

fn main() {
    let slime = Slime::default();
    assert_eq!(slime.name, "スライム", "Slime's name is wrong");
    assert_eq!(slime.hp, 70, "Slime's HP is wrong");

    let tamed = TamedSlime::default();
    assert_eq!(tamed.name, "スライム", "TamedSlime's name is wrong");
    assert_eq!(tamed.hp, 70, "TamedSlime's HP is wrong");
}
