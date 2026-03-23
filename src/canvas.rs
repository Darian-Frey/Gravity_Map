use eframe::egui;

pub(crate) fn world_to_screen(world_pos: egui::Pos2, cam_offset: egui::Vec2, zoom: f32) -> egui::Pos2 {
    egui::pos2(world_pos.x * zoom + cam_offset.x, world_pos.y * zoom + cam_offset.y)
}

pub(crate) fn handle_pan_zoom(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    panel_rect: egui::Rect,
    cam_offset: &mut egui::Vec2,
    zoom: &mut f32,
    id_salt: &str,
) {
    let panel_response = ui.interact(panel_rect, ui.id().with(id_salt), egui::Sense::drag());
    if panel_response.dragged() {
        *cam_offset += panel_response.drag_delta();
    }

    let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
    if scroll_delta != 0.0 {
        let zoom_factor = 1.0 + scroll_delta * 0.002;
        let new_zoom = (*zoom * zoom_factor).clamp(0.1, 5.0);
        if let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos()) {
            let mouse_world_before = (mouse_pos - *cam_offset) / *zoom;
            *zoom = new_zoom;
            let mouse_world_after = (mouse_pos - *cam_offset) / *zoom;
            *cam_offset += (mouse_world_after - mouse_world_before) * *zoom;
        } else {
            *zoom = new_zoom;
        }
    }
}

pub(crate) fn draw_grid(
    painter: &egui::Painter,
    panel_rect: egui::Rect,
    cam_offset: egui::Vec2,
    zoom: f32,
) {
    let grid_spacing = 100.0_f32;
    let grid_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 15);
    let label_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 40);
    let scaled_spacing = grid_spacing * zoom;

    let left = panel_rect.left();
    let right = panel_rect.right();
    let top = panel_rect.top();
    let bottom = panel_rect.bottom();

    let first_x = ((left - cam_offset.x) / scaled_spacing).floor() as i32;
    let last_x = ((right - cam_offset.x) / scaled_spacing).ceil() as i32;
    for ix in first_x..=last_x {
        let x = ix as f32 * scaled_spacing + cam_offset.x;
        painter.line_segment(
            [egui::pos2(x, top), egui::pos2(x, bottom)],
            egui::Stroke::new(1.0, grid_color),
        );
        if ix % 5 == 0 {
            painter.text(
                egui::pos2(x + 2.0, top + 2.0),
                egui::Align2::LEFT_TOP,
                format!("{}", ix as f32 * grid_spacing),
                egui::FontId::proportional(10.0),
                label_color,
            );
        }
    }

    let first_y = ((top - cam_offset.y) / scaled_spacing).floor() as i32;
    let last_y = ((bottom - cam_offset.y) / scaled_spacing).ceil() as i32;
    for iy in first_y..=last_y {
        let y = iy as f32 * scaled_spacing + cam_offset.y;
        painter.line_segment(
            [egui::pos2(left, y), egui::pos2(right, y)],
            egui::Stroke::new(1.0, grid_color),
        );
        if iy % 5 == 0 {
            painter.text(
                egui::pos2(left + 2.0, y + 2.0),
                egui::Align2::LEFT_TOP,
                format!("{}", iy as f32 * grid_spacing),
                egui::FontId::proportional(10.0),
                label_color,
            );
        }
    }
}
