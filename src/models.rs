#[derive(Debug)]
pub struct Device {
    pub ip: String,
    pub mac: Option<String>,
    pub status: Status,
}

#[derive(Debug)]
pub enum Status {
    Occupied,
    NotFound,
}
