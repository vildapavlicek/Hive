mod unit;
mod storage;

pub enum UnitTypes {
    Egg,
    Queen,
    Worker,
}

#[derive(Debug)]
pub struct Hive {
    queens: Vec<unit::Queen>,
    workers: Vec<unit::Worker>,
    eggs: Vec<unit::Egg>,
    storage: storage::Storage,
}

impl Hive {
    pub fn new() -> Self {
        Hive {
            queens: vec![unit::Queen::new()],
            workers: unit::Worker::generate_workers(20),
            eggs: vec![unit::Egg::new()],
            storage: storage::Storage::new(5000),
        }
    }

    pub fn run(hive: &mut Hive) {
        let mut cycle: u64 = 0;
        let mut run: bool = true;

        while run {
            if cycle == 50 {
                run = false;
            }


            for queen in hive.queens.iter_mut() {
                feed(&mut hive.storage, queen);
            }

            cycle += 1;
        }
    }
}

fn feed<T: unit::Eat>(storage: &mut storage::Storage, unit: &T) {
    let ate = unit.eat() as u64;
    storage.remove_food(ate);
}
