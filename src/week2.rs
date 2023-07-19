use std::io;

enum Operation {
        Add(f64, f64),
        Subtract(f64, f64),
        Multiply(f64, f64),
        Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
        match op {
                Operation::Add(a, b) => a + b,
                Operation::Subtract(a, b) => a - b,
                Operation::Multiply(a, b) => a * b,
                Operation::Divide(a, b) => a / b,
        }
}

fn main() {
        println!("enter first number: ");
        let mut number1 = String::new();
        io::stdin().read_line(&mut number1);
        let number1:f64 = number1.trim().parse().expect("");

        println!("enter the operation");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation);
        let operation:char = operation.trim().chars().next().expect("");

        println!("enter second number: ");
        let mut number2 = String::new();
        io::stdin().read_line(&mut number2);
        let number2:f64 = number2.trim().parse().expect("");

        let operation_enum = match operation {
        '+' => Operation::Add(number1, number2),
        '-' => Operation::Subtract(number1, number2),
        '*' => Operation::Multiply(number1, number2),
        '/' => Operation::Divide(number1, number2),
        _ => panic!("Invalid operation. Please enter +, -, *, or /"),
    };

    let result = calculate(operation_enum);
    println!("Result: {}", result);
}