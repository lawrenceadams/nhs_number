/// Generates the modulo 11 checksum used in NHS and CHI Numbers
/// Uses a vector of `u32` and emits the final checksum digit for the input
pub(crate) fn generate_modulo_11_checksum(predicate: [u32; 9]) -> u32 {
    let checksum_digit: u32 = predicate
        .iter()
        .enumerate()
        .map(|(idx, digit)| digit * (10 - idx as u32))
        .sum();

    11 - checksum_digit % 11
}
