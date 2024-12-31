use crate::ore::OreType;

pub struct Upgrade {
    pub desc: String,
    pub ore_type: OreType,
    pub cost: u32,
    pub bought: bool,
    pub upgrade_type: UpgradeType
}

impl Upgrade {
    pub fn new(desc: &str, ore_type: OreType, cost: u32, upgrade_type: UpgradeType) -> Self {
        Self {
            desc: desc.to_string(),
            ore_type,
            cost,
            bought: false,
            upgrade_type
        }
    }

    pub fn can_show(&self, money:u32) -> bool {
        if !self.bought && self.cost < money * 10 {
            true
        } else {
            false
        }
    }

    pub fn buy(&mut self, money: &mut u32) {
        if self.cost < *money {
            self.bought = true;
            *money -= self.cost;

        }
    }
}

pub enum UpgradeType {
    AutoMine,
    DoubleValue,
    UnlockOre
}

pub fn starting_upgrades() -> Vec<Upgrade> {
    vec![
        Upgrade::new("coal double value", OreType::Coal, 110, UpgradeType::DoubleValue),
        Upgrade::new("unlock iron", OreType::Iron, 110, UpgradeType::UnlockOre)
    ]
}