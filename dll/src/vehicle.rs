#[derive(Debug)]
pub struct Vehicle {
    pub address: usize
}

impl Vehicle {
    pub fn new(address: usize) -> Self {
        Self { address }
    }

    pub fn get_health(&self) -> f32 {
        unsafe { *((self.address + 0x4C0) as *const f32) }
    }
}