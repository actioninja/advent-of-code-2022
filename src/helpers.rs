/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn strip_newline(in_str: &str) -> String {
    in_str.replace("\r", "")
}
