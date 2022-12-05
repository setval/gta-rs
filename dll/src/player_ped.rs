use std::ffi::c_void;
use std::mem::transmute;
use crate::ped::Ped;

#[derive(Debug)]
pub struct PlayerPed {
    pub ped: Ped,
}

impl PlayerPed {
    pub fn new(address: usize) -> Self {
        Self {
            ped: Ped::new(address),
        }
    }

    pub fn set_wanted_level(&self, level: i32) {
        let func: extern "thiscall" fn(*const c_void, i32) = unsafe{ transmute(0x609F10 as *const()) };
        func(self.ped.address as *const c_void, level);
    }
}

pub fn find_player_ped(id: i32) -> Option<PlayerPed> {
    let func: fn(i32) -> usize = unsafe { transmute(0x56E210 as *const c_void) };
    let addr = func(id);
    match addr {
        0 => None,
        _ => Some(PlayerPed::new(addr)),
    }
}

pub fn get_current_player_ped() -> PlayerPed {
    match find_player_ped(-1) {
        Some(ped) => ped,
        None => panic!("Could not find current player ped"),
    }
}