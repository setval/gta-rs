use std::time::Instant;

pub struct HelloHud {
    start_time: Instant,
}

impl HelloHud {
    pub fn new() -> Self {
        hudhook::utils::alloc_console();
        hudhook::utils::simplelog();
        Self { start_time: Instant::now() }
    }
}

use hudhook::hooks::{ImguiRenderLoop, ImguiRenderLoopFlags};
use imgui::*;


impl ImguiRenderLoop for HelloHud {
    fn render(&mut self, ui: &mut Ui, _flags: &ImguiRenderLoopFlags) {
        Window::new("##hello")
            .size([320., 200.], Condition::Always)
            .build(ui, || {
                ui.text("Hello, world!");
                ui.text(format!("Elapsed: {:?}", self.start_time.elapsed()));
            });
    }
}