extern crate rand;
use rand::Rng;

pub trait Eat{
    fn eat(&self) -> u8;
}

const QUEEN_APPETITE: u8 = 5;
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

impl Eat for Queen {
    fn eat(&self) -> u8 {
        self.apetite
    }
}


const WORKER_APPETITE: u8 = 5;
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
        let rng_efficiency = rand::thread_rng().gen_range(WORKER_LIFETIME_MIN, WORKER_LIFETIME_MAX);

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

impl Eat for Worker {
    fn eat(&self) -> u8 {
        self.apetite
    }
}

const EGG_HATCH_TIMER_MIN: u8 = 10;
const EGG_HATCH_TIMER_MAX: u8 = 61;

#[derive(Debug)]
pub struct Egg {
    hatch_timer: u8,
}

impl Egg {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rng_hatch_timer = rng.gen_range(EGG_HATCH_TIMER_MIN, EGG_HATCH_TIMER_MAX);

        Egg {
            hatch_timer: rng_hatch_timer,
        }
    }
}