/*

    Author:     Sam Whitlock
    Date:       February 2, 2025

    Completed?  Yes
    Notes:
        There aren't any test cases in this file due to the complexity of writing a
        node tree in this way with all of the wrapping.

*/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    // The sums vector
    let mut sums: Vec<i64> = Vec::new();

    // Now to loop through and add sums to a vector
    add_to_vec(&mut sums, root, 0);

    // Making sure there are k layers
    if (k > sums.len() as i32) {
        return -1;
    }

    // Now we just have to sort it and find the kth one
    sums.sort();
    sums.reverse();

    println!("{:?}", sums);

    return sums[(k - 1) as usize];
}

// A recursive function that sums each thing to the layer's index
fn add_to_vec(sums: &mut Vec<i64>, raw_node: Option<Rc<RefCell<TreeNode>>>, layer_number: i64) {
    let node = match raw_node {
        Some(node) => Rc::try_unwrap(node)
            .expect("More than one reference exists")
            .into_inner(),
        None => TreeNode {
            val: -69,
            left: None,
            right: None,
        },
    };

    // An escape value if it is null
    if (node.val == -69) {
        return;
    }

    // Initializing the layer if it doesn't exist
    if (sums.len() <= layer_number as usize) {
        sums.push(node.val as i64);
    } else {
        // It does exist so we just need to add it
        let temp = sums[layer_number as usize];
        sums[layer_number as usize] = temp + node.val as i64;
    }

    // Calling the next two
    add_to_vec(sums, node.left, layer_number + 1);
    add_to_vec(sums, node.right, layer_number + 1);
}
