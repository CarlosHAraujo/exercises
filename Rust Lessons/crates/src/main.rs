use rand::Rng;
mod messages;
mod calculation;
use num::integer::gcd_lcm;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number:u32 = rng.gen_range(1..1000);
    println!("Random number {}", random_number);


    messages::greet("John");
    let difference = calculation::subtract(25,15);

    println!("The difference is {}", difference);


    let a = 420;
    let b = 69;

    let result = gcd_lcm(a,b);
    println!("LCM and GCD of {} and {} are: {} {} respectively", a, b, result.0, result.1);
}
