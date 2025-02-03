/*

    Author:     Sam Whitlock
    Date:       February 2, 2025

    Completed?  Yes
    Notes:
        None

*/

pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut longest: i32 = 1;
    let mut current: i32 = 0;
    let mut is_increasing = true;
    let mut last: i32 = -1;

    for num in nums.iter() {
        if last < *num {
            // We can increase the current if it is going up
            if is_increasing {
                current += 1;
            } else {
                // We need to reset current to 1 and change the direction
                current = 2;
                is_increasing = true;
            }
        } else if *num < last {
            // We can increase the current if it is going down
            if !is_increasing {
                current += 1;
            } else {
                // We need to reset current to 1 and change the direction
                current = 2;
                is_increasing = false;
            }
        } else {
            // We need to reset the current to 1
            current = 1;
        }

        // Updating the last
        last = *num;

        // Checking to see if it is longer than what we had before
        if current > longest {
            longest = current;
        }
    }
    longest
}

fn main() {
    let mut nums: Vec<i32> = vec![1, 4, 3, 3, 2];
    println!("Should be 2: {}", longest_monotonic_subarray(nums));

    nums = vec![3, 3, 3, 3];
    println!("Should be 1: {}", longest_monotonic_subarray(nums));

    nums = vec![3, 2, 1];
    println!("Should be 3: {}", longest_monotonic_subarray(nums));

    nums = vec![1, 4, 3, 3, 2];
    println!("Should be 2: {}", longest_monotonic_subarray(nums));
}
