use std::io::{Write, stdin, stdout};

// Calculate function
fn calculate(operator: char, numbers: (f64, f64)) -> f64 {
    match operator {
        '+' => numbers.0 + numbers.1,
        '-' => numbers.0 - numbers.1,
        '*' => numbers.0 * numbers.1,
        '/' => numbers.0 / numbers.1,
        '^' => numbers.0.powf(numbers.1),
        _ => 0.0,
    }
}

// Handle input
fn input_handling() -> (char, (f64, f64)) {
    let mut operator = ' ';
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
    print!("Operator (+, -, *, /, ^): ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut operator_input)
        .expect("[ERR] => Failed to read operator");
    operator = operator_input.trim().chars().next().unwrap();

    (operator, numbers)
}

fn main() {
    println!("Welcome to CLI Calculator");

    loop {
        let calculation = input_handling();
        println!("Result: {}", calculate(calculation.0, calculation.1));
    }
}
