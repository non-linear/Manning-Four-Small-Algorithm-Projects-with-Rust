// Verify that the Vec is sorted.
pub fn check_sorted(vec: &Vec<i32>) {
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
