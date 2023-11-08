mod prng;
mod utils;

use utils::{get_i32, make_random_vec, print_vec};

// Use bubble sort to sort the vector.
fn bubble_sort(vec: &mut Vec<i32>) {
    let mut n: usize = vec.len();
    while n > 1 {
        let mut new_n: usize = 0;
        for i in 1..n {
            if vec[i - 1] > vec[i] {
                vec.swap(i, i - 1);
                new_n = i;
            }
        }
        n = new_n;
    }
}

// Verify that the Vec is sorted.
fn check_sorted(vec: &Vec<i32>) {
    let mut minimum = i32::MIN;
    for item in vec {
        if item < &minimum {
            println!("The vector is NOT sorted!");
            return;
        } else {
            minimum = *item;
        }
    }
    print!("The vector is sorted!");
}

fn main() {
    let num_items = get_i32("Please specify the number of items: ");
    let max = get_i32("Please specify maximum: ");

    let mut random_vec = make_random_vec(num_items, max);
    print_vec(&random_vec, num_items);
    bubble_sort(&mut random_vec);
    print_vec(&random_vec, num_items);
    check_sorted(&random_vec);
}
