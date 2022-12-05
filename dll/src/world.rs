use std::{ffi::c_void, mem::transmute};

pub fn set_weather(id: u16) {
    unsafe { *(0xC81320 as *mut u16) = id; }
}

pub fn get_weather() -> u16 {
    unsafe { *(0xC81320 as *const u16) }.into()
}

pub fn add(entity_addr: usize) {
    let func: extern "cdecl" fn(*const c_void) = unsafe { transmute(0x563220 as *const()) };
    func(entity_addr as *const c_void);
}

pub fn remove(entity_addr: usize) {
    let func: extern "cdecl" fn(*const c_void) = unsafe { transmute(0x563280 as *const()) };
    func(entity_addr as *const c_void);
}