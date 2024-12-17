use crate::core::algorithm::generate_modulo_11_checksum;
use rand::Rng;

pub struct NhsNumber;
pub struct ChiNumber;

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
            // let mut chars: Vec<usize> = (0..9).map(|_| thr.gen_range(0..=9)).collect();
            let mut chars: usize = thr.gen_range(100_000_0000..999_999_9999);
            let checksum = generate_modulo_11_checksum(&chars);

            chars.push(checksum);

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
    assert_eq!(NhsNumber::validate("9434765919".into()), true);
    assert_eq!(NhsNumber::validate("4785027629".into()), true);
    assert_eq!(NhsNumber::validate("12312".into()), false)
}

#[test]
fn test_generate_nhs_numbers() {
    dbg!(NhsNumber::generate(Some(10)));
    assert!(false);
}
