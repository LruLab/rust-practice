//! check **Clone** implementation
//! 
//! モンスターは分身することができます.
//! 分身したモンスターは, 分身元のモンスターと同じ名前, HPを持ち, 独立にダメージを受け, 行動することができます.

use implements::{Monster, Battle, Slime, TamedSlime};

fn main() {
    let mut slime = Slime::default();
    let mut tamed = TamedSlime::default();

    slime.damage(10);
    tamed.damage(10);

    let mut slime_clone = slime.clone();
    let mut tamed_clone = tamed.clone();

    assert_eq!(slime, slime_clone);
    assert_eq!(tamed, tamed_clone);

    slime_clone.damage(10);
    tamed_clone.damage(10);

    assert_ne!(slime, slime_clone);
    assert_ne!(tamed, tamed_clone);
}