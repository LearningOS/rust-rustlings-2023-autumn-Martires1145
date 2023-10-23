// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

use std::mem::swap;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(_t) = my_option{

    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec = {
        let mut a = vec![1, 2, 3, 4, 5];
        a.resize(5, 0);
        a
    };
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45_i32;
    let mut value_b = 66_i32;
    // Let's swap these two!
    swap(&mut value_b, &mut value_a);
    println!("value a: {}; value b: {}", value_a, value_b);
}
