fn main() {
    // 1. Store three numbers in a tuple
    let numbers = (1.1, 2.2, 3.3);
    
    println!("Tuple: {:?}", numbers);

    // 2. Destructure the tuple
    let (x, y, z) = numbers;

    println!("Destructure tuple: ({}, {}, {})", x, y, z);

    // 3. Calculate and print sum, average and product of the numbers
    let sum = x + y + z;
    let average = sum / 3.0;
    let product = x * y * z;

    println!("Sum: {}", sum);
    println!("Average: {}", average);
    println!("Product: {}", product);
}
