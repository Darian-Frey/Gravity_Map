use eframe::egui;
use crate::app::GravityApp;

pub(crate) fn render_neural(app: &mut GravityApp, ctx: &egui::Context, ui: &mut egui::Ui, panel_rect: egui::Rect) {
    // Wire up the shared canvas now so when Mode 2 nodes/tethers land, the
    // pan/zoom/grid plumbing is already in place — no second refactor required.
    app.neural_canvas.handle_pan_zoom(ctx, ui, panel_rect, "neural_canvas_pan");
    app.neural_canvas.draw_grid(ui.painter(), panel_rect);

    ui.horizontal(|ui| {
        ui.label("🧠 Neural Network — Functional View");
    });
    ui.separator();
    // TODO: Mode 2 content (dependency parser, force-directed layout, directional tethers)
}
