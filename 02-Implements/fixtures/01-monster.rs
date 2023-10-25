//! check **Monster** implementation
//!
//! モンスターは, それぞれ最大HPと攻撃力を持ちます.
//! `Slime`の最大HPは70, 攻撃力は3であり, これは`TamedSlime`にも共通です.

use implements::{Monster, Slime, TamedSlime};

fn main() {
    assert_eq!(Slime::MAX_HP, 70, "Slime's MAX_HP is wrong");
    assert_eq!(Slime::POWER, 3, "Slime's POWER is wrong");

    assert_eq!(TamedSlime::MAX_HP, 70, "TamedSlime's MAX_HP is wrong");
    assert_eq!(TamedSlime::POWER, 3, "TamedSlime's POWER is wrong");
}
