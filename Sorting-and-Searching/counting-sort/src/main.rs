use std::fmt;

use utilities::get_i32::get_i32;
use utilities::make_random_vec::make_random_vec;

#[derive(Clone)]
struct Customer {
    id: String,
    num_purchases: i32,
}
impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

fn make_random_customer_vec(num_items: i32, max: i32) -> Vec<Customer> {
    return make_random_vec(num_items, max)
        .into_iter()
        .zip(0..)
        .map(|(number, position): (i32, i32)| Customer {
            id: format!("C{position}").to_string(),
            num_purchases: number,
        })
        .collect();
}

fn check_sorted(vec_customer: &Vec<Customer>) {
    let vec = vec_customer
        .iter()
        .map(|customer| &customer.num_purchases)
        .cloned()
        .collect::<Vec<i32>>();
    utilities::check_sorted::check_sorted(&vec);
}

fn counting_sort(vec: &mut Vec<Customer>) {
    let length = vec.len();
    let dummy_customer = Customer {
        id: String::from(""),
        num_purchases: 0,
    };
    let mut output_vec = vec![dummy_customer; length];

    let max_num_purchases = vec.iter().fold(0, |acc: i32, customer: &Customer| {
        acc.max(customer.num_purchases)
    });
    let mut c = vec![0 as i32; max_num_purchases as usize + 1];
    for i in 0..length {
        c[vec[i].num_purchases as usize] += 1;
    }
    for i in 1..=max_num_purchases as usize {
        c[i] += c[i - 1];
    }

    for i in (0..length).rev() {
        let num_purchases = vec[i].num_purchases as usize;
        c[num_purchases] -= 1;

        let target_index = c[num_purchases as usize] as usize;
        output_vec[target_index] = vec[i].clone();
    }
    for i in 0..length {
        vec[i] = output_vec[i].clone();
    }
}

fn main() {
    let num_items = get_i32("Please specify the number of items: ");
    let max = get_i32("Please specify maximum: ");

    let mut random_vec = make_random_customer_vec(num_items, max);
    counting_sort(&mut random_vec);
    check_sorted(&random_vec);
}
