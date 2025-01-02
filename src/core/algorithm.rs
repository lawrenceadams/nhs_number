/// Returns a boolean representing whether the input given is valid.
/// Accepts an input string slice, and parses it into a Vec<u32>.
/// To use the algorithm directly, see `generate_modulo_11_checksum`.
/// ```
/// use crate::nhs_number::core::algorithm::modulo_11;
/// assert!(modulo_11("2706031867"));
/// ```
pub fn modulo_11(predicate: &str) -> bool {
    let input = clean_input(predicate);

    if input.len() != 10 {
        return false;
    }

    let parsed_string: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10))
        .map(|val| val.unwrap())
        .collect();

    let checksum_digit = generate_modulo_11_checksum(&parsed_string[0..9]);

    checksum_digit == *parsed_string.last().unwrap()
}

/// Generates the modulo 11 checksum used in NHS and CHI Numbers
/// Uses a vector of `u32` and emits the final checksum digit for the input
pub fn generate_modulo_11_checksum(predicate: &[u32]) -> u32 {
    if predicate.len() != 9 {
        panic!("Somehow passed an invalid length of string")
    }

    let checksum_digit: u32 = predicate
        .iter()
        .enumerate()
        .take(9)
        .map(|(idx, digit)| digit * (10 - idx as u32))
        .sum();

    11 - checksum_digit % 11
}

/// Strips spaces that can be found in NHS Numbers (e.g. `xxx-yyy-zzzz` should be `xxxyyyzzzz`)
fn clean_input(input: &str) -> String {
    input.replace(" ", "")
}
