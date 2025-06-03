// Imports
use std::io::{Write, stdin, stdout};

// Operator enum
enum Operator{
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Empty,
    Exit
}

// Calculate function
fn calculate(operator: Operator, numbers: (f64, f64)) -> f64 {
    match operator {
        Operator::Add => numbers.0 + numbers.1,
        Operator::Subtract => numbers.0 - numbers.1,
        Operator::Multiply => numbers.0 * numbers.1,
        Operator::Divide => numbers.0 / numbers.1,
        Operator::Power => numbers.0.powf(numbers.1),
        Operator::Empty => 0.0,
        Operator::Exit => std::process::exit(0)
    }
}

// Handle input
fn input_handling() -> (Operator, (f64, f64)) {
    let operator: Operator;
    let mut numbers = (0.0, 0.0);
    let mut operator_input = String::new();
    let mut number_1_input = String::new();
    let mut number_2_input = String::new();

    // First number
    print!("First number: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut number_1_input)
        .expect("[ERR] => Failed to read first number input");
    numbers.0 = number_1_input.trim().parse().unwrap();

    // Second number
    print!("Second Number: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut number_2_input)
        .expect("[ERR] => Failed to read second number input");
    numbers.1 = number_2_input.trim().parse().unwrap();

    // Operator
    print!("Operator (+, -, *, /, ^, e to exit): ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut operator_input)
        .expect("[ERR] => Failed to read operator");
    match operator_input.trim().chars().next().unwrap() {
        '+' => operator = Operator::Add,
        '-' => operator = Operator::Subtract,
        '*' => operator = Operator::Multiply,
        '/' => operator = Operator::Divide,
        '^' => operator = Operator::Power,
        'e' => operator = Operator::Exit,
        _ => operator = Operator::Empty
    }

    // Return operator and first + second number tuple
    (operator, numbers)
}

fn main() {
    // Welcome message
    println!("Welcome to rustycalc");

    // Continuous input handling and calculation unless 'e' operator
    loop {
        // The user's calculation request: Operator, first and second number
        let calculation_request = input_handling();

        // Result of the user's calculation request
        let result = calculate(calculation_request.0, calculation_request.1);

        // Print result
        println!("Result: {}", result);
    }
}