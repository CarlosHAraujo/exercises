fn main() {
    let number = 35;

    if number%5==0 && number%3==0 {
        println!("{} is a TriQunit", number);
    } else if number%6==0 && number%4==0 {
        println!("{} is a HexaQuad", number);
    } else {
        println!("{} is just another number",number);
    }


    let is_weekend:bool = false;
    let activity = if is_weekend {"go hiking"} else {"go to school"};
    println!("{} is today's activity", activity);

    
    let arr = [10, 20, 30, 40, 50];
    for elem in arr{
        println!("{}", elem);
    }


    let mut counter = 10;
    while counter>0{
        println!("Countdown: {}", counter);
        counter-=1;
    }
    println!("Lift off!");


    let mut index = 0;
    loop {
        index+=1;
        println!("index {}", index);

        if index==100{
            println!("Max index reached");
            break;
        }
    }


    let mut number = 1;
    let mut index = 0;
    let mut n1 = 0;
    let mut n2 = 0;
    while index<10{
        if index==0{
            println!("0");
            index+=1;
            continue;
        }
        println!("{},", number);
        index+=1;
        n2 = n1;
        n1 = number;
        number = n1 + n2;
    }

    let mut index = 0;
    let mut n1 = 0;
    let mut n2 = 0;
    loop {
        if index==10{
            break;
        }
        if index==0{
            println!("0, ");
        } else if index==1{
            println!("1, ");
            n1 = 1;
        } else {
            let number = n1 + n2;
            println!("{}, ", number);
            n2 = n1;
            n1 = number;
        }
        index+=1;        
    }


    let arr = [70,75,72,68,74,78,73];
    let mut total = 0;
    for temp in arr{
        total+=temp;
    }
    println!("The sum of temperatures is {}", total);
    println!("The average temperature is {}", total/arr.len());

    let temperatures = [70.1, 80.2, 75.3, 68.0, 71.8, 77.7, 65.5];
    let mut sum: f32 = 0.0;
    for temp in temperatures{
        sum+=temp;
    }

    let average = sum/temperatures.len() as f32;
    println!("The weekly sum is: {}", sum);
    println!("The average weekly temperature is: {}", average);
}
