fn main() {
    // 1. Declare an immutable variable with age
    let age = 28;

    // 2. Declare a mutable variable with favorite number and change it
    let mut number = 1;
    number = 14;

    // 3. Use shadowing to convert a string to its length
    let characters = "abcdefgh";
    let characters = characters.len();

    // 4. Print all values
    println!("Age: {}", age);
    println!("Number: {}", number);
    println!("Characters length: {}", characters);
}
