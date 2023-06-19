#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

use strange_macros::end_loop_in_test_or_bench;

fn main() {
    #[end_loop_in_test_or_bench]
    loop {
        let n = 10;
        println!("Looping");
        // More code here
    }
}
