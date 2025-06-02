use std::io::{stdin, stdout, Write};

// Calculate function
fn calculate (operator: char, numbers: (f64, f64)) -> f64 {
    match operator { 
        '+' => {
            numbers.0 + numbers.1
        }
        '-' => {
            numbers.0 - numbers.1
        }
        '*' => {
            numbers.0 * numbers.1
        }
        '/' => {
            numbers.0 / numbers.1
        }
        '^' => {
            numbers.0.powf(numbers.1)
        }
        _ =>{
            0.0
        }
    }
}

// Handle input
fn input_handling(){
    let mut operator = ' ';
    let mut numbers = (0.0, 0.0);
    let mut operator_input = String::new();
    let mut number_1_input = String::new();
    let mut number_2_input = String::new();
    
    // Operator
    print!("Operator (+, ): ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut operator_input)
        .expect("[ERR] => Failed to read operator");
    operator = operator_input.trim().chars().next().unwrap();
}


fn main() {
    println!("Welcome to CLI Calculator")
}
