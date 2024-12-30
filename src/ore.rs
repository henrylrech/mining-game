use crossterm::event::KeyCode;

pub struct Ore {
    pub name: String,
    pub value: u32,
    pub locked: bool,
    pub count: u32,
    pub char: KeyCode
}

impl Ore {
    pub fn new(name: &str, value: u32, char: KeyCode, locked: bool) -> Self {
        Self {
            name: name.to_string(),
            value,
            locked,
            count: 0,
            char
        }
    }

    pub fn mine(&mut self, money: &mut u32) {
        self.count += 1;
        *money += self.value;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }
}