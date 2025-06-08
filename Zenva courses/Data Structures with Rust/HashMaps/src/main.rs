use std::collections::HashMap;
mod challenge;

fn main() {
    // HashMap<K, V>

    // let mut population = HashMap::new();

    // population.insert("Tokyo", 37_400_100);
    // population.insert("London", 17_400_100);
    // population.insert("Dubai", 7_400_100);

    //println!("{:?}", population);

    // match population.get("Tokyo") {
    //     Some(&pop) => println!("Population of Tokyo: {}", pop),
    //     None => println!("City not found")
    // }

    // Updating

    // population.insert("Tokyo", 47_000_000);

    // println!("{:?}", population);

    // population.remove("Tokyo");

    // println!("{:?}", population);

    // Iteration

    // for (city, pop) in &population {
    //     println!("{}: {}", city, pop);
    // }

    // for value in population.values() {
    //     println!("Population: {}", value);
    // }

    // population.entry("Delhi").or_insert(32_000_000);
    // population.entry("Dubai").or_insert(9_000_000);

    // println!("{:?}", population);

    // let mut scores = HashMap::with_capacity(10);

    // for i in 0..5 {
    //     scores.insert(i, i * 10);
    // }

    // println!("{:?}", scores);
    // println!("Capacity: {}", scores.capacity());

    challenge::challenge();
}
