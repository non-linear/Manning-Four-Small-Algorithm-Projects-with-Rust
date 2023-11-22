// 1. Exhaustive search
use std::time::Instant;
use utilities::prng::Prng;
// use std::time::{SystemTime, UNIX_EPOCH};

const NUM_ITEMS: i32 = 20; // A reasonable value for exhaustive search.

const MIN_VALUE: i32 = 1;
const MAX_VALUE: i32 = 10;
const MIN_WEIGHT: i32 = 4;
const MAX_WEIGHT: i32 = 10;

struct Item {
    value: i32,
    weight: i32,
    is_selected: bool,
}

fn main() {
    // Prepare a Prng using the same seed each time.
    let mut prng = Prng { seed: 1337 };
    //prng.randomize();

    // Make some random items.
    let mut items = make_items(
        &mut prng, NUM_ITEMS, MIN_VALUE, MAX_VALUE, MIN_WEIGHT, MAX_WEIGHT,
    );
    let allowed_weight = sum_weights(&mut items, true) / 2;

    // Display basic parameters.
    println!("*** Parameters ***");
    println!("# items:        {}", NUM_ITEMS);
    println!("Total value:    {}", sum_values(&mut items, true));
    println!("Total weight:   {}", sum_weights(&mut items, true));
    println!("Allowed weight: {}", allowed_weight);
    println!();

    // Dynamic programming
    println!("*** Dynamic programming ***");
    run_algorithm(&dynamic_programming, &mut items, allowed_weight);
}

// Make some random items.
fn make_items(
    prng: &mut Prng,
    num_items: i32,
    min_value: i32,
    max_value: i32,
    min_weight: i32,
    max_weight: i32,
) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        let item = Item {
            value: prng.next_i32(min_value, max_value),
            weight: prng.next_i32(min_weight, max_weight),
            is_selected: false,
        };
        items.push(item);
    }
    return items;
}

// Return a copy of the items.
fn copy_items(items: &mut Vec<Item>) -> Vec<Item> {
    let mut new_items: Vec<Item> = Vec::with_capacity(items.len());
    for item in items {
        let new_item = Item {
            value: item.value,
            weight: item.weight,
            is_selected: item.is_selected,
        };
        new_items.push(new_item);
    }
    return new_items;
}

// Return the total value of the items.
// If add_all is true, add up all items.
// If add_all is false, only add up the selected items.
fn sum_values(items: &mut Vec<Item>, add_all: bool) -> i32 {
    if add_all {
        return items.iter().map(|item| item.value).sum();
    } else {
        return items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| item.value)
            .sum();
    }
}

// Return the total weight of the items.
// If add_all is false, only add up the selected items.
// If add_all is true, add up all items.
fn sum_weights(items: &mut Vec<Item>, add_all: bool) -> i32 {
    if add_all {
        return items.iter().map(|item| item.weight).sum();
    } else {
        return items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| item.weight)
            .sum();
    }
}

// Return the value of this solution.
// If the solution is too heavy, return -1 so we prefer an empty solution.
#[allow(dead_code)]
fn solution_value(items: &mut Vec<Item>, allowed_weight: i32) -> i32 {
    // If the solution's total weight > allowed_weight,
    // return -1 so even an empty solution is better.
    if sum_weights(items, false) > allowed_weight {
        return -1;
    }

    // Return the sum of the selected values.
    return sum_values(items, false);
}

// Print the selected items.
fn print_selected(items: &mut Vec<Item>) {
    let mut num_printed = 0;
    for i in 0..items.len() {
        if items[i].is_selected {
            print!("{}({}, {}) ", i, items[i].value, items[i].weight)
        }
        num_printed += 1;
        if num_printed > 100 {
            println!("...");
            return;
        }
    }
    println!();
}

// Run the algorithm. Display the elapsed time and solution.
fn run_algorithm(
    alg: &dyn Fn(&mut Vec<Item>, i32) -> (Vec<Item>, i32, i32),
    items: &mut Vec<Item>,
    allowed_weight: i32,
) {
    // Copy the items so the run isn't influenced by a previous run.
    let mut test_items = copy_items(items);

    let start = Instant::now();

    // Run the algorithm.
    let mut solution: Vec<Item>;
    let total_value: i32;
    let function_calls: i32;
    (solution, total_value, function_calls) = alg(&mut test_items, allowed_weight);

    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);

    print_selected(&mut solution);
    println!(
        "Value: {}, Weight: {}, Calls: {}",
        total_value,
        sum_weights(&mut solution, false),
        function_calls
    );
    println!();
}

// Use dynamic programming to find a solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
fn dynamic_programming(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    //  Make the function allocate the solution_value and prev_weight arrays so each has num_items rows and allowed_weight + 1 columns.
    // Allocate the arrays.
    let mut solution_value: Vec<Vec<i32>> = Vec::with_capacity(NUM_ITEMS as usize);
    let mut prev_weight: Vec<Vec<i32>> = Vec::with_capacity(NUM_ITEMS as usize);
    for _i in 0..NUM_ITEMS {
        solution_value.push(vec![0; (allowed_weight + 1) as usize]);
        prev_weight.push(vec![0; (allowed_weight + 1) as usize]);
    }

    // Initialize the row item 0.
    for w in 0..=allowed_weight {
        if items[0].weight <= w {
            // items[0] fits.
            solution_value[0][w as usize] = items[0].value;
            prev_weight[0][w as usize] = -1;
        } else {
            // items[0] does not fit.
            solution_value[0][w as usize] = 0;
            prev_weight[0][w as usize] = w;
        }
    }

    // Fill in the remaining table rows.
    for i in 1..NUM_ITEMS as usize {
        for w in 0..=allowed_weight as usize {
            items[i].is_selected = false;
            let value_without_item = sum_values(items, false);
            let weight_without_item = sum_weights(items, false);

            items[i].is_selected = true;
            let value_with_item = sum_values(items, false);

            if items[i].weight + weight_without_item <= allowed_weight
                && value_with_item > value_without_item
            {
                solution_value[i][w as usize] = value_with_item;
                prev_weight[i][w as usize] = w as i32;
            } else {
                solution_value[i][w as usize] = value_without_item;
                prev_weight[i][w as usize] = w as i32 - items[i].weight;
            }
        }
    }

    for i in 0..items.len() {
        items[i].is_selected = false;
    }

    // Reconstruct the solution.
    // Get the row and column for the final solution.
    let mut back_i = NUM_ITEMS - 1;
    let mut back_w = allowed_weight;

    // Work backwards until we reach an initial solution.
    while back_i >= 0 {
        // Check prev_weight for the current solution.
        let prev_w = prev_weight[back_i as usize][back_w as usize];
        if back_w == prev_w {
            // We skipped item back_i.
            // Leave back_w unchanged.
        } else {
            // We added item back_i.
            items[back_i as usize].is_selected = true; // Select this item in the solution.
            back_w = prev_w; // Move to the previous solution's weight.
        }
        back_i -= 1; // Move to the previous row.
    }

    // Reconstruct the solution.
    // Get the row and column for the final solution.
    let mut back_i = NUM_ITEMS - 1;
    let mut back_w = allowed_weight;

    // Work backwards until we reach an initial solution.
    while back_i >= 0 {
        // Check prev_weight for the current solution.
        let prev_w = prev_weight[back_i as usize][back_w as usize];
        if back_w == prev_w {
            // We skipped item back_i.
            // Leave back_w unchanged.
        } else {
            // We added item back_i.
            items[back_i as usize].is_selected = true; // Select this item in the solution.
            back_w = prev_w; // Move to the previous solution's weight.
        }
        back_i -= 1; // Move to the previous row.
    }
    // Return a copy of the items.
    let solution = copy_items(items);
    return (
        solution,
        solution_value[(NUM_ITEMS - 1) as usize][allowed_weight as usize],
        1,
    );
}
