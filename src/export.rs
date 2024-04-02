use std::{path::PathBuf, time::Duration};

use eframe::{
    egui::{Context, Direction, Layout, Style, Ui},
    emath::Align,
};
use egui_file::FileDialog;
use egui_notify::Toasts;

use crate::section_title;
use crate::theme::Theme;

#[derive(Default)]
pub struct ExportMenu {
    path: Option<PathBuf>,
    toasts: Toasts,
    file_dialog: Option<FileDialog>,
}

impl ExportMenu {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context, style: &Style) {
        ui.add(section_title("Export", None));

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

                    let mut dialog = FileDialog::save_file(Some(PathBuf::from("../../assets/styles"))).filename_filter(filter);
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
                if ui.button("Export").clicked() {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(path) = &self.path {
                        let file = std::fs::File::create(path).expect("opened");
                        let file = std::io::BufWriter::new(file);

                        let theme = Theme {
                            style: style.clone()
                        };
                        match ron::ser::to_writer_pretty(
                            file,
                            &theme,
                            ron::ser::PrettyConfig::default().struct_names(true),
                        ) {
                            Ok(_) => self.toasts.success("Exported!"),
                            Err(err) => self.toasts.error(format!("Export Error: {err}")),
                        }
                    } else {
                        self.toasts.error("You must select a save file to export!")
                    }
                        .set_duration(Some(Duration::from_secs(5)));
                }
            },
        );

        self.toasts.show(ctx);
    }
}
