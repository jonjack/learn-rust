// run with: cargo run --bin variables
use rand::random;

fn main() {
    // let s1: str = "s1";
    // let s2: str = "s1";

    // println!("s1 = {s1}, s2 = {s2}");

    // let i1: i32 = (random()).to;
    //i1.to_string();

    //println!("i1 = {i1}");

    let s1 = String::from("hello");
    println!("s1 = {s1}");
    some_func(s1);
    println!("s1 = {s1}");
}

fn some_func(s1: String) {
    println!("some_func() now owns s1: {s1}");
}