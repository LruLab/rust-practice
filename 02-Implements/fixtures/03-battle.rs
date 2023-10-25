//! check **Battle** implementation
//!
//! モンスターは敵の場合攻撃を仕掛け, 味方の場合は的に攻撃を仕掛けることができます.
//! 攻撃した際のダメージは, 攻撃力によって決定されます.
//! このダメージ量を返す`attack`メソッドを実装してください.
//!
//! また, 特定の攻撃を受けたときにHPが減少するように, `damage`メソッドを実装してください.

use implements::{Battle, Slime, TamedSlime};

fn main() {
    let mut slime = Slime::default();
    let mut tamed = TamedSlime::default();

    assert_eq!(slime.attack(), 3, "Slime's attack is wrong");
    assert_eq!(tamed.attack(), 3, "TamedSlime's attack is wrong");

    slime.damage(10);
    assert_eq!(slime.hp, 60, "Slime's damage is wrong");

    tamed.damage(10);
    assert_eq!(tamed.hp, 60, "TamedSlime's damage is wrong");

    slime.damage(100);
    assert_eq!(slime.hp, 0, "Slime's damage is wrong");

    tamed.damage(100);
    assert_eq!(tamed.hp, 0, "TamedSlime's damage is wrong");
}
