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

    // Exhaustive search
    if NUM_ITEMS > 40 {
        // Only run exhaustive search if num_items is small enough.
        println!("Too many items for exhaustive search\n");
    } else {
        println!("*** Exhaustive Search ***");
        run_algorithm(&branch_and_bound, &mut items, allowed_weight);
    }
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

// Recursively assign values in or out of the solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
fn branch_and_bound(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i32) {
    let best_value = 0;
    let current_value = 0;
    let current_weight = 0;
    let remaining_value = items.iter().map(|x| x.value).sum();
    return do_branch_and_bound(
        items,
        allowed_weight,
        0,
        best_value,
        current_value,
        current_weight,
        remaining_value,
    );
}

fn do_branch_and_bound(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    next_index: i32,
    best_value: i32,
    current_value: i32,
    current_weight: i32,
    remaining_value: i32,
) -> (Vec<Item>, i32, i32) {
    let value_of_solution = solution_value(items, allowed_weight);
    if next_index as usize == items.len() {
        let copy_of_the_items = copy_items(items);
        return (copy_of_the_items, value_of_solution, 1);
    }

    if current_value + remaining_value <= best_value {
        return (Vec::<Item>::new(), value_of_solution, 1);
    }

    let test1_solution;
    let test1_value;
    let test1_calls;

    let test2_solution;
    let test2_value;
    let test2_calls;

    if current_weight + items[next_index as usize].weight <= allowed_weight {
        items[next_index as usize].is_selected = true;
        (test1_solution, test1_value, test1_calls) = do_branch_and_bound(
            items,
            allowed_weight,
            next_index + 1,
            best_value,
            current_value + items[next_index as usize].value,
            current_weight + items[next_index as usize].weight,
            remaining_value - items[next_index as usize].value,
        );
    } else {
        test1_solution = Vec::<Item>::new();
        test1_value = 0;
        test1_calls = 1;
    }

    if current_value + remaining_value - items[next_index as usize].value > best_value {
        items[next_index as usize].is_selected = false;
        (test2_solution, test2_value, test2_calls) = do_branch_and_bound(
            items,
            allowed_weight,
            next_index + 1,
            best_value,
            current_value,
            current_weight,
            remaining_value - items[next_index as usize].value,
        );
    } else {
        test2_solution = Vec::<Item>::new();
        test2_value = 0;
        test2_calls = 1;
    }

    if test1_value > test2_value {
        return (test1_solution, test1_value, test1_calls);
    } else {
        return (test2_solution, test2_value, test2_calls);
    }
}
