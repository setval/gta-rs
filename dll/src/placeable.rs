#![allow(dead_code)]
use std::{ffi::c_void, mem::transmute};

use crate::game::Vector3;

#[derive(Debug)]
pub struct Placeable {
    address: usize
}

impl Placeable {
    pub fn new(address: usize) -> Self {
        Self {
            address,
        }
    }

    pub fn get_position_point(&self) -> usize {
        unsafe { *((self.address + 0x14) as *const usize) }
    }

    pub fn get_position(&self) -> Vector3 {
        Vector3::from(unsafe { *((self.get_position_point() + 0x30) as *const [u8; 12])})
    }

    pub fn set_position(&self, position: Vector3) {
        // unsafe { *((self.get_position_point() + 0x30) as *mut [u8; 12]) = position.into(); }
        
        let func: extern "thiscall" fn(*const c_void, f32, f32, f32) = unsafe { transmute(0x420B80 as *const()) };
        func(
            self.address as *const c_void,
            position.x,
            position.y,
            position.z
        )
    }
}