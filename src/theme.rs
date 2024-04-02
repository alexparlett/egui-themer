use eframe::egui;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Theme {
    pub style: egui::Style
}