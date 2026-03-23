use eframe::egui;

struct IdeaNode {
    id: usize,
    title: String,
    content: String,
    pos: egui::Pos2,
    is_python: bool,
}

struct GravityApp {
    mode: String,
    nodes: Vec<IdeaNode>,
}

impl Default for GravityApp {
    fn default() -> Self {
        Self {
            mode: "Blueprint".to_owned(),
            nodes: Vec::new(),
        }
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
                ui.horizontal(|ui| {
                    ui.label("Map your ideas, PRDs, and Tech Stacks here.");
                    
                    if ui.button("➕ Add Idea Node").clicked() {
                        let id = self.nodes.len();
                        self.nodes.push(IdeaNode {
                            id,
                            title: format!("Idea {}", id),
                            content: "Enter your plan here...".to_owned(),
                            pos: egui::pos2(100.0 + (id as f32 * 10.0), 100.0),
                            is_python: false,
                        });
                    }

                    if ui.button("🐍 Add Python Module").clicked() {
                        let id = self.nodes.len();
                        self.nodes.push(IdeaNode {
                            id,
                            title: format!("module_{}.py", id),
                            content: "def main():\n    print('Hello Gravity')".to_owned(),
                            pos: egui::pos2(150.0, 150.0),
                            is_python: true,
                        });
                    }
                });

                // Render the draggable Nodes
                for node in &mut self.nodes {
                    let title = if node.is_python {
                        format!("🐍 {}", node.title)
                    } else {
                        format!("💡 {}", node.title)
                    };

                    egui::Window::new(title)
                        .default_pos(node.pos)
                        .show(ctx, |ui| {
                            ui.text_edit_singleline(&mut node.title);
                            ui.add(egui::TextEdit::multiline(&mut node.content).desired_rows(4));
                        });
                }

            } else {
                ui.label("Software Gravity Mode: Entry Point (The Sun) detected.");
                ui.label("Scanning for Python files...");
                ui.label("(Tree-sitter Python parser active)");
            }
        });
    }
}

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
