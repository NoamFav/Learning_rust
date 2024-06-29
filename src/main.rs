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
}

// Define an enum
enum Day {
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
            Day::Friday => println!("Fridays are great!"),
            _ => println!("Another day."),
        }
    }
}

fn main() {
    // Create an instance of the struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Call a method on the struct
    person.say_hello();

    // Create an instance of the enum
    let today = Day::Monday;

    // Call a method on the enum
    today.mood();
}
