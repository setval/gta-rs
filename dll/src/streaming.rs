use std::mem::transmute;

pub fn request_model(model_index: u32, flags: i32) {
    let func: fn(model_index: u32, flags: i32) = unsafe { transmute(0x4087E0 as *const()) };
    func(model_index, flags);
}

pub fn load_all_requested_models(only_quick_requests: bool) {
    let func: fn(only_quick_requests: bool) = unsafe { transmute(0x40EA10 as *const()) };
    func(only_quick_requests);
}