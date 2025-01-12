struct Color(i32,i32,i32); // RGB

struct Point(f64,f64); //xy coord

struct User {
    name:String,
    email:String,
    active:bool,
    sign_in_count:u64,
    date_of_birth:String,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn Area(&self) -> u32 {
        self.width * self.height
    }

    fn print_dimensions(&self) {
        println!("Height {} Width {}", self.height, self.width);
    }
}

fn print_user_message(user:&User) {
    println!("Hello {}, you have signed in {}", user.name, user.sign_in_count);
}

struct Book {
    title: String,
    author: String,
    number_of_pages: u32
}

impl Book {
    fn get_summary(&self) -> String {
        format!("Book {} of {} has {} pages", self.title, self.author, self.number_of_pages)
    }
}

fn main() {
    let books = [Book {
        title: String::from("Book 1"),
        author: String::from("John"),
        number_of_pages: 155,
    }, Book {
        title: String::from("Book 2"),
        author: String::from("John"),
        number_of_pages: 300,
    }, Book {
        title: String::from("Book 3"),
        author: String::from("John"),
        number_of_pages: 329,
    }, Book {
        title: String::from("Book 4"),
        author: String::from("John"),
        number_of_pages: 500,
    }, Book {
        title: String::from("Book 5"),
        author: String::from("John"),
        number_of_pages: 200,
    }];

    for book in books {
        println!("{}", book.get_summary());
    }

    
    let user1 = User {
        name: String::from("John"),
        email: String::from("john@email.com"),
        active: true,
        sign_in_count: 5,
        date_of_birth: String::from("12/12/1932"),
    };

    println!("{} {}", user1.name, user1.email);


    print_user_message(&user1);


    let red = Color(255,0,0);
    let p1 = Point(4.3,2.5);


    let rect1 = Rectangle {
        width: 25,
        height: 20,
    };
    let area = rect1.Area();
    println!("The rectangle area is: {}", area);
}
