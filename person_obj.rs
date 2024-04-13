// Define a struct named `Person` with two fields: `name` of type `String` and `age` of type `u32`.
struct Person {
    name: String,
    age: u32,
}

// Implementation block for the `Person` struct.
impl Person {
    // Function to create a new `Person` instance.
    fn new(name: String, age: u32) -> Self {
        // Return a new `Person` instance with the provided `name` and `age`.
        Self { name, age }
    }

    // Function to print the details of a `Person`.
    fn print_details(&self) {
        // Print the name and age of the person.
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
    }
}

fn main() {
    // Create a new `Person` instance using the `new` function.
    let person1 = Person::new(String::from("Alice"), 30);

    // Call the `print_details` function on `person1` to print its details.
    person1.print_details();
}
