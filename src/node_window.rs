use eframe::egui;

/// Result of rendering a node-shell window. The caller uses these fields to
/// update the node's world-space position and to react to right-click delete.
pub(crate) struct NodeWindowResponse {
    /// Drag delta already converted to world space (divided by zoom).
    /// Zero if the window is not movable or did not move this frame.
    pub world_drag_delta: egui::Vec2,
    /// True if the user right-clicked while hovering inside this window's rect.
    pub right_clicked_inside: bool,
}

/// Shared shell for rendering a node as an `egui::Window`.
///
/// Each mode renders its nodes very differently inside the window body, but
/// they all need the same outer shell: place the window at a screen position,
/// let the user drag it (and translate that drag back into world coordinates),
/// and detect right-clicks for the delete-confirmation flow. Centralising that
/// here means Mode 1, Mode 3, and (eventually) Mode 2 can pick up new node-window
/// features (multi-select, drag-to-link, resize) in one place instead of three.
///
/// The body of the window is supplied by the caller via the `add_contents`
/// closure, so each mode is still free to render whatever it wants inside.
pub(crate) fn render_node_window<R>(
    ctx: &egui::Context,
    id: egui::Id,
    title: String,
    screen_pos: egui::Pos2,
    zoom: f32,
    movable: bool,
    frame: Option<egui::Frame>,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> NodeWindowResponse {
    let mut window = egui::Window::new(title)
        .current_pos(screen_pos)
        .movable(movable)
        .constrain(false)
        .id(id);
    if let Some(f) = frame {
        window = window.frame(f);
    }
    let response = window.show(ctx, add_contents);

    let mut out = NodeWindowResponse {
        world_drag_delta: egui::Vec2::ZERO,
        right_clicked_inside: false,
    };

    if let Some(inner) = response {
        let rect = inner.response.rect;

        if movable {
            let actual_pos = rect.min;
            let delta = actual_pos - screen_pos;
            if delta.length() > 0.5 && zoom.abs() > f32::EPSILON {
                out.world_drag_delta = delta / zoom;
            }
        }

        let (right_clicked, hover_pos) = ctx.input(|i| {
            (
                i.pointer.button_clicked(egui::PointerButton::Secondary),
                i.pointer.hover_pos(),
            )
        });
        if right_clicked {
            if let Some(mp) = hover_pos {
                if rect.contains(mp) {
                    out.right_clicked_inside = true;
                }
            }
        }
    }

    out
}
