mod hive;

fn main() {
    println!("Hello, world!");
    let hive = hive::Hive::new();

    println!("{:?}", hive);
}