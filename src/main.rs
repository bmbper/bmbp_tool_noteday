mod app;
mod data;
mod part;
mod util;
mod view;
mod dialog;
mod orm;

use egui::{vec2, ViewportBuilder};
use app::NoteDayApp;

fn main() -> eframe::Result<()> {
    // 设置初始化窗口大小
    let viewport = ViewportBuilder::default().with_inner_size(vec2(1920.0,1080.0));
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "NoteDay",
        options,
        Box::new(|cc|Ok( Box::new(NoteDayApp::new(cc)))),
    )
}
