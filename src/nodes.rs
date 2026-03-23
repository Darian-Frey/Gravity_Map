use eframe::egui;

pub(crate) struct IdeaNode {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub file_path: String,
    pub pos: egui::Pos2,
    pub is_dir: bool,
    pub depth: usize,
    pub parent_id: Option<usize>,
}

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum NodeCategory {
    Idea,
    Task,
    Bug,
    Feature,
    Research,
    Done,
}

impl NodeCategory {
    pub fn all() -> &'static [NodeCategory] {
        &[Self::Idea, Self::Task, Self::Bug, Self::Feature, Self::Research, Self::Done]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Idea => "💡 Idea",
            Self::Task => "📋 Task",
            Self::Bug => "🐛 Bug",
            Self::Feature => "✨ Feature",
            Self::Research => "🔬 Research",
            Self::Done => "✅ Done",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Idea => "💡",
            Self::Task => "📋",
            Self::Bug => "🐛",
            Self::Feature => "✨",
            Self::Research => "🔬",
            Self::Done => "✅",
        }
    }

    pub fn color(&self) -> egui::Color32 {
        match self {
            Self::Idea => egui::Color32::from_rgb(180, 160, 255),
            Self::Task => egui::Color32::from_rgb(100, 180, 255),
            Self::Bug => egui::Color32::from_rgb(255, 100, 100),
            Self::Feature => egui::Color32::from_rgb(100, 255, 160),
            Self::Research => egui::Color32::from_rgb(255, 200, 80),
            Self::Done => egui::Color32::from_rgb(120, 120, 120),
        }
    }
}

pub(crate) struct BlueprintNode {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub pos: egui::Pos2,
    pub category: NodeCategory,
}
