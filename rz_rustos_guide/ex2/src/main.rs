/// 1. println!
/// 2. format
fn main() {
    //! println test
    println!("Hello from the main.rs file!");

    println!("{} is from {}", "Brad", "Mass");

    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    println!(
        "{name} likse to play {activity}",
        name = "John",
        activity = "Golf"
    );

    println!("10+10={}", 10 + 10);

    let name = "Brad";
    let age: i32 = 20;

    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 0x51;
    println!("my ID: {}", ID);

    let person: (&str, &str, i8) = ("Brad", "Mass", 20);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    println!("Binary: {:b} Hex {:x} Octal {:o}", 10, 10, 10);

    println!("{:?}", (12, true, "hello"));
}
