#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

impl From<[u8; 12]> for Vector3 {
    fn from(value: [u8; 12]) -> Self {
        let x = f32::from_le_bytes(value[0..4].try_into().unwrap());
        let y = f32::from_le_bytes(value[4..8].try_into().unwrap());
        let z = f32::from_le_bytes(value[8..12].try_into().unwrap());
        Vector3 {x, y, z}
    }
}

impl Into<[u8; 12]> for Vector3 {
    fn into(self) -> [u8; 12] {
        unsafe { any_as_u8_slice(&self).try_into().unwrap() }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        std::mem::size_of::<T>(),
    )
}