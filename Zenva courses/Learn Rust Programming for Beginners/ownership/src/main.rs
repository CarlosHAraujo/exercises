fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{s2} world");


    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{s1}");
    println!("{s2}");


    let name = String::from("John");
    print_greeting(&name);
    println!("{name}");


    let y = 5;
    let x = y;
    println!("y = {y}, x = {x}");


    let message = String::from("Hello, world!");
    let hello = &message[0..5];
    println!("{}", hello);


    let arr = [1,2,3,4,5];
    let slice = &arr[1..4];
    for num in slice {
        println!("{}", num);
    }

    
    let arr = [1,2,3,4,5];
    let slice = &arr[1..3];
    println!("{:?}", slice);
}

fn print_greeting(str:&String) {
    println!("Welcome {str}");
}
