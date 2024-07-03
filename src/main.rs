mod app;
mod calcander;
mod fourarea;
mod inbox;

use eframe::App;
use crate::app::NoteDayApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions { ..Default::default() };
    eframe::run_native("日历记事本", options,
                       Box::new(|cc| Box::new(NoteDayApp::new(cc))),
    )
}
