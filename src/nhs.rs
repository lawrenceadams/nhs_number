use crate::algorithm::generate_modulo_11_checksum;
use crate::errors::NumberValidationError;
use crate::number;
use crate::number::PatientIdentifier;
use rand::Rng;
use std::fmt::{Display, Formatter};

/// Struct that defines the NHS Number type.
/// Can be declared with `new` or from implementations.
#[derive(Debug)]
pub struct Number(number::Number);

impl PatientIdentifier for Number {
    fn generate(count: Option<usize>) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        let count = count.unwrap_or(1);
        let mut thr = rand::thread_rng();

        while result.len() < count {
            let mut digits = [0u32; 9];

            for digit in digits.iter_mut() {
                *digit = thr.gen_range(0..=9);
            }

            let checksum = generate_modulo_11_checksum(digits);

            match checksum {
                10 => continue,
                11 => result.push(Number(number::Number {
                    digits,
                    checksum: 0,
                })),
                _ => result.push(Number(number::Number { digits, checksum })),
            };
        }

        result
    }
}

impl TryFrom<[u32; 10]> for Number {
    type Error = NumberValidationError;

    fn try_from(value: [u32; 10]) -> Result<Self, Self::Error> {
        if let Ok(predicate) = <[u32; 9]>::try_from(&value[0..9]) {
            let checksum = generate_modulo_11_checksum(predicate);

            let checksum = match checksum {
                11 => 0,
                10 => return Err(NumberValidationError::InvalidChecksum),
                _ => checksum,
            };

            if value[9] != checksum {
                return Err(NumberValidationError::InvalidChecksum);
            }

            Ok(Number(number::Number {
                digits: predicate,
                checksum,
            }))
        } else {
            Err(NumberValidationError::ParsingError)
        }
    }
}

impl TryFrom<Vec<u32>> for Number {
    type Error = NumberValidationError;

    fn try_from(value: Vec<u32>) -> Result<Self, Self::Error> {
        let digits: [u32; 10] = value.try_into().expect("Unable to parse input string");

        Number::try_from(digits)
    }
}

impl TryFrom<String> for Number {
    type Error = NumberValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let digits: Vec<u32> = value
            .chars()
            .filter(|digit| digit.is_numeric())
            .map(|c| c.to_digit(10))
            .map(|val| val.unwrap())
            .collect();

        if digits.len() != 10 {
            return Err(NumberValidationError::InvalidLength);
        }

        Number::try_from(digits)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let unrolled_digits: String = self
            .0
            .digits
            .into_iter()
            .map(|c| c.to_string())
            .collect::<String>();
        let checksum_str = self.0.checksum.to_string();

        write!(f, "{unrolled_digits}{checksum_str}")
    }
}

#[cfg(test)]
#[test]
fn create_nhs_number_from_array() {
    assert!(Number::try_from([3, 4, 1, 7, 2, 8, 4, 9, 9, 6]).is_ok());
}

#[test]
fn create_nhs_number_from_string() {
    assert!(Number::try_from(String::from("357 593 9551")).is_ok());
}

#[test]
fn create_nhs_number_from_vec() {
    assert!(Number::try_from(vec![3, 4, 1, 7, 2, 8, 4, 9, 9, 6]).is_ok());
}

#[test]
fn test_generate_nhs_numbers() {
    let nhs_numbers = Number::generate(Some(1_000));
    assert_eq!(nhs_numbers.len(), 1_000);

    for number in nhs_numbers {
        let mut test_case: [u32; 10] = [0; 10];
        test_case[..9].copy_from_slice(&number.0.digits);
        test_case[9] = number.0.checksum;

        assert!(Number::try_from(test_case).is_ok());
    }
}
#[test]
fn test_display_for_number() {
    let display_string = crate::nhs::Number::try_from("298 876 0845".to_string()).unwrap();

    assert_eq!(format!("{display_string}"), "2988760845".to_string());
}
