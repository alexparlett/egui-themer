use std::{path::PathBuf, time::Duration};

use eframe::{
    egui::{Context, Direction, Layout, Style, Ui},
    emath::Align,
};
use egui_file::FileDialog;
use egui_notify::Toasts;
use wasm_bindgen::UnwrapThrowExt;

use crate::section_title;
use crate::theme::Theme;

#[derive(Default)]
pub struct ImportMenu {
    path: Option<PathBuf>,
    toasts: Toasts,
    file_dialog: Option<FileDialog>,
}

impl ImportMenu {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context, style: &mut Style) {
        ui.add(section_title("Import", None));

        ui.columns(2, |cols| {
            cols[0].label("Path:");
            cols[1].with_layout(Layout::right_to_left(Align::Min), |ui| {
                if ui
                    .button(
                        self.path
                            .as_ref()
                            .map(|f| {
                                f.file_name()
                                    .map(|n| n.to_str().unwrap_or("invalid!"))
                                    .unwrap_or("Select a File!")
                            })
                            .unwrap_or("Select"),
                    )
                    .clicked()
                {
                    let filter = Box::new({
                        move |filename: &str| -> bool { filename.ends_with(".theme.ron") }
                    });

                    let mut dialog = FileDialog::open_file(Some(PathBuf::from("../../assets/styles")))
                        .filename_filter(filter);
                    dialog.open();
                    self.file_dialog = Some(dialog);
                }

                if let Some(dialog) = &mut self.file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            self.path = Some(file.to_path_buf());
                        }
                    }
                }
            });
        });

        ui.allocate_ui_with_layout(
            [ui.available_width(), 0.0].into(),
            Layout::centered_and_justified(Direction::TopDown),
            |ui| {
                if ui.button("Import").clicked() {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(path) = &self.path {
                        let file = std::fs::File::open(path).expect("opened");
                        let file = std::io::BufReader::new(file);

                        let imported: Theme = ron::de::from_reader(
                            file,
                        ).unwrap_throw();

                        style.override_text_style = imported.style.override_text_style.clone();
                        style.override_font_id = imported.style.override_font_id.clone();
                        style.text_styles = imported.style.text_styles.clone();
                        style.drag_value_text_style = imported.style.drag_value_text_style.clone();
                        style.wrap = imported.style.wrap.clone();
                        style.spacing = imported.style.spacing.clone();
                        style.interaction = imported.style.interaction.clone();
                        style.visuals = imported.style.visuals.clone();
                        style.animation_time = imported.style.animation_time.clone();
                        style.debug = imported.style.debug.clone();
                        style.explanation_tooltips = imported.style.explanation_tooltips.clone();
                        style.url_in_tooltip = imported.style.url_in_tooltip.clone();
                        style.always_scroll_the_only_direction = imported.style.always_scroll_the_only_direction.clone();

                        self.toasts.success("Imported!")
                    } else {
                        self.toasts.error("You must select a file to import!")
                    }
                        .set_duration(Some(Duration::from_secs(5)));
                }
            },
        );

        self.toasts.show(ctx);
    }
}
