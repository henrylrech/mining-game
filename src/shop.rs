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

    pub fn visible(&self, money: u32) -> bool {
        if !self.bought && self.cost < money * 2 {
            true
        } else {
            false
        }
    }

    pub fn buy(&mut self, money: &mut u32) -> Result<(), ()> {
        if self.cost < *money {
            self.bought = true;
            *money -= self.cost;
            return Ok(());
        }
        Err(())
    }
}
#[derive(Clone)]
pub enum UpgradeType {
    AutoMine,
    DoubleValue,
    UnlockOre
}

pub fn starting_upgrades() -> Vec<Upgrade> {
    vec![
        Upgrade::new("coal double value", OreType::Coal, 110, UpgradeType::DoubleValue),
        Upgrade::new("unlock iron", OreType::Iron, 450, UpgradeType::UnlockOre),
        Upgrade::new("iron double value", OreType::Iron, 1200, UpgradeType::DoubleValue),
        Upgrade::new("unlock platinum", OreType::Platinum, 2600, UpgradeType::UnlockOre),
        Upgrade::new("platinum double value", OreType::Platinum, 5500, UpgradeType::DoubleValue),
        Upgrade::new("unlock emerald", OreType::Emerald, 10800, UpgradeType::UnlockOre),
        Upgrade::new("emerald double value", OreType::Emerald, 14700, UpgradeType::DoubleValue),
        Upgrade::new("unlock ruby", OreType::Ruby, 30400, UpgradeType::UnlockOre),
        Upgrade::new("ruby double value", OreType::Ruby, 35600, UpgradeType::DoubleValue),
        Upgrade::new("unlock sapphire", OreType::Sapphire, 50900, UpgradeType::UnlockOre),
        Upgrade::new("sapphire double value", OreType::Sapphire, 75500, UpgradeType::DoubleValue),
        Upgrade::new("unlock gold", OreType::Gold, 102000, UpgradeType::UnlockOre),
        Upgrade::new("gold double value", OreType::Gold, 140400, UpgradeType::DoubleValue),
        Upgrade::new("unlock diamond", OreType::Diamond, 200000, UpgradeType::UnlockOre),
        Upgrade::new("diamond double value", OreType::Diamond, 220000, UpgradeType::DoubleValue),
    ]
}