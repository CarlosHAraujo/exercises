fn main() {
    let name = String::from("John");
    greet_user(name);

    let sum = calculate_sum(5, 10);
    println!("The sum is {}", sum);

    let scores = [85, 90, 78, 92, 88];
    for score in scores {
        let grade = get_letter_grade(score);
        println!("The score is {} and the corresponding grade is {}", score, grade);
    }
}

fn greet_user(name:String) {
    println!("Hello {}, welcome to rust programming", name);
}

fn calculate_sum(a:i32, b:i32) -> i32 {
    let sum = a + b;
    sum
}

fn get_letter_grade(grade:i32) -> char {
    if grade>=90 && grade<=100 {
        'A'
    } else if grade>=80 && grade<90 {
        'B'
    } else if grade>=70 && grade<80 {
        'C'
    } else if grade>=60 && grade<70 {
        'D'
    } else {
        'F'
    }
}
