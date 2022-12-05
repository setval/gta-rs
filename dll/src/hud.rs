use std::mem::transmute;

pub fn set_help_text(text: &str, quick_message: bool, permanent: bool, add_to_brief: bool)  {
    let func: fn(text: *const u8, quick_message: bool, permanent: bool, add_to_brief: bool) = unsafe { transmute(0x588BE0 as *const()) };
    func(text.as_ptr(), quick_message, permanent, add_to_brief);
}

pub fn set_text(text: &str)  {
    let func: fn(text: *const u8) = unsafe { transmute(0x588F60 as *const()) };
    func(text.as_ptr());
}