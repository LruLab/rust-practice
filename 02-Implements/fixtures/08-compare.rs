//! check **PartialEq/Eq**, **PartialOrd/Ord** implementation
//! 
//! `Slime`と`TamedSlime`は, 残りのHP量に比例して大きくなります.
//! そのサイズに応じて, 比較が行えるようにしてください.
//! `Slime`と`TamedSlime`の比較は考慮しなくて構いません.

use implements::{Monster, Battle, Slime, TamedSlime};

fn main() {
    let mut slime1 = Slime::default();
    let slime2 = Slime::default();

    assert!(slime1 == slime2);
    assert!(slime1 <= slime2);
    assert!(slime1 >= slime2);

    slime1.damage(10);

    assert!(slime1 != slime2);
    assert!(slime1 < slime2);
    assert!(slime2 > slime1);

    let mut tamed1 = TamedSlime::default();
    let tamed2 = TamedSlime::default();

    assert!(tamed1 == tamed2);
    assert!(tamed1 <= tamed2);
    assert!(tamed1 >= tamed2);

    tamed1.damage(10);

    assert!(tamed1 != tamed2);
    assert!(tamed1 < tamed2);
    assert!(tamed2 > tamed1);

    let named_slime1 = Slime::new("A-スライム".to_string());
    let named_slime2 = Slime::new("Z-スライム".to_string());

    assert!(named_slime1 < named_slime2);
}