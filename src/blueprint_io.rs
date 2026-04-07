use serde::{Serialize, Deserialize};
use crate::app::GravityApp;
use crate::nodes::{BlueprintNode, NodeCategory};

#[derive(Serialize, Deserialize)]
struct SavedNode {
    id: usize,
    title: String,
    content: String,
    x: f32,
    y: f32,
    category: NodeCategory,
}

#[derive(Serialize, Deserialize)]
struct SavedTether {
    from: usize,
    to: usize,
}

#[derive(Serialize, Deserialize)]
struct BlueprintSave {
    project_name: String,
    saved_at: String,
    cam_offset_x: f32,
    cam_offset_y: f32,
    zoom: f32,
    next_id: usize,
    nodes: Vec<SavedNode>,
    tethers: Vec<SavedTether>,
}

pub(crate) fn save_blueprint(app: &GravityApp) {
    let save = BlueprintSave {
        project_name: app.bp_project_name.clone(),
        saved_at: chrono_now(),
        cam_offset_x: app.blueprint_canvas.offset.x,
        cam_offset_y: app.blueprint_canvas.offset.y,
        zoom: app.blueprint_canvas.zoom,
        next_id: app.bp_next_id,
        nodes: app.bp_nodes.iter().map(|n| SavedNode {
            id: n.id,
            title: n.title.clone(),
            content: n.content.clone(),
            x: n.pos.x,
            y: n.pos.y,
            category: n.category,
        }).collect(),
        tethers: app.bp_tethers.iter().map(|&(from, to)| SavedTether { from, to }).collect(),
    };

    let json = match serde_json::to_string_pretty(&save) {
        Ok(j) => j,
        Err(_) => return,
    };

    if let Some(path) = rfd::FileDialog::new()
        .set_file_name(&format!("{}.json", app.bp_project_name))
        .add_filter("JSON", &["json"])
        .save_file()
    {
        let _ = std::fs::write(path, json);
    }
}

pub(crate) fn load_blueprint(app: &mut GravityApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .pick_file()
    {
        let data = match std::fs::read_to_string(&path) {
            Ok(d) => d,
            Err(_) => return,
        };
        let save: BlueprintSave = match serde_json::from_str(&data) {
            Ok(s) => s,
            Err(_) => return,
        };

        app.bp_project_name = save.project_name;
        app.blueprint_canvas.offset = eframe::egui::vec2(save.cam_offset_x, save.cam_offset_y);
        app.blueprint_canvas.zoom = save.zoom;
        app.bp_next_id = save.next_id;
        app.bp_link_from = None;
        app.bp_link_mode = false;
        app.bp_delete_tether = None;
        app.bp_delete_node = None;

        app.bp_nodes = save.nodes.into_iter().map(|n| BlueprintNode {
            id: n.id,
            title: n.title,
            content: n.content,
            pos: eframe::egui::pos2(n.x, n.y),
            category: n.category,
        }).collect();

        app.bp_tethers = save.tethers.into_iter().map(|t| (t.from, t.to)).collect();
    }
}

fn chrono_now() -> String {
    // Simple timestamp without external crate
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    // Format as readable UTC-ish timestamp
    let days = secs / 86400;
    let years = 1970 + days / 365; // approximate
    let remainder_secs = secs % 86400;
    let hours = remainder_secs / 3600;
    let minutes = (remainder_secs % 3600) / 60;
    let seconds = remainder_secs % 60;
    format!("{}-xx-xx {:02}:{:02}:{:02} UTC (epoch: {})", years, hours, minutes, seconds, secs)
}
