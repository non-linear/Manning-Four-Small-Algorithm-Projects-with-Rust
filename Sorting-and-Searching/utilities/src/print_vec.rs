// Print at most num_items items.
use std::fmt::Display;

pub fn print_vec(vec: &Vec<impl Display>, num_items: usize) {
    let mut max = vec.len();
    if max > num_items {
        max = num_items;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}
