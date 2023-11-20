use utilities::{
    get_i32::get_i32, make_random_vec::make_random_vec, print_vec::print_vec, quicksort::quicksort,
};

// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn binary_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut left: usize = 0;
    let mut right: usize = vec.len() - 1;

    let mut n_comparisons: i32 = 0;

    while left <= right {
        let m: usize = (left + right) / 2;
        n_comparisons += 1;
        if vec[m] < target {
            left = m + 1;
        } else if vec[m] > target {
            right = m - 1
        } else {
            return (m as i32, n_comparisons);
        }
    }
    return (-1, n_comparisons);
}

fn main() {
    let num_items = get_i32("Please specify the number of items: ");
    let max = get_i32("Please specify maximum: ");
    let target = get_i32("Please specify target: ");

    let mut random_vec = make_random_vec(num_items, max);
    print_vec(
        &random_vec,
        if num_items < 40 {
            num_items as usize
        } else {
            40
        },
    );
    quicksort(&mut random_vec);
    let (index, items_searched) = binary_search(&mut random_vec, target);
    println!("Searched {items_searched} items.");
    if index == -1 {
        println!("Item {target} not found.");
    } else {
        println!("Item {target} found at index {index} after searching {items_searched} items.");
    }
}
