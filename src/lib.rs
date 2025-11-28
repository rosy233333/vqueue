#![no_std]

mod deque;
mod slot_array;

vdso_helper::use_mut_cfg! {}

#[cfg(test)]
mod test_mut_cfg {
    extern crate std;

    use super::*;
    use std::println;

    // run with `cargo test test_constants -- --nocapture`
    #[test]
    fn test_constants() {
        println!("QUEUE_LEN: {}", QUEUE_LEN);
        println!("ARRAY_LEN: {}", ARRAY_LEN);
        // println!("BOOL_TEST: {}", BOOL_TEST);
        // println!("EXPR_TEST: {}", EXPR_TEST);
        // println!("FLOAT_TEST: {}", FLOAT_TEST);
    }
}
