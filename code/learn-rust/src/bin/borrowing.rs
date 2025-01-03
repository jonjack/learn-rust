use rand::random;
use std::collections::hash_map::HashMap;

fn main() {
    let mut s1 = String::from("Hello");
    let len = change(&mut s1); // ownership of s1 gets moved to the function's scope

    println!("s1 = {s1}"); // compile error because s1 is no longer owned here
}

fn length1(s: String) -> (usize) {
    s.len()
}

fn length2(s: &String) -> (usize) {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" World");
}
