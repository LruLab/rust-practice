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

// Deriveを利用することで, 多少楽ができるTraitもあります.
// 今回の場合, 比較条件やデフォルト値は特定の値が指定されているため利用できません.
// DebugやPartialEq, PartialOrd, Eq, Ord, Cloneは指定の挙動がderiveと同等のため, これらを利用することで実装を簡略化できます.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Slime {
    pub hp: u32,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TamedSlime {
    pub hp: u32,
    pub name: String,
}

// 共通の振る舞いはTraitにまとめることができる, という話でした.
// 今回の場合, 「HP操作がある」「スライムはどちらもデフォルト名称同一」等ちょこちょこ共通化したいものはありました.
// 同一crate内に限られますが, 「このTraitを実装するすべてのStructに実装する」という指定ができます
pub trait SlimeLike {
    const DEFAULT_NAME: &'static str = "スライム";

    fn hp(&self) -> u32;
    fn set_hp(&mut self, hp: u32);
}

impl<S: SlimeLike> Monster for S {
    const MAX_HP: u32 = 70;
    const POWER: u32 = 3;
}

impl<S: SlimeLike> Battle for S {
    fn attack(&self) -> u32 {
        Self::POWER
    }

    fn damage(&mut self, damage: u32) {
        self.set_hp(self.hp().saturating_sub(damage));
    }
}

impl Tamed for TamedSlime {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

// TraitはStructの型を知らないので, このあたりの構造体のフィールド参照等は共通化できません
impl SlimeLike for Slime {
    fn hp(&self) -> u32 {
        self.hp
    }

    fn set_hp(&mut self, hp: u32) {
        self.hp = hp;
    }
}

impl SlimeLike for TamedSlime {
    fn hp(&self) -> u32 {
        self.hp
    }

    fn set_hp(&mut self, hp: u32) {
        self.hp = hp;
    }
}

// コンストラクタも完全同一記述ですが, Struct直接構築前提なら共通化できません.
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

impl Default for Slime {
    fn default() -> Self {
        Self::new(Self::DEFAULT_NAME.to_string())
    }
}

impl Default for TamedSlime {
    fn default() -> Self {
        Self::new(Self::DEFAULT_NAME.to_string())
    }
}

// 変換処理に関しては元よりderiveはないため, 実装するほかありません.
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

impl From<TamedSlime> for Slime {
    fn from(tamed_slime: TamedSlime) -> Self {
        Self {
            hp: Self::MAX_HP,
            name: tamed_slime.name,
        }
    }
}

fn main() {}
