#[derive(Debug)]
pub struct Number {
    pub digits: [u32; 9],
    pub checksum: u32,
}

pub(crate) trait PatientIdentifier: TryFrom<[u32; 10]> {
    fn generate(count: Option<usize>) -> Vec<Self>;
}
