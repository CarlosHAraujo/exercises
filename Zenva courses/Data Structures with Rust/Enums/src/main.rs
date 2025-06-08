fn divide(numerator: i32, denominator: i32) -> Option<i32> {
    if denominator == 0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn read_file(file_path: &str) -> Result<String, String> {
    if file_path == "valid.txt" {
        Ok(String::from("File content here"))
    } else {
        Err(String::from("File not found!"))
    }
}

fn main() {
    // let result = divide(10, 2);
    
    // match result {
    //     Some(value) => println!("Result: {}", value),
    //     None => println!("Cannot divide by zero!")
    // }

    // let no_result = divide(10, 0);
    // match no_result {
    //     Some(value) => println!("Result: {}", value),
    //     None => println!("Cannot divide by zero!"),
    // }

    let file_result = read_file("valid.txt");
    match file_result {
        Ok(content) => println!("File read done: {}", content),
        Err(error) => println!("Error: {}", error),
    }

    let invalid_file_result = read_file("invalid.txt");
    match invalid_file_result {
        Ok(content) => println!("File read done: {}", content),
        Err(error) => println!("Error: {}", error),
    }
}
