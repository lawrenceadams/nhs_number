use crate::core::algorithm::generate_modulo_11_checksum;
use rand::Rng;

pub struct NhsNumber;

trait PatientIdentifier {
    fn generate(count: Option<usize>) -> Vec<String>;
    fn validate(input: String) -> bool;
}

impl PatientIdentifier for NhsNumber {
    fn generate(count: Option<usize>) -> Vec<String> {
        let count = count.unwrap_or(1);

        let mut result: Vec<String> = vec![];

        let mut thr = rand::thread_rng();
        while result.len() < count {
            let mut chars: Vec<u32> = vec![];

            chars.extend(
                std::iter::once(thr.gen_range(1..=9))
                    .chain(std::iter::repeat_with(|| thr.gen_range(0..=9)).take(8)),
            );

            let checksum = generate_modulo_11_checksum(&chars);

            if checksum == 10 {
                break;
            }

            if checksum == 11 {
                chars.push(0);
            } else {
                chars.push(checksum);
            }

            let res: String = chars.iter().map(ToString::to_string).collect();

            result.push(res);
        }
        result
    }

    fn validate(input: String) -> bool {
        crate::core::algorithm::modulo_11(&input)
    }
}

#[cfg(test)]
#[test]
fn validate_nhs_numbers() {
    assert!(NhsNumber::validate(String::from("9434765919")));
    assert!(NhsNumber::validate(String::from("341 728 4996")));
    assert!(!NhsNumber::validate(String::from("12312")));
}

#[test]
fn test_generate_nhs_numbers() {
    let nhs_numbers = NhsNumber::generate(Some(1000));

    for number in nhs_numbers {
        assert!(NhsNumber::validate(number));
    }
}
