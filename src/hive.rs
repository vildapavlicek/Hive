mod unit;
pub mod statistics;

extern crate rand;

pub mod hive {
    use crate::hive::unit::units::{Queen, Worker, Egg, Ant, UnitTypes};
    use crate::hive::unit::warehouse::Storage;
    use crate::hive::statistics::stats::{Statistics, DeathType};
    use crate::producer::producer::{Message};
    use tokio::sync::mpsc::{Sender};

    const EGGS_AT_START: u32 = 1;
    const QUEENS_AT_START: u32 = 1;
    const WORKERS_AT_START: u32 = 20;
    

    pub struct Hive {
        queens: Vec<Queen>,
        workers: Vec<Worker>,
        eggs: Vec<Egg>,
        storage: Storage,
        stats: Statistics,
        tx: Sender<Message>,
    }

    impl Hive {
        pub fn new(tx: Sender<Message>) -> Self {
            Hive {
                queens: vec![Queen::new()],
                workers: Worker::generate_workers(20),
                eggs: vec![Egg::new()],
                storage: Storage::new(15000),
                stats: Statistics::new(EGGS_AT_START, QUEENS_AT_START, WORKERS_AT_START),
                tx: tx,
            }
        }

        pub fn anyone_alive(&self) -> bool {
           if self.queens.len() > 0 {
               return true;
           } else if self.eggs.len() > 0 {
               return true;
           } else if self.workers.len() > 0 {
               return true;
           }

           false

        }

        pub async fn run(&mut self) {

            while self.anyone_alive() {
                self.stats.increment_day();
                
                for queen in self.queens.iter_mut() {
                    if queen.is_alive(){
                        feed(&mut self.storage, queen, &mut self.stats);
                        produce_egg(&mut self.eggs, queen, &mut self.stats);
                        age(queen, &mut self.stats);
                    }
                }
                
                for worker in self.workers.iter_mut() {
                    if worker.is_alive(){
                        feed(&mut self.storage, worker, &mut self.stats);
                        produce_food(&mut self.storage, worker);
                        age(worker, &mut self.stats);
                    }
                }

                for egg in self.eggs.iter_mut() {
                    if egg.get_hatch_timer() > 0 {
                        egg.reduce_hatch_timer()
                    } else {
                        hatch(egg, &mut self.workers, &mut self.queens, &mut self.stats);
                    }
                }

                self.eggs.retain(|e| !e.is_hatched());
                self.queens.retain(|q| q.is_alive());
                self.workers.retain(|w| w.is_alive());

                let msg = Message::new(self.stats.to_json_stringify(), self.stats.get_day());
                match self.tx.send(msg).await {
                    Ok(_) => (),
                    Err(v) => println!("Failed to send message over channel, err: {}", v),
                };
            }

            println!("Hive loop finished");
        }
    }

    fn age<T: Ant>(unit: &mut T, stats: &mut Statistics) {
        unit.ages();
        if !unit.is_alive() {
            unit.dies();

            match unit.unit_type() {
                UnitTypes::Queen => stats.increment_queen_dead(DeathType::Age),
                UnitTypes::Worker => stats.increment_worker_dead(DeathType::Age),
                _ => ()
            }
        }
    }

    fn feed<T: Ant>(storage: &mut Storage, unit: &mut T, stats: &mut Statistics) {
        let ate = unit.eat() as u64;
        if storage.get_food() < ate {
            unit.dies();
            
            match unit.unit_type() {
                UnitTypes::Queen => stats.increment_queen_dead(DeathType::Hunger),
                UnitTypes::Worker => stats.increment_worker_dead(DeathType::Hunger),
                _ => ()
            }

            return
        }

        storage.remove_food(ate);
    }

    fn produce_food<T: Ant>(storage: &mut Storage, worker: &mut T) {
        match worker.work() {
            Some(work_result) => {
                storage.add_food(work_result as u64);
            },
            None => return
        }
    }

    fn produce_egg<T: Ant>(eggs: &mut Vec<Egg>, queen: &mut T, stats: &mut Statistics) {
        match queen.work() {
            Some(_) =>  {
                eggs.push(Egg::new());
                stats.increment_eggs_not_hatched();
            },
            None => return
        }
    }

    fn hatch(egg: &mut Egg, workers: &mut Vec<Worker>, queens: &mut Vec<Queen>, stats: &mut Statistics) {
        egg.hatch();

        if rand::random::<f32>() < 0.9 {
            workers.push(Worker::new());
            stats.increment_worker_alive();
        } else {
            queens.push(Queen::new());
            stats.increment_queen_alive();
        }
            
        stats.increment_egg_hatched();
    }
}
