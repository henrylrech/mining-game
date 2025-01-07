use crossterm::event::KeyCode;

use crate::shop::UpgradeType;

pub struct Ore {
    pub name: String,
    pub ore_type: OreType,
    pub value: u32,
    pub locked: bool,
    pub count: u32,
    pub char: KeyCode,
    pub auto_miners: u32
}

impl Ore {
    pub fn new(name: &str, value: u32, char: KeyCode, locked: bool, ore_type: OreType) -> Self {
        Self {
            name: name.to_string(),
            ore_type,
            value,
            locked,
            count: 0,
            char,
            auto_miners: 0
        }
    }

    pub fn mine(&mut self, money: &mut u32) {
        if self.locked {
            return
        }
        self.count += 1;
        *money += self.value;
    }

    pub fn automine(&mut self, money: &mut u32) {
        if self.locked {
            return
        }
        self.count += 1;
        *money += self.value * self.auto_miners;
    }

    pub fn upgrade(&mut self, upgrade_type: &UpgradeType) {
        match upgrade_type {
            UpgradeType::AutoMine => {
                self.auto_miners += 1;
            },
            UpgradeType::DoubleValue => {
                self.value *= 2;
            },
            UpgradeType::UnlockOre => {
                self.locked = false;
            }
        } 
    }
}

#[derive(PartialEq, Clone)]
pub enum OreType {
    Coal,
    Iron,
    Platinum,
    Emerald,
    Ruby,
    Sapphire,
    Gold,
    Diamond,
}

pub fn starting_ores() -> Vec<Ore> {
    vec![
        Ore::new("Coal", 1, KeyCode::Char('c'), false, OreType::Coal),
        Ore::new("Iron", 5, KeyCode::Char('i'), true, OreType::Iron),
        Ore::new("Platinum", 10, KeyCode::Char('p'), true, OreType::Platinum),
        Ore::new("Emerald", 20, KeyCode::Char('e'), true, OreType::Emerald),
        Ore::new("Ruby", 35, KeyCode::Char('r'), true, OreType::Ruby),
        Ore::new("Sapphire", 50, KeyCode::Char('s'), true, OreType::Sapphire),
        Ore::new("Gold", 70, KeyCode::Char('g'), true, OreType::Gold),
        Ore::new("Diamond", 100, KeyCode::Char('d'), true, OreType::Diamond),
    ]
}