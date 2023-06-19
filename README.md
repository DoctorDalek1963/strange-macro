# A strange macro

This repo contains a simple example of a strange macro. It should work and the output `TokenStream`
should be fine, but it doesn't compile properly and I have no idea why.

I want this (from `src/main.rs`):
```rust
fn main() {
    #[end_loop_in_test_or_bench]
    loop {
        let n = 10;
        println!("Looping");
        // More code here
    }
}
```
to expand to this:
```rust
fn main() {
    #[cfg(any(test, feature = "bench"))]
    let mut end_loop_in_test_or_bench_counter = 0u8;

    loop {
        let n = 10;
        println!("Looping");

        #[cfg(any(test, feature = "bench"))]
        {
            end_loop_in_test_or_bench_counter += 1;
            if end_loop_in_test_or_bench_counter > 100 {
                break;
            }
        }
    }
}
```

But it won't build and `cargo expand` gives the same errors as `cargo build` and shows the main
function empty.
