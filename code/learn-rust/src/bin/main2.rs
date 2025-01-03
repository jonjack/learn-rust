use multiple_binaries::utils;

fn main() {
    let sum = utils::add_numbers(5, 3);
    println!("Sum: {}", sum);

    let greeting = utils::greet("Alice");
    println!("{}", greeting);
}
