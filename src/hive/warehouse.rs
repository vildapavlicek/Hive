#[derive(Debug)]
pub struct Storage {
    food: u64,
}

impl Storage {
    pub fn new(food_value: u64) -> Self {
        Storage { food: food_value }
    }

    pub fn add_food(&mut self, to_add: u64) {
        self.food += to_add;
    }

    pub fn remove_food(&mut self, to_remove: u64) {
        self.food -= to_remove;
    }

    pub fn get_food(&self) -> u64 {
        self.food
    }
}

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Food in storage: {}", self.food)
    }
}
