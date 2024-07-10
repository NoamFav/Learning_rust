// Define a struct
struct Person {
    name: String,
    age: u8,
}

// Implement methods for the struct
impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }

    fn say_goodbye(&self) {
        println!("Goodbye, {}!", self.name);
    }
}

// Define an enum
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// Implement methods for the enum
impl Day {
    fn mood(&self) {
        match self {
            Day::Monday => println!("Mondays are tough."),
            Day::Tuesday => println!("Tuesdays are better than Mondays."),
            Day::Wednesday => println!("Wednesdays are hump days."),
            Day::Thursday => println!("Thursdays are almost Fridays."),
            Day::Friday => println!("Fridays are great!"),
            Day::Saturday => println!("Saturdays are the best!"),
            Day::Sunday => println!("Sundays are relaxing."),
        }
    }
}

pub fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Call a method on the struct
    person.say_hello();

    // Create an instance of the enum
    let today = Day::Wednesday;

    // Call a method on the enum
    today.mood();

    person.say_goodbye();
}