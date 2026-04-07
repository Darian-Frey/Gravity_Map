use eframe::egui;
use crate::canvas::CanvasState;
use crate::nodes::{IdeaNode, BlueprintNode};
use crate::modes;

#[derive(PartialEq, Clone)]
pub(crate) enum Mode {
    Gravity,
    NeuralNetwork,
    Blueprint,
}

pub(crate) struct GravityApp {
    pub mode: Mode,

    // Mode 1: Gravity
    pub nodes: Vec<IdeaNode>,
    pub gravity_canvas: CanvasState,
    pub collapse_action: i8,

    // Mode 2: Neural Network
    pub neural_canvas: CanvasState,

    // Mode 3: Blueprint
    pub bp_nodes: Vec<BlueprintNode>,
    pub blueprint_canvas: CanvasState,
    pub bp_next_id: usize,
    pub bp_tethers: Vec<(usize, usize)>,
    pub bp_link_from: Option<usize>,
    pub bp_link_mode: bool,
    pub bp_delete_tether: Option<usize>,
    pub bp_delete_node: Option<usize>,
    pub bp_project_name: String,
}

impl Default for GravityApp {
    fn default() -> Self {
        Self {
            mode: Mode::Gravity,

            nodes: Vec::new(),
            gravity_canvas: CanvasState::default(),
            collapse_action: 0,

            neural_canvas: CanvasState::default(),

            bp_nodes: Vec::new(),
            blueprint_canvas: CanvasState::default(),
            bp_next_id: 0,
            bp_tethers: Vec::new(),
            bp_link_from: None,
            bp_link_mode: false,
            bp_delete_tether: None,
            bp_delete_node: None,
            bp_project_name: "Untitled".to_owned(),
        }
    }
}

impl eframe::App for GravityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gravity Map");

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, Mode::Gravity, "Mode 1: Gravity");
                ui.selectable_value(&mut self.mode, Mode::NeuralNetwork, "Mode 2: Neural Network");
                ui.selectable_value(&mut self.mode, Mode::Blueprint, "Mode 3: Blueprint");
            });

            ui.separator();

            let panel_rect = ui.available_rect_before_wrap();

            match self.mode {
                Mode::Gravity => modes::gravity::render_gravity(self, ctx, ui, panel_rect),
                Mode::NeuralNetwork => modes::neural::render_neural(self, ctx, ui, panel_rect),
                Mode::Blueprint => modes::blueprint::render_blueprint(self, ctx, ui, panel_rect),
            }

            self.collapse_action = 0;
            ctx.request_repaint();
        });
    }
}
