fn main() {
    // Declare array, initialize all values, compiler infers length = 7
let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  
// Declare array, initialize all values to 0, length = 5
let bytes = [0; 5];

// Get the first day of the week
let first = days[0];

// Get the second day of the week
let second = days[1];

// Declare vector, initialize with three values
let three_nums = vec![15, 3, 46];
println!("Initial vector: {:?}", three_nums);  
  
// Declare vector, value = "0", length = 5
let zeroes = vec![0; 5];
println!("Zeroes: {:?}", zeroes); 

// Create empty vector, declare vector mutable so it can grow and shrink
let mut fruit: Vec<&str> = Vec::new();

// Push values onto end of vector, type changes from generic `T` to String
fruit.push("Apple");
fruit.push("Banana");
fruit.push("Cherry");
println!("Fruits: {:?}", fruit); 
}
