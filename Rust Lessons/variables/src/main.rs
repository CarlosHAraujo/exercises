fn main() {
    let mut value = 9;
    value = 10;
    println!("The value is {}", value);

    let x = 64;
    let x = x + 1; // Shadowing
    let x = "RUST PROGRAMMING";

    println!("The value of x is {}", x);

    let greeting: &str = "Hello, world!";
    println!("{}", greeting);

    let mut name = String::from("Zenva");
    name.push_str(" Academy");
    println!("{}", name);

    let name = "John Doe";
    let mut age = 45;
    const YEAR: i8 = 25;
    age = age + YEAR;

    println!("{} age in the year of 2050 will be: {}", name, age);
}
