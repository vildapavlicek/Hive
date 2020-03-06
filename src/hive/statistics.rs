pub mod stats {

    pub enum DeathType {
        Hunger,
        Age
    }

    #[derive(Debug)]
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

        pub fn new(queens_alive: u32, ants_alive: u32) -> Self {
            Statistics {
                day: 0,
                queens_alive: queens_alive,
                queens_dead: 0,
                workers_alive: ants_alive,
                workers_dead: 0,
                death_by_age: 0,
                death_by_hunger: 0,
                eggs_not_hatched: 0,
                eggs_hatched: 0,
            }
        }

        pub fn get_day(&self) -> u32 {
            self.day
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
                DeathType::Age => self.death_by_age += 1
            }
        }

        pub fn increment_egg_hatched(&mut self) {
            self.eggs_hatched += 1;

            if self.eggs_not_hatched > 0 {
                self.eggs_not_hatched -=1;
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
                DeathType::Age => self.death_by_age += 1
            }
        }

        pub fn report(&self) {
            println!("Day {}", self.day);
            println!("Queens alive: {} | Queens dead: {}", self.queens_alive, self.queens_dead);
            println!("Ants alive: {} | Ants dead: {}", self.workers_alive, self.workers_dead);
            println!("Ants dead by hunger: {} | Ants dead by age: {}", self.death_by_hunger, self.death_by_age);
            println!("Unhatched eggs: {} | Hatched eggs: {}", self.eggs_not_hatched, self.eggs_hatched);
        }
    }
}