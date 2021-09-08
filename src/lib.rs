mod reverse_linked_list;

pub use reverse_linked_list::test_reverse_list;

// for benchmark
#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
