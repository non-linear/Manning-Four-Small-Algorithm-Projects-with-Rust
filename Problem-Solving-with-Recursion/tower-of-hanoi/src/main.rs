const NUM_DISKS: usize = 7;

fn main() {
    // Make three posts with NUM_DISKS entries, all set to 0.
    let mut posts = [[0; NUM_DISKS]; 3];
    // Put the disks on the first post in order, smallest first (on top).
    for i in 0..NUM_DISKS {
        posts[0][i] = i + 1;
    }
    // Draw the initial setup.
    draw_posts(&posts);
    // Move the disks.
    move_disks(&mut posts, NUM_DISKS, 0, 1, 2);
    // Draw the final state.
    draw_posts(&posts);
    println!("Finished!");
}

// Draw the posts by showing the size of the disk at each level.
fn draw_posts(posts: &[[usize; NUM_DISKS]; 3]) {
    for disk in 0..NUM_DISKS {
        for post in 0..3 {
            print!("{} ", posts[post][disk]);
        }
        println!("");
    }
    println!("—————");
}

// Move one disk from from_post to to_post.
fn move_disk(posts: &mut [[usize; NUM_DISKS]; 3], from_post: usize, to_post: usize) {
    for from_post_position in 0..NUM_DISKS {
        let from_disk = posts[from_post][from_post_position];
        if from_disk > 0 {
            for to_post_position in (0..NUM_DISKS).rev() {
                if posts[to_post][to_post_position] == 0 {
                    posts[to_post][to_post_position] = from_disk;
                    posts[from_post][from_post_position] = 0;
                    return;
                }
            }
        }
    }
}

// Move the disks from from_post to to_post
// using temp_post as temporary storage.
fn move_disks(
    posts: &mut [[usize; NUM_DISKS]; 3],
    num_to_move: usize,
    from_post: usize,
    to_post: usize,
    temp_post: usize,
) {
    if num_to_move > 1 {
        move_disks(posts, num_to_move - 1, from_post, temp_post, to_post);
    }
    move_disk(posts, from_post, to_post);
    if num_to_move > 1 {
        move_disks(posts, num_to_move - 1, temp_post, to_post, from_post);
    }
}
