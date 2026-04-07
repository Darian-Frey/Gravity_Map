use eframe::egui;

/// Pan/zoom state for an infinite 2D canvas. Each mode owns one of these so the
/// modes never share view state by accident — and so adding a new mode is just
/// "give it its own CanvasState" rather than "thread two more fields through
/// every call site".
#[derive(Clone, Debug)]
pub(crate) struct CanvasState {
    pub offset: egui::Vec2,
    pub zoom: f32,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            offset: egui::Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl CanvasState {
    pub fn world_to_screen(&self, world_pos: egui::Pos2) -> egui::Pos2 {
        egui::pos2(
            world_pos.x * self.zoom + self.offset.x,
            world_pos.y * self.zoom + self.offset.y,
        )
    }

    pub fn handle_pan_zoom(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        panel_rect: egui::Rect,
        id_salt: &str,
    ) {
        // Pan via either:
        //   - middle-mouse drag, or
        //   - Ctrl + left-drag (trackpad-friendly fallback for users without
        //     a middle button).
        //
        // Plain left-drag on the canvas is reserved for window / node
        // interaction so dragging an empty area of the canvas no longer fights
        // with dragging a node window. (Closes BUG-001.)
        //
        // We use the raw pointer delta from `ctx.input` rather than
        // `panel_response.drag_delta()` because the latter is wired to the
        // primary button only and returns zero for middle-button drags.
        //
        // NOTE: we originally tried Space + left-drag (the Figma/Miro pattern)
        // but on at least some Linux Mint Cinnamon setups the compositor stops
        // delivering pointer-move events the instant space is held while a
        // mouse button is down — `hover_pos` literally freezes until space is
        // released. The behaviour is outside the app's control, so we use Ctrl
        // as the modifier instead.
        let (ctrl_held, pointer_delta) = ctx.input(|i| {
            (i.modifiers.ctrl, i.pointer.delta())
        });
        if ctrl_held {
            ctx.set_cursor_icon(egui::CursorIcon::Grab);
        }

        let panel_response = ui.interact(panel_rect, ui.id().with(id_salt), egui::Sense::drag());
        let middle_drag = panel_response.dragged_by(egui::PointerButton::Middle);
        let ctrl_left_drag = ctrl_held && panel_response.dragged_by(egui::PointerButton::Primary);
        if middle_drag || ctrl_left_drag {
            self.offset += pointer_delta;
            ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
        }

        let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
        if scroll_delta != 0.0 {
            let zoom_factor = 1.0 + scroll_delta * 0.002;
            let new_zoom = (self.zoom * zoom_factor).clamp(0.1, 5.0);
            if let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos()) {
                let mouse_world_before = (mouse_pos - self.offset) / self.zoom;
                self.zoom = new_zoom;
                let mouse_world_after = (mouse_pos - self.offset) / self.zoom;
                self.offset += (mouse_world_after - mouse_world_before) * self.zoom;
            } else {
                self.zoom = new_zoom;
            }
        }
    }

    /// Adjust `offset` and `zoom` so that every point in `points` fits inside
    /// `panel_rect`, leaving `margin` pixels of breathing room on each side.
    ///
    /// - If `points` is empty, this is a no-op.
    /// - If all points are clustered at (or very near) one location, the
    ///   canvas is centered on them at the default zoom rather than zoomed in
    ///   to maximum.
    /// - The resulting zoom is clamped to the same [0.1, 5.0] range as
    ///   scroll-wheel zoom, so a single tiny cluster won't blow past the
    ///   sensible limits.
    pub fn zoom_to_fit(&mut self, panel_rect: egui::Rect, points: &[egui::Pos2], margin: f32) {
        if points.is_empty() {
            return;
        }

        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        for p in points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        let world_center = egui::pos2((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);
        let screen_center = panel_rect.center();

        let bbox_w_raw = max_x - min_x;
        let bbox_h_raw = max_y - min_y;

        // Cluster of one (or all points effectively coincident): just center
        // them at default zoom — no useful "fit" exists.
        if bbox_w_raw < 1.0 && bbox_h_raw < 1.0 {
            self.zoom = 1.0;
            self.offset = screen_center.to_vec2() - world_center.to_vec2();
            return;
        }

        let bbox_w = bbox_w_raw.max(1.0);
        let bbox_h = bbox_h_raw.max(1.0);

        let avail_w = (panel_rect.width() - 2.0 * margin).max(1.0);
        let avail_h = (panel_rect.height() - 2.0 * margin).max(1.0);

        let new_zoom = (avail_w / bbox_w).min(avail_h / bbox_h).clamp(0.1, 5.0);

        self.zoom = new_zoom;
        self.offset = screen_center.to_vec2() - world_center.to_vec2() * new_zoom;
    }

    pub fn draw_grid(&self, painter: &egui::Painter, panel_rect: egui::Rect) {
        let grid_spacing = 100.0_f32;
        let grid_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 15);
        let label_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 40);
        let scaled_spacing = grid_spacing * self.zoom;

        let left = panel_rect.left();
        let right = panel_rect.right();
        let top = panel_rect.top();
        let bottom = panel_rect.bottom();

        let first_x = ((left - self.offset.x) / scaled_spacing).floor() as i32;
        let last_x = ((right - self.offset.x) / scaled_spacing).ceil() as i32;
        for ix in first_x..=last_x {
            let x = ix as f32 * scaled_spacing + self.offset.x;
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

        let first_y = ((top - self.offset.y) / scaled_spacing).floor() as i32;
        let last_y = ((bottom - self.offset.y) / scaled_spacing).ceil() as i32;
        for iy in first_y..=last_y {
            let y = iy as f32 * scaled_spacing + self.offset.y;
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
}
