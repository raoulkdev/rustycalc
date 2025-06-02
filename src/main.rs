// Operator enum
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

// Calculate function
fn calculate (operator: Operator, numbers: (f64, f64)) -> f64 {
    match operator { 
        Operator::Add => {
            numbers.0 + numbers.1
        }
        Operator::Subtract => {
            numbers.0 - numbers.1
        }
        Operator::Multiply => {
            numbers.0 * numbers.1
        }
        Operator::Divide => {
            numbers.0 / numbers.1
        }
        Operator::Power => {
            numbers.0.powf(numbers.1)
        }
    }
}


fn main() {
    println!("Welcome to CLI Calculator")
}
