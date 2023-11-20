use utilities::{get_i32::get_i32, make_random_vec::make_random_vec, print_vec::print_vec};

// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn linear_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    for i in 0..vec.len() as i32 {
        if vec[i as usize] == target {
            return (i, i + 1);
        }
    }
    return (-1, vec.len() as i32);
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
    let (index, items_searched) = linear_search(&mut random_vec, target);
    println!("Searched {items_searched} items.");
    if index == -1 {
        println!("Item {target} not found.");
    } else {
        println!("Item {target} found at index {index} after searching {items_searched} items.");
    }
}
