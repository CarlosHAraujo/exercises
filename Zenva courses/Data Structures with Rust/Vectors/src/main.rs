fn main() {
    // Vec<T>
    // heap allocated

    // let mut numbers: Vec<i32> = Vec::new();

    // numbers.push(10);
    // numbers.push(20);
    // numbers.push(30);

    // println!("{:?}", numbers);

    // vector macro

    // let mut numbers = vec![1,2,3,4,5];

    // println!("{:?}", numbers);

    // let mut fruits = vec!["apple", "banana", "orange"];

    // fruits.push("grape");

    // println!("{:?}", fruits);

    // let  removed_fruit = fruits.pop();

    // println!("{:?}, removed: {:?}", fruits, removed_fruit);

    // accessing and modifying elements

    // let numbers = vec![100, 200, 300, 400, 500, 600];

    // let second = numbers[1];

    // println!("The second element is: {}", second);

    // match numbers.get(5) {
    //     Some(value) => println!("The value at index 5 is: {}", value),
    //     None => println!("No value at index 5")
    // }

    // iterating over a vector

    // let animals = vec!["dog", "cat", "rabbit"];

    // for animal in &animals {
    //     println!("{}", animal);
    // }

    // let mut numbers = vec![1, 2, 3, 4, 5];

    // for number in &mut numbers {
    //     *number *= 2;
    // }

    // println!("{:?}", numbers);

    // Prealloc a vector

    let mut vec = Vec::with_capacity(10);

    for i in 0..10 {
        vec.push(i);
    }

    println!("Vector: {:?}, Capacity: {:?}", vec, vec.capacity());
}
