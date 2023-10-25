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

impl Monster for Slime {
    const MAX_HP: u32 = 70;
    const POWER: u32 = 3;
}

impl Monster for TamedSlime {
    const MAX_HP: u32 = Slime::MAX_HP;
    const POWER: u32 = Slime::POWER;
}

// Default
// デフォルト値を定義する
impl Default for Slime {
    fn default() -> Self {
        Self {
            hp: Self::MAX_HP,
            name: "スライム".to_string(),
        }
    }
}

impl Default for TamedSlime {
    fn default() -> Self {
        Self {
            hp: Self::MAX_HP,
            name: "スライム".to_string(),
        }
    }
}

impl Battle for Slime {
    fn attack(&self) -> u32 {
        Self::POWER
    }

    fn damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }
}

impl Battle for TamedSlime {
    fn attack(&self) -> u32 {
        Self::POWER
    }

    fn damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }
}

impl Tamed for TamedSlime {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

// Debug
// コンソール上等に対して出力を行う際の文字列化方法を定義する
// フォーマットは自由に書けるため, 「デバッグ用に整えて出力!」を毎回書く必要はない
impl Debug for Slime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Slime({}) [{}/{}]",
            self.name,
            self.hp,
            Self::MAX_HP
        ))
    }
}

impl Debug for TamedSlime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Slime {{TAMED}} ({}) [{}/{}]",
            self.name,
            self.hp,
            Self::MAX_HP
        ))
    }
}

// コンストラクタ
// Rustの構造体は直接フィールド初期化する術があるが, Privateなフィールドの指定や, 事前検証処理が必要となるような場合にはコンストラクタを用いる.
//
// 全フィールドがPublicな場合, コンストラクタを用意しても, 直接フィールド初期化する方法が利用できてしまう.
// この方法を取られると, 将来的にPrivateフィールドを追加したときにエラーが発生したり, Validationを回避されたりするため,
// #[non_exhaustive]を付与して「将来的なフィールド増加の可能性の考慮の強制」を行うことでコンストラクタを用いることを強制することができる
impl Slime {
    pub fn new(name: String) -> Self {
        Self {
            hp: Self::MAX_HP,
            name,
        }
    }
}

impl TamedSlime {
    pub fn new(name: String) -> Self {
        Self {
            hp: Self::MAX_HP,
            name,
        }
    }
}

// TryFrom/TryInto
// エラーの発生する可能性のある変換を行う場合に用いる
// TryFromは自動的にTryIntoを実装する (TryIntoはTryFromを実装しない)
impl TryFrom<Slime> for TamedSlime {
    type Error = String;

    fn try_from(slime: Slime) -> Result<Self, Self::Error> {
        if slime.hp <= Self::MAX_HP / 2 {
            Ok(Self {
                hp: slime.hp,
                name: slime.name,
            })
        } else {
            Err("Failed to tame".to_string())
        }
    }
}

// From/Into
// エラーの発生しない変換を行う場合に用いる
// Fromは自動的にIntoを実装する (IntoはFromを実装しない)
impl From<TamedSlime> for Slime {
    fn from(tamed_slime: TamedSlime) -> Self {
        Self {
            hp: Self::MAX_HP,
            name: tamed_slime.name,
        }
    }
}

// Clone
// CloneはDeepCopyを実装する
impl Clone for Slime {
    fn clone(&self) -> Self {
        Self {
            hp: self.hp,
            name: self.name.clone(),
        }
    }
}

impl Clone for TamedSlime {
    fn clone(&self) -> Self {
        Self {
            hp: self.hp,
            name: self.name.clone(),
        }
    }
}

// Eq, PartialEq
// PartialEqは == と != を実装する
// Eqは PartialEqが実装されていてかつ, 自身が自身と等しいことを判定できる場合にのみ実装できる
// Eqは「同値関係」, PartialEqは「部分同値関係」を表現する物
impl PartialEq for Slime {
    fn eq(&self, other: &Self) -> bool {
        self.hp == other.hp
    }
}

impl PartialEq for TamedSlime {
    fn eq(&self, other: &Self) -> bool {
        self.hp == other.hp
    }
}

impl Eq for Slime {}

impl Eq for TamedSlime {}

// Ord, PartialOrd
// PartialOrdは <, <=, >, >= を実装する
// Ordは PartialOrdが実装されていてかつ, 自身が自身以上/以下であることを判定できる場合にのみ実装できる
// Ordは「全順序」, PartialOrdは「半順序」を表現する物
impl PartialOrd for Slime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hp.cmp(&other.hp) {
            std::cmp::Ordering::Equal => Some(self.name.cmp(&other.name)),
            ordering => Some(ordering),
        }
    }
}

impl PartialOrd for TamedSlime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hp.cmp(&other.hp) {
            std::cmp::Ordering::Equal => Some(self.name.cmp(&other.name)),
            ordering => Some(ordering),
        }
    }
}

impl Ord for Slime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hp.cmp(&other.hp) {
            std::cmp::Ordering::Equal => self.name.cmp(&other.name),
            ordering => ordering,
        }
    }
}

impl Ord for TamedSlime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hp.cmp(&other.hp) {
            std::cmp::Ordering::Equal => self.name.cmp(&other.name),
            ordering => ordering,
        }
    }
}

fn main() {}
