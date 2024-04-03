use std::path::PathBuf;

use eframe::egui;
use eframe::egui::{Align, Color32, Context, LayerId, Layout, pos2, Rect, TextureHandle, Ui};
use egui_file::FileDialog;

use crate::section_title;

#[derive(Default)]
pub struct Background {
    path: Option<PathBuf>,
    texture: Option<TextureHandle>,
    file_dialog: Option<FileDialog>,
}

impl Background {

    pub fn texture(&self) -> &Option<TextureHandle> {
        &self.texture
    }

    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.add(section_title("Background", None));
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
                    let mut dialog = FileDialog::open_file(Some(PathBuf::from("../../assets/images")));
                    dialog.open();
                    self.file_dialog = Some(dialog);
                }

                if let Some(dialog) = &mut self.file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            let image = image::io::Reader::open(file).unwrap().decode().unwrap();
                            let size = [image.width() as _, image.height() as _];
                            let image_buffer = image.to_rgba8();
                            let pixels = image_buffer.as_flat_samples();
                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                size,
                                pixels.as_slice(),
                            );
                            let handle = ctx.load_texture(
                                "background",
                                color_image,
                                Default::default(),
                            );
                            self.texture = Some(handle);
                            self.path = Some(file.to_path_buf());
                        }
                    }
                }
            });
        });
    }

    pub fn show(&self, ctx: &Context) {
        match self.texture() {
            None => {}
            Some(texture) => {
                ctx.layer_painter(LayerId::background())
                    .image(texture.id(), ctx.screen_rect(), Rect::from_min_max(pos2(0., 0.), pos2(1., 1.)), Color32::WHITE);
            }
        }
    }
}