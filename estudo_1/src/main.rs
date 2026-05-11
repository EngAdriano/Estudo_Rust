fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the first number:");
    std::io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    println!("Enter the second number:");
    std::io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");

    let num1: i32 = input1.trim().parse().expect("Invalid number");
    let num2: i32 = input2.trim().parse().expect("Invalid number");
    let sum = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, sum);
}


