mod app;
mod data;
mod dialog;
mod orm;
mod part;
mod util;
mod view;

use app::NoteDayApp;
use egui::{vec2, ViewportBuilder};

fn main() -> eframe::Result<()> {
    // 设置初始化窗口大小
    let viewport = ViewportBuilder::default().with_inner_size(vec2(1920.0, 1080.0));
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "NoteDay",
        options,
        Box::new(|cc| Ok(Box::new(NoteDayApp::new(cc)))),
    )
}
