//! check **Tamed** implementation
//!
//! 仲間になったモンスターには名前をつけることができます.
//! この名前を設定する`set_name`メソッドを実装してください.

use implements::{Tamed, Slime, TamedSlime};

fn main() {
    let mut tamed = TamedSlime::default();
    tamed.set_name("スーパースライム3世".to_string());
    assert_eq!(tamed.name, "スーパースライム3世", "TamedSlime name cannot be set");
}
