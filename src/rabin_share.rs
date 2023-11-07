#[derive(Debug, Clone)]
pub struct RabinShare {
    pub id: u8,
    pub original_length: usize,
    pub body: Vec<u8>,
}
