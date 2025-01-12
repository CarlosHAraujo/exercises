enum TrafficLight {
    Red,
    Green,
    Yellow
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(i32)
}

fn calculate_area(shape:Shape) {
    match shape {
        Shape::Circle(radius) => println!("Area of circle is {}", 3.14*radius*radius),
        Shape::Rectangle(width, height) => {
            let area = width*height;
            println!("Area of rectangle is {}", area);
        },
        Shape::Square(side) => println!("Area of square is {}", side*side),
    }
}

fn safe_divide(numerator:f64,denominator:f64) -> Option<f64> {
    if denominator==0.0 {
        None
    } else {
        Some(numerator/denominator)
    }
}

enum CarType {
    SUV,
    Sedan,
    Coupe,
}

enum Vehicle {
    Car(CarType),
    Truck(u32),
    Motorcycle
}

impl Vehicle {
    fn parking_rate(&self) -> i8 {
        match self {
            Vehicle::Car(carType) => match carType {
                CarType::Coupe => 10,
                CarType::SUV => 20,
                CarType::Sedan => 15
            },
            Vehicle::Truck(capacity) => if capacity>&10 { 25 } else { 20 },
            Vehicle::Motorcycle => 10
        }
    }
}

fn main() {
    let vehicles = [Vehicle::Car(CarType::Coupe), Vehicle::Truck(10), Vehicle::Motorcycle, Vehicle::Car(CarType::SUV)];
    for vehicle in vehicles {
        println!("The parking value is {}", vehicle.parking_rate());
    }


    let light = TrafficLight::Red;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Attention"),
        TrafficLight::Green => println!("Go")
    }


    let rect1 = Shape::Rectangle(30.0, 17.3);
    let square1 = Shape::Square(25);

    calculate_area(rect1);
    calculate_area(square1);


    let result = safe_divide(30.0, 0.0);
    match result {
        Some(value) => println!("Succesful division: {}", value),
        None => println!("Cannot divide by zero")
    }


    if let TrafficLight::Red = light {
        println!("You shall not pass!");
    } else if let TrafficLight::Yellow = light {
        println!("Be ready to stop");
    }
}
