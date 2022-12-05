use std::{mem::transmute};

pub struct Pool {}

pub fn get_vehicle(handle: i32) -> usize {
    let func: fn(handle: i32) -> usize = unsafe {transmute(0x54FFF0 as *const())};
    func(handle)
}