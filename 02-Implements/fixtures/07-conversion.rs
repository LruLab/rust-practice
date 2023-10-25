//! check **From/Into**, **TryFrom/TryInto** implementation
//!
//! `Slime`は仲間になると`TamedSlime`に変化します.
//! `Slime`はHPが`MAX_HP`の半分以下であるとき, 仲間にすることができます.
//! また, `TamedSlime`はいつでも逃がして`Slime`に戻すことができます.

use implements::{Monster, Battle, Slime, TamedSlime};

fn main() {
    // try from
    let tamed = TamedSlime::try_from(Slime::default());
    assert!(tamed.is_err(), "Slime cannot be tamed");

    let mut slime = Slime::default();
    slime.damage(TamedSlime::MAX_HP / 2 + 1);
    let tamed = TamedSlime::try_from(slime);
    assert!(tamed.is_ok(), "Slime can be tamed");

    // try into
    let tamed: Result<TamedSlime, String> = Slime::default().try_into();
    assert!(tamed.is_err(), "Slime cannot be tamed");

    let mut slime = Slime::default();
    slime.damage(TamedSlime::MAX_HP / 2 + 1);
    let tamed: Result<TamedSlime, String> = slime.try_into();
    assert!(tamed.is_ok(), "Slime can be tamed");

    // from
    let tamed = TamedSlime::default();
    let slime = Slime::from(tamed);

    // into
    let tamed = TamedSlime::default();
    let slime: Slime = tamed.into();
}
