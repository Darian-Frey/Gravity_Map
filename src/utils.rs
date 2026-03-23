use eframe::egui;

pub(crate) fn dist_to_bezier(point: egui::Pos2, p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2) -> f32 {
    let steps = 16;
    let mut min_dist = f32::MAX;
    for i in 0..steps {
        let t0 = i as f32 / steps as f32;
        let t1 = (i + 1) as f32 / steps as f32;
        let a = bezier_point(p0, p1, p2, t0);
        let b = bezier_point(p0, p1, p2, t1);
        min_dist = min_dist.min(point_to_segment_dist(point, a, b));
    }
    min_dist
}

pub(crate) fn bezier_point(p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, t: f32) -> egui::Pos2 {
    let inv = 1.0 - t;
    egui::pos2(
        inv * inv * p0.x + 2.0 * inv * t * p1.x + t * t * p2.x,
        inv * inv * p0.y + 2.0 * inv * t * p1.y + t * t * p2.y,
    )
}

fn point_to_segment_dist(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let t = (ap.x * ab.x + ap.y * ab.y) / (ab.x * ab.x + ab.y * ab.y + 0.0001);
    let t = t.clamp(0.0, 1.0);
    let closest = egui::pos2(a.x + t * ab.x, a.y + t * ab.y);
    (p - closest).length()
}

pub(crate) fn depth_colors(depth: usize) -> (egui::Color32, egui::Color32) {
    match depth {
        0 => (
            egui::Color32::from_rgba_unmultiplied(80, 150, 255, 25),
            egui::Color32::from_rgba_unmultiplied(150, 220, 255, 180),
        ),
        1 => (
            egui::Color32::from_rgba_unmultiplied(80, 255, 120, 25),
            egui::Color32::from_rgba_unmultiplied(150, 255, 180, 180),
        ),
        2 => (
            egui::Color32::from_rgba_unmultiplied(255, 180, 50, 25),
            egui::Color32::from_rgba_unmultiplied(255, 220, 130, 180),
        ),
        3 => (
            egui::Color32::from_rgba_unmultiplied(255, 80, 80, 25),
            egui::Color32::from_rgba_unmultiplied(255, 150, 150, 180),
        ),
        4 => (
            egui::Color32::from_rgba_unmultiplied(200, 80, 255, 25),
            egui::Color32::from_rgba_unmultiplied(220, 160, 255, 180),
        ),
        _ => (
            egui::Color32::from_rgba_unmultiplied(255, 255, 80, 25),
            egui::Color32::from_rgba_unmultiplied(255, 255, 180, 180),
        ),
    }
}
