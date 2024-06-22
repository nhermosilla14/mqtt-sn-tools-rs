
pub trait SensorNet {
    fn init(&mut self);
    fn get_description(&self) -> String;
    fn receive(&mut self) -> Result<Vec<u8>, std::io::Error>;
    fn send(&mut self, data: &[u8]) -> Result<usize, std::io::Error>;
    fn get_timeout(&self) -> u64;
}