use eframe::egui;
use crate::app::GravityApp;
use crate::nodes::{BlueprintNode, NodeCategory};
use crate::canvas;
use crate::utils::dist_to_bezier;

pub(crate) fn render_blueprint(app: &mut GravityApp, ctx: &egui::Context, ui: &mut egui::Ui, panel_rect: egui::Rect) {
    canvas::handle_pan_zoom(ctx, ui, panel_rect, &mut app.bp_cam_offset, &mut app.bp_zoom, "bp_canvas_pan");

    let bp_zoom = app.bp_zoom;
    let bp_cam = app.bp_cam_offset;
    canvas::draw_grid(ui.painter(), panel_rect, bp_cam, bp_zoom);

    let bp_to_screen = |world_pos: egui::Pos2| canvas::world_to_screen(world_pos, bp_cam, bp_zoom);

    // --- TOOLBAR ---
    ui.horizontal(|ui| {
        if ui.button("💡 Add Idea").clicked() {
            let id = app.bp_next_id;
            app.bp_next_id += 1;
            let spawn = egui::pos2(640.0, 360.0);
            app.bp_nodes.push(BlueprintNode {
                id,
                title: format!("Idea {}", id),
                content: String::new(),
                pos: spawn,
                category: NodeCategory::Idea,
            });
        }
        if ui.button("🗑 Clear").clicked() {
            app.bp_nodes.clear();
            app.bp_tethers.clear();
            app.bp_next_id = 0;
            app.bp_link_from = None;
            app.bp_link_mode = false;
        }

        ui.add_space(8.0);
        let link_label = if app.bp_link_mode { "🔗 Linking... (click to cancel)" } else { "🔗 Link Nodes" };
        if ui.button(link_label).clicked() {
            app.bp_link_mode = !app.bp_link_mode;
            app.bp_link_from = None;
        }
    });
    ui.separator();

    // --- TETHER LINES ---
    {
        let painter = ui.painter();
        let screen_center = egui::pos2(panel_rect.center().x, panel_rect.center().y);
        let positions: Vec<(usize, egui::Pos2)> = app.bp_nodes.iter().map(|n| (n.id, n.pos)).collect();
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
        let right_clicked = ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Secondary));

        for (idx, &(from_id, to_id)) in app.bp_tethers.iter().enumerate() {
            let from_pos = positions.iter().find(|(id, _)| *id == from_id).map(|(_, p)| *p);
            let to_pos = positions.iter().find(|(id, _)| *id == to_id).map(|(_, p)| *p);
            if let (Some(fp), Some(tp)) = (from_pos, to_pos) {
                let ps = bp_to_screen(fp);
                let cs = bp_to_screen(tp);
                let mid = ps + (cs - ps) * 0.5;
                let cp = mid + (screen_center - mid) * 0.2;

                let near = mouse_pos
                    .map(|mp| dist_to_bezier(mp, ps, cp, cs) < 8.0)
                    .unwrap_or(false);

                let glow = if near {
                    egui::Color32::from_rgba_unmultiplied(255, 80, 80, 40)
                } else {
                    egui::Color32::from_rgba_unmultiplied(150, 100, 255, 25)
                };
                let core = if near {
                    egui::Color32::from_rgba_unmultiplied(255, 120, 120, 200)
                } else {
                    egui::Color32::from_rgba_unmultiplied(200, 170, 255, 180)
                };
                let (gw, cw) = if near { (8.0, 2.5) } else { (4.0, 1.2) };

                painter.add(egui::epaint::QuadraticBezierShape {
                    points: [ps, cp, cs], closed: false,
                    fill: egui::Color32::TRANSPARENT,
                    stroke: egui::Stroke::new(gw, glow).into(),
                });
                painter.add(egui::epaint::QuadraticBezierShape {
                    points: [ps, cp, cs], closed: false,
                    fill: egui::Color32::TRANSPARENT,
                    stroke: egui::Stroke::new(cw, core).into(),
                });

                if near && right_clicked {
                    app.bp_delete_tether = Some(idx);
                }
            }
        }

        // Draw preview line while linking
        if app.bp_link_mode {
            if let Some(from_id) = app.bp_link_from {
                if let Some((_, from_pos)) = positions.iter().find(|(id, _)| *id == from_id) {
                    let ps = bp_to_screen(*from_pos);
                    if let Some(mouse) = ctx.input(|i| i.pointer.hover_pos()) {
                        painter.line_segment(
                            [ps, mouse],
                            egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(200, 170, 255, 120)),
                        );
                    }
                }
            }
        }
    }

    // --- DELETE TETHER CONFIRMATION ---
    if let Some(idx) = app.bp_delete_tether {
        let mut open = true;
        egui::Window::new("Delete Tether?")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label("Are you sure you want to delete this tether?");
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Yes, delete").clicked() {
                        if idx < app.bp_tethers.len() {
                            app.bp_tethers.remove(idx);
                        }
                        app.bp_delete_tether = None;
                    }
                    if ui.button("Cancel").clicked() {
                        app.bp_delete_tether = None;
                    }
                });
            });
        if !open {
            app.bp_delete_tether = None;
        }
    }

    // --- RENDER WINDOWS ---
    let mut clicked_node_id: Option<usize> = None;
    let mut clicked_delete_node: Option<usize> = None;
    for node in &mut app.bp_nodes {
        let screen_pos = bp_to_screen(node.pos);
        let win_id = egui::Id::new(node.id + 9000);

        let node_id = node.id;
        let is_link_mode = app.bp_link_mode;
        let cat_color = node.category.color();
        let cat_icon = node.category.icon();
        let frame = egui::Frame::window(&ctx.style())
            .stroke(egui::Stroke::new(2.0, cat_color));
        let response = egui::Window::new(format!("{} {}", cat_icon, node.title))
            .current_pos(screen_pos)
            .movable(!is_link_mode)
            .constrain(false)
            .frame(frame)
            .id(win_id)
            .show(ctx, |ui| {
                ui.text_edit_singleline(&mut node.title);
                ui.horizontal(|ui| {
                    ui.label("Category:");
                    egui::ComboBox::from_id_salt(node_id)
                        .selected_text(node.category.label())
                        .show_ui(ui, |ui| {
                            for &cat in NodeCategory::all() {
                                ui.selectable_value(&mut node.category, cat, cat.label());
                            }
                        });
                });
                ui.add(egui::TextEdit::multiline(&mut node.content).desired_rows(3).desired_width(300.0));
                if is_link_mode {
                    ui.separator();
                    if ui.button("⬤ Connect").clicked() {
                        clicked_node_id = Some(node_id);
                    }
                }
            });

        // Right-click detection on the entire window
        if let Some(inner) = &response {
            let win_rect = inner.response.rect;
            let right_clicked = ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Secondary));
            let hover_pos = ctx.input(|i| i.pointer.hover_pos());
            if right_clicked {
                if let Some(mp) = hover_pos {
                    if win_rect.contains(mp) {
                        clicked_delete_node = Some(node_id);
                    }
                }
            }
        }

        if !is_link_mode {
            if let Some(inner) = response {
                let actual_pos = inner.response.rect.min;
                let delta = actual_pos - screen_pos;
                if delta.length() > 0.5 {
                    node.pos += delta / bp_zoom;
                }
            }
        }
    }

    if let Some(nid) = clicked_delete_node {
        app.bp_delete_node = Some(nid);
    }

    // Handle link mode clicks
    if let Some(target_id) = clicked_node_id {
        if let Some(from_id) = app.bp_link_from {
            if from_id != target_id {
                let exists = app.bp_tethers.iter().any(|&(a, b)|
                    (a == from_id && b == target_id) || (a == target_id && b == from_id)
                );
                if !exists {
                    app.bp_tethers.push((from_id, target_id));
                }
            }
            app.bp_link_from = None;
        } else {
            app.bp_link_from = Some(target_id);
        }
    }

    // --- DELETE NODE CONFIRMATION ---
    if let Some(nid) = app.bp_delete_node {
        let node_title = app.bp_nodes.iter()
            .find(|n| n.id == nid)
            .map(|n| n.title.clone())
            .unwrap_or_default();
        let mut open = true;
        egui::Window::new("Delete Node?")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label(format!("Delete \"{}\" and all its connections?", node_title));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Yes, delete").clicked() {
                        app.bp_tethers.retain(|&(a, b)| a != nid && b != nid);
                        app.bp_nodes.retain(|n| n.id != nid);
                        app.bp_delete_node = None;
                    }
                    if ui.button("Cancel").clicked() {
                        app.bp_delete_node = None;
                    }
                });
            });
        if !open {
            app.bp_delete_node = None;
        }
    }
}
