fn main() {
    // Define the value of N (you can change this value)
    let n: u32 = 10;

    // Calculate the square of the sum
    let square_of_sum: u32 = (1..=n).sum::<u32>().pow(2);

    // Calculate the sum of the squares
    let sum_of_squares: u32 = (1..=n).map(|x| x * x).sum();

    // Calculate the difference
    let difference = square_of_sum - sum_of_squares;

    // Print the results
    println!("For the first {} natural numbers:", n);
    println!("Square of the sum: {}", square_of_sum);
    println!("Sum of the squares: {}", sum_of_squares);
    println!("Difference: {}", difference);
}
