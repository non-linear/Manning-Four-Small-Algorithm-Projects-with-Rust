// use std::ops::{IndexMut, Range};

use utilities::check_sorted::check_sorted;
use utilities::get_i32::get_i32;
use utilities::make_random_vec::make_random_vec;

// Use quicksort to sort the vector.
fn quicksort(vec: &mut [i32]) {
    // print!("Calling quicksort with length: {}", vec.len());
    let n: usize = vec.len();
    if n <= 1 {
        return;
    }

    let pivot = partition(vec);

    quicksort(&mut vec[0..pivot]);
    quicksort(&mut vec[pivot + 1..]);
}

// partition: helper function for quicksort
fn partition(vec: &mut [i32]) -> usize {
    let length = vec.len();
    if length <= 1 {
        return 0;
    }
    let low: usize = 0;
    let high: usize = length - 1;

    let pivot = vec[high];
    let mut temporary_pivot_index: i32 = low as i32 - 1;

    for j in low..high {
        if vec[j] <= pivot {
            temporary_pivot_index += 1;
            vec.swap(temporary_pivot_index as usize, j);
        }
    }
    temporary_pivot_index += 1;
    vec.swap(temporary_pivot_index as usize, high);
    temporary_pivot_index as usize
}

fn main() {
    let num_items = get_i32("Please specify the number of items: ");
    let max = get_i32("Please specify maximum: ");

    let mut random_vec = make_random_vec(num_items, max);
    quicksort(&mut random_vec);
    check_sorted(&random_vec);
}
