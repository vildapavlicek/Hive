#[derive(Debug)]
pub struct Storage{
    food: u64,
}

impl Storage {
    pub fn new(food_value: u64) -> Self {
        Storage {
            food: food_value,
        }
    }

    pub fn remove_food(&mut self, to_remove: u64) {
        self.food = self.food - to_remove;
    }
}