pub mod statistics;
pub mod unit;
pub mod warehouse;

extern crate rand;

use std::borrow::BorrowMut;
use std::io::Read;
use {
    statistics::*,
    unit::{Ant, Egg, Queen, UnitTypes, Worker},
    warehouse::Storage,
};

const EGGS_AT_START: u32 = 1;
const QUEENS_AT_START: u32 = 1;
const WORKERS_AT_START: u32 = 20;

pub struct Hive {
    ants: Vec<Box<dyn Ant>>,
    // queens: Vec<Queen>,
    // workers: Vec<Worker>,
    eggs: Vec<Egg>,
    storage: Storage,
    stats: Statistics,
}

impl Hive {
    pub fn new() -> Self {
        let mut workers = Worker::generate_workers(20);
        let mut ants: Vec<Box<dyn Ant>> = Vec::new();

        for worker in workers.into_iter() {
            ants.push(Box::new(worker))
        }

        ants.push(Box::new(Queen::new()));

        Hive {
            ants,
            eggs: vec![Egg::new()],
            storage: Storage::new(15000),
            stats: Statistics::new(EGGS_AT_START, QUEENS_AT_START, WORKERS_AT_START),
        }
    }

    pub fn anyone_alive(&self) -> bool {
        // if self.queens.len() > 0 {
        //     return true;
        // } else if self.eggs.len() > 0 {
        //     return true;
        // } else if self.workers.len() > 0 {
        //     return true;
        // }

        // false

        !self.ants.is_empty() || !self.eggs.is_empty()
    }

    pub async fn run(mut self) {
        let ants: &mut Vec<Box<dyn Ant>> = self.ants.as_mut();
        let eggs = self.eggs.as_mut();
        let storage = self.storage.borrow_mut();
        let stats = self.stats.borrow_mut();

        while !ants.is_empty() {
            stats.increment_day();

            ants.iter_mut().for_each(|ant| {
                match ant.unit_type() {
                    UnitTypes::Queen => {
                        produce_egg(eggs, ant, stats);
                    }
                    UnitTypes::Worker => {
                        produce_food(storage, ant);
                    }
                };
                feed(storage, ant, stats);
                age(ant, stats);
            });

            eggs.iter_mut().for_each(|egg| {
                if let Ok(_) = egg.reduce_hatch_timer() {
                    hatch(egg, ants, stats);
                }
            });

            eggs.retain(|e| !e.is_hatched());
            ants.retain(|ant| ant.is_alive());

            println!("{}", stats);
            println!("{}", storage);
        }
        println!("Hive loop finished");
    }
}

fn age(unit: &mut Box<dyn Ant>, stats: &mut Statistics) {
    unit.ages();
    if !unit.is_alive() {
        unit.dies();

        match unit.unit_type() {
            UnitTypes::Queen => stats.increment_queen_dead(DeathType::Age),
            UnitTypes::Worker => stats.increment_worker_dead(DeathType::Age),
        }
    }
}

fn feed(storage: &mut Storage, unit: &mut Box<dyn Ant>, stats: &mut Statistics) {
    let ate = unit.eat() as u64;
    if storage.get_food() < ate {
        unit.dies();

        match unit.unit_type() {
            UnitTypes::Queen => stats.increment_queen_dead(DeathType::Hunger),
            UnitTypes::Worker => stats.increment_worker_dead(DeathType::Hunger),
        }
        return;
    }

    storage.remove_food(ate);
}

fn produce_food(storage: &mut Storage, worker: &mut Box<dyn Ant>) {
    match worker.work() {
        Some(work_result) => {
            storage.add_food(work_result as u64);
        }
        None => return,
    }
}

fn produce_egg(eggs: &mut Vec<Egg>, queen: &mut Box<dyn Ant>, stats: &mut Statistics) {
    match queen.work() {
        Some(_) => {
            eggs.push(Egg::new());
            stats.increment_eggs_not_hatched();
        }
        None => return,
    }
}

fn hatch(egg: &mut Egg, ants: &mut Vec<Box<dyn Ant>>, stats: &mut Statistics) {
    egg.hatch();

    if rand::random::<f32>() < 0.9 {
        ants.push(Box::new(Worker::new()));
        stats.increment_worker_alive();
    } else {
        ants.push(Box::new(Queen::new()));
        stats.increment_queen_alive();
    }

    stats.increment_egg_hatched();
}
