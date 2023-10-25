//! check **Debug** implementation
//! 
//! どんなモンスターでも画面上に情報が出せなければ話になりません.
//! 各モンスターを`dbg!`や`println!`マクロで出力可能な状態にしてください.
//! 出力はいかなる形式でも構いません.

use implements::{Monster, Slime, TamedSlime};

fn main() {
    let slime = Slime::default();
    dbg!(slime);

    let tamed = TamedSlime::default();
    dbg!(tamed);
}