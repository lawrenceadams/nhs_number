pub fn modulo_11(predicate: &str) -> bool {
    let input = clean_input(predicate);

    println!("{}", input.len());

    if input.len() != 10 {
        return false;
    }

    let parsed_string: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10))
        .map(|val| val.unwrap())
        .collect();

    let checksum_digit: u32 = parsed_string
        .iter()
        .enumerate()
        .take(9)
        .map(|(idx, digit)| digit * (10 - (idx as u32)))
        .sum();

    dbg!(&parsed_string);
    dbg!(&checksum_digit);
    dbg!(11 - (&checksum_digit % 11));

    11 - (checksum_digit % 11) == *parsed_string.last().unwrap()
}

pub fn generate_modulo_11_checksum(predicate: &Vec<usize>) -> usize {
    // if predicate.len() != 9 {
    //     panic!("Somehow passed an invalid length of string")
    // }

    let checksum_digit: usize = predicate
        .iter()
        .enumerate()
        .take(9)
        .map(|(idx, digit)| digit * (10 - idx))
        .sum();

    11 - checksum_digit % 11
}

/// Cleans up input for further consumption
fn clean_input(input: &str) -> String {
    input.replace(" ", "")
}
