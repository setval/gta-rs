#![allow(dead_code)]




use std::mem::transmute;
use std::ffi::c_void;


use crate::{vehicle::Vehicle};
use crate::physical::Physical;

pub enum PedType {
    Player1 = 0,
    Player2 = 1,
    PlayerNetwork = 2,
    PlayerUnused = 3,
    Civmale = 4,
    Civfemale = 5,
}

#[derive(Debug)]
pub struct Ped {
    pub address: usize,
    pub physical: Physical,
}

impl Ped {
    pub fn new(address: usize) -> Self {
        Self {
            address,
            physical: Physical::new(address),
        }
    }

    pub fn get_health(&self) -> f32 {
        unsafe { *((self.address + 0x540) as *const f32) }
    }

    pub fn set_health(&self, health: f32) {
        unsafe { *((self.address + 0x540) as *mut f32) = health; }
    }

    pub fn clear_weapons(&self) {
        let func: fn(this: *const c_void) = unsafe {transmute(0x5E6320 as *const())};
        func(self.address as *const c_void);
    }

    pub fn get_target_ped(&self) -> Option<Ped> {
        let target_address = unsafe { *((self.address + 0x79C) as *const usize) };
        match target_address {
            0 => None,
            _ => Some(Ped::new(target_address))
        }
    }

    pub fn get_target_car(&self) -> Option<Vehicle> {
        let target_address = unsafe { *((self.address + 0x568) as *const usize) };
        match target_address {
            0 => None,
            _ => Some(Vehicle::new(target_address))
        }
    }
}

pub fn get_current_ped() -> Ped {
    Ped::new(unsafe { *(0xB6F5F0 as *const usize) })
}

pub fn create_civilian_ped(type_ped: PedType, model_index: u32) -> usize {
    // let mut buf = unsafe { c_ffi::malloc(0x79C)};
    let layout = std::alloc::Layout::new::<[u8; 0x79C]>();
    let buf = unsafe { std::alloc::alloc(layout) };
    let func: extern "thiscall" fn(this: *const c_void, type_ped: u32, model_index: u32) -> usize = unsafe { transmute(0x5DDB70 as *const()) };
    func(
        buf as *const c_void,
        type_ped as u32,
        model_index,
    );
    buf as usize
}

pub fn create_ped(ped_type: PedType) -> usize {
    let mut buf = [0u8; 0x79C];
    let func: extern "thiscall" fn(this: *const c_void, type_ped: u32) -> usize = unsafe { transmute(0x5E8030 as *const()) };
    func(
        buf.as_mut_ptr() as *const c_void,
        ped_type as u32,
    )
}