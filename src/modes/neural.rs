use eframe::egui;
use crate::app::GravityApp;

pub(crate) fn render_neural(_app: &mut GravityApp, _ctx: &egui::Context, ui: &mut egui::Ui, _panel_rect: egui::Rect) {
    ui.horizontal(|ui| {
        ui.label("🧠 Neural Network — Functional View");
    });
    ui.separator();
    // TODO: Mode 2 content
}
