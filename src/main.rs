mod app;
mod calcander;
mod da;
mod fourarea;
mod inbox;

use app::NoteDayApp;
use da::DayDialog;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    let mut app_dialog = DayDialog::new();
    eframe::run_native(
        "日历记事本",
        options,
        Box::new(|cc| Box::new(NoteDayApp::new(cc))),
    )
}
