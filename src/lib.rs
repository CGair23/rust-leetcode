mod data_structure;
mod s0001_two_sum;
mod s0002_add_two_numbers;
mod s0206_reverse_linked_list;
mod s0003_longest_substring_without_repeating_characters;

pub use s0206_reverse_linked_list::test_reverse_list;

// for benchmark
#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
