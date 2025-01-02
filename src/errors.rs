#[derive(Debug)]
pub enum NumberValidationError {
    InvalidChecksum,
    ParsingError,
    InvalidLength,
}
