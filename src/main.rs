use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Gravity Map",
        options,
        Box::new(|_cc| Ok(Box::new(GravityApp::default()))),
    )
}

struct GravityApp {
    mode: String,
}

impl Default for GravityApp {
    fn default() -> Self {
        Self { mode: "Blueprint".to_owned() }
    }
}

impl eframe::App for GravityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌌 Gravity Map");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, "Blueprint".to_owned(), "Mode 1: Blueprint");
                ui.selectable_value(&mut self.mode, "Gravity".to_owned(), "Mode 2: Gravity");
            });

            ui.separator();

            if self.mode == "Blueprint" {
                ui.label("Map your ideas, PRDs, and Tech Stacks here.");
                if ui.button("Add Idea Node").clicked() {
                    // Logic for adding nodes goes here
                }
            } else {
                ui.label("Software Gravity Mode: Entry Point (The Sun) detected.");
                ui.label("Scanning for C# / Rust / Python files...");
            }
        });
    }
}
