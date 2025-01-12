fn main() {
    let john = create_greeting("John", 25);
    
    println!("{}", john);
}

fn create_greeting(name:&str, age:u8) -> String {
    format!("My name is {} and I am {} years old.", name, age)
}
