// Create a function that finds out the average of several numbers and returns it
fn average(numbers: Vec<i64>) -> f64 {
    let sum: i64 = numbers.iter().sum();
    let count = numbers.len();
    let average = sum as f64 / count as f64;
    // do a loop so that it prints out the numbers
    for number in &numbers {
        println!("{}", number);     
    }
    average
}

fn main() {
    println!("Hello, world!");
    let item = 5;
}
