extern crate rand;

pub mod units {
    use rand::Rng;

    #[derive(Debug)]
    pub enum UnitTypes {
        Egg,
        Queen,
        Worker,
    }

    pub trait Ant {
        fn eat(&self) -> u8;
        fn work(&mut self) -> Option<u8>;
        fn dies(&mut self);
        fn ages(&mut self);
        fn is_alive(&self) -> bool;
        fn unit_type(&self) -> UnitTypes;
    }

    const QUEEN_APPETITE: u8 = 8;
    const QUEEN_LAY_EGG_COOLDOWN: u8 = 10;
    const QUEEN_LIFETIME_MIN: u32 = 90;
    const QUEEN_LIFETIME_MAX: u32 = 366;

    #[derive(Debug)]
    pub struct Queen {
        apetite: u8,
        lay_egg_cooldown: u8,
        lifetime: u32,
        is_alive: bool,
    }

    impl Queen {
        pub fn new() -> Self {
            let mut rng = rand::thread_rng();
            let rng_lifetime = rng.gen_range(QUEEN_LIFETIME_MIN, QUEEN_LIFETIME_MAX);

            Queen {
                apetite: QUEEN_APPETITE,
                lay_egg_cooldown: QUEEN_LAY_EGG_COOLDOWN,
                lifetime: rng_lifetime,
                is_alive: true,
            }
        }
    }

    impl Ant for Queen {
        fn eat(&self) -> u8 {
            self.apetite
        }

        fn work(&mut self) -> Option<u8> {
            if self.lay_egg_cooldown != 0 {
                self.lay_egg_cooldown -= 1;
                return None;
            }

            self.lay_egg_cooldown = QUEEN_LAY_EGG_COOLDOWN;
            Some(1)
        }

        fn dies(&mut self) {
            self.is_alive = false;
        }

        fn ages(&mut self) {
            if self.lifetime < 1 {
                self.dies();
                return
            }

            self.lifetime -= 1;
        }

        fn is_alive(&self) -> bool {
            self.is_alive
        }

        fn unit_type(&self) -> UnitTypes {
            UnitTypes::Queen
        }
    }


    const WORKER_APPETITE: u8 = 2;
    const WORKER_COOLDOWN: u8 = 5;
    const WORKER_EFFICIENCY_MIN: u8 = 5;
    const WORKER_EFFICIENCY_MAX: u8 = 26;
    const WORKER_LIFETIME_MIN: u8 = 20;
    const WORKER_LIFETIME_MAX: u8 = 61;

    #[derive(Debug)]
    pub struct Worker {
        apetite: u8,
        work_cooldown: u8,
        work_efficiency: u8,
        lifetime: u8,
        is_alive: bool,
    }

    impl Worker {
        pub fn new() -> Self {

            let rng_lifetime = rand::thread_rng().gen_range(WORKER_LIFETIME_MIN, WORKER_LIFETIME_MAX);
            let rng_efficiency = rand::thread_rng().gen_range(WORKER_EFFICIENCY_MIN, WORKER_EFFICIENCY_MAX);

            Worker {
                apetite: WORKER_APPETITE,
                work_cooldown: WORKER_COOLDOWN,
                work_efficiency: rng_efficiency,
                lifetime: rng_lifetime,
                is_alive: true
            }
        }



        pub fn generate_workers(count: i32) -> Vec<Worker> {
            let mut workers = vec![];
            let mut i = 0;

            while i < count {
                workers.push(Worker::new());
                i += 1;
            }

            workers
        }
    }

    impl Ant for Worker {
        fn eat(&self) -> u8 {
            self.apetite
        }

        fn work(&mut self) -> Option<u8> {
            if self.work_cooldown != 0 {
                self.work_cooldown -= 1;
                return None;
            }

            self.work_cooldown = WORKER_COOLDOWN;
            Some(self.work_efficiency)
        }

        fn dies(&mut self) {
            self.is_alive = false;
        }

        fn ages(&mut self) {
            if self.lifetime < 1 {
                self.dies();
                return
            }

            self.lifetime -= 1;
        }

        fn is_alive(&self) -> bool {
            self.is_alive
        }

        fn unit_type(&self) -> UnitTypes {
            UnitTypes::Worker
        }
    }

    const EGG_HATCH_TIMER_MIN: u8 = 10;
    const EGG_HATCH_TIMER_MAX: u8 = 61;

    #[derive(Debug)]
    pub struct Egg {
        unit_type: UnitTypes,
        hatch_timer: u8,
        hatched: bool,
    }

    impl Egg {
        pub fn new() -> Self {
            let rng_hatch_timer = rand::thread_rng().gen_range(EGG_HATCH_TIMER_MIN, EGG_HATCH_TIMER_MAX);

            Egg {
                unit_type: UnitTypes::Egg,
                hatch_timer: rng_hatch_timer,
                hatched: false,
            }
        }

        pub fn get_hatch_timer(&self) -> u8 {
            self.hatch_timer
        }

        pub fn reduce_hatch_timer(&mut self) {
            self.hatch_timer -= 1;
        }

        pub fn hatch(&mut self) {
            self.hatched = true;
        }

        pub fn is_hatched(&self) -> bool {
            self.hatched
        }
    }
}

pub mod warehouse {
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
}
