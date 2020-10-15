use serde::{Deserialize, Serialize};

pub enum DeathType {
    Hunger,
    Age,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Statistics {
    day: u32,
    queens_alive: u32,
    queens_dead: u32,
    workers_alive: u32,
    workers_dead: u32,
    death_by_age: u32,
    death_by_hunger: u32,
    eggs_not_hatched: u32,
    eggs_hatched: u32,
}

impl Statistics {
    pub fn new(eggs: u32, queens_alive: u32, workers_alive: u32) -> Self {
        Statistics {
            day: 0,
            queens_alive,
            queens_dead: 0,
            workers_alive,
            workers_dead: 0,
            death_by_age: 0,
            death_by_hunger: 0,
            eggs_not_hatched: eggs,
            eggs_hatched: 0,
        }
    }

    pub fn increment_day(&mut self) {
        self.day += 1;
    }

    pub fn increment_worker_alive(&mut self) {
        self.workers_alive += 1
    }

    pub fn increment_worker_dead(&mut self, death_type: DeathType) {
        if self.workers_alive > 0 {
            self.workers_alive -= 1;
        }

        self.workers_dead += 1;

        match death_type {
            DeathType::Hunger => self.death_by_hunger += 1,
            DeathType::Age => self.death_by_age += 1,
        }
    }

    pub fn increment_egg_hatched(&mut self) {
        self.eggs_hatched += 1;

        if self.eggs_not_hatched > 0 {
            self.eggs_not_hatched -= 1;
        }
    }

    pub fn increment_eggs_not_hatched(&mut self) {
        self.eggs_not_hatched += 1;
    }

    pub fn increment_queen_alive(&mut self) {
        self.queens_alive += 1;
    }

    pub fn increment_queen_dead(&mut self, death_type: DeathType) {
        self.queens_dead += 1;

        if self.queens_alive > 0 {
            self.queens_alive -= 1;
        }

        match death_type {
            DeathType::Hunger => self.death_by_hunger += 1,
            DeathType::Age => self.death_by_age += 1,
        }
    }
}

impl std::fmt::Display for Statistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|----------------\n| Day {}\n|-> Queens\n|--> Alive: {}\n|--> Dead: {}\n|-> Workers\n|--> Alive: {}\n|--> Dead: {}\n|-> Cause of Death\n|--> Age: {}\n|--> Hunger: {}\n|-> Eggs\n|--> Unhatched: {}\n|--> Hatched: {}",
               self.day, self.queens_alive, self.queens_dead, self.workers_alive, self.workers_dead, self.death_by_age, self.death_by_hunger, self.eggs_not_hatched, self.eggs_hatched)
    }
}

#[cfg(test)]
mod test {
    use super::super::{DeathType, Statistics};
    #[test]
    fn test_increment_eggs_hatched() {
        let mut stats = Statistics::new(1, 0, 0);
        stats.increment_egg_hatched();

        assert_eq!(stats.eggs_hatched, 1);
        assert_eq!(stats.eggs_not_hatched, 0);
    }

    #[test]
    fn test_increment_worker_dead_by_age() {
        let mut stats = Statistics::new(0, 1, 1);

        stats.increment_worker_dead(DeathType::Age);
        assert_eq!(stats.workers_alive, 0);
        assert_eq!(stats.workers_dead, 1);
        assert_eq!(stats.queens_alive, 1);
        assert_eq!(stats.queens_dead, 0);
        assert_eq!(stats.death_by_age, 1);
        assert_eq!(stats.death_by_hunger, 0);
    }

    #[test]
    fn test_increment_worker_dead_by_hunger() {
        let mut stats = Statistics::new(0, 1, 1);

        stats.increment_worker_dead(DeathType::Hunger);
        assert_eq!(stats.workers_alive, 0);
        assert_eq!(stats.workers_dead, 1);
        assert_eq!(stats.queens_alive, 1);
        assert_eq!(stats.queens_dead, 0);
        assert_eq!(stats.death_by_age, 0);
        assert_eq!(stats.death_by_hunger, 1);
    }
}
