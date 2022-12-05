#![feature(abi_thiscall)]

mod logger;
mod memory;
mod world;
mod ped;
mod player_ped;
mod game;
mod vehicle;
mod pool;
mod streaming;
mod hud;
mod physical;
mod entity;
mod placeable;
mod render;

use crate::{logger::init_logs, game::Vector3, ped::create_civilian_ped, render::HelloHud};
use hudhook::hooks::{self, ImguiRenderLoop, dx9::ImguiDx9Hooks};
use log::{debug, info};
use windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};
use std::ffi::c_void;

fn main() {
    init_logs();
    info!("begin system");

    let player = ped::get_current_ped();
    let position = player.physical.entity.placeable.get_position();
    debug!("position: {:?}", position);
    let model_index: u32 = 103;
    streaming::request_model(model_index, 0);
    streaming::load_all_requested_models(false);    
    let addr_civil_ped_point = create_civilian_ped(ped::PedType::Civmale, model_index);
    debug!("addr_civil_ped_point: {:?}", addr_civil_ped_point);
    let addr_civil_ped = ped::Ped::new(addr_civil_ped_point);
    addr_civil_ped.physical.entity.placeable.set_position(Vector3 { x: position.x + 5.0 as f32, y: position.y+2.0, z: position.z });
    world::add(addr_civil_ped.address);
    std::thread::sleep(std::time::Duration::from_secs(10));
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    #[allow(non_snake_case)] hModule: windows::Win32::Foundation::HINSTANCE,
    #[allow(non_snake_case)] dwReason: u32,
    #[allow(non_snake_case)] _lpReserved: *const c_void,
) -> bool {
    if dwReason != DLL_PROCESS_ATTACH {
        return true;
    }
    // hudhook::lifecycle::global_state::set_module(hModule);
    std::thread::spawn(move || {
        // let mut hooks: Box<dyn hooks::Hooks> = 
        //     HelloHud::new().into_hook::<ImguiDx9Hooks>();
        // hooks.hook();
        // hudhook::lifecycle::global_state::set_hooks(hooks);
        main();
        info!("end system");
        hooks.unhook();
        memory::free(hModule);
    });
    true
}