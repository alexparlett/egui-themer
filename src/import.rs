use std::{fs, path::PathBuf, time::Duration};

use eframe::{
    egui::{Context, Direction, Layout, Style, Ui},
    emath::Align,
};
use egui_file::FileDialog;
use egui_notify::Toasts;
use handlebars::{handlebars_helper, Handlebars, JsonValue};
use rust_format::{Formatter, RustFmt};
use wasm_bindgen::UnwrapThrowExt;

use crate::section_title;

const TEMPLATE: &str = include_str!("template.rs");

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
                    let mut dialog = FileDialog::save_file(None);
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
                        match path.extension().unwrap_throw().to_str().unwrap_throw() {
                            "ron" => {
                                let file = std::fs::File::open(path).expect("opened");
                                let file = std::io::BufReader::new(file);

                                let imported: Style = ron::de::from_reader(
                                    file,
                                ).unwrap_throw();

                                style.override_text_style = imported.override_text_style.clone();
                                style.override_font_id = imported.override_font_id.clone();
                                style.text_styles = imported.text_styles.clone();
                                style.drag_value_text_style = imported.drag_value_text_style.clone();
                                style.wrap = imported.wrap.clone();
                                style.spacing = imported.spacing.clone();
                                style.interaction = imported.interaction.clone();
                                style.visuals = imported.visuals.clone();
                                style.animation_time = imported.animation_time.clone();
                                style.debug = imported.debug.clone();
                                style.explanation_tooltips = imported.explanation_tooltips.clone();
                                style.url_in_tooltip = imported.url_in_tooltip.clone();
                                style.always_scroll_the_only_direction = imported.always_scroll_the_only_direction.clone();

                                self.toasts.success("Imported!")
                            }
                            _ => self.toasts.error("Un supported file type!")
                        }
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

fn generate_source(style: &Style, eframe: bool) -> Result<String, String> {
    let mut reg = Handlebars::new();

    reg.register_template_string("template", TEMPLATE)
        .map_err(|err| err.to_string())?;

    reg.register_helper("vec2", Box::new(vec2));
    reg.register_helper("stroke", Box::new(stroke));
    reg.register_helper("rounding", Box::new(rounding));
    reg.register_helper("color32", Box::new(color32));
    reg.register_helper("widgetvisuals", Box::new(widgetvisuals));

    let raw = reg
        .render(
            "template",
            &serde_json::json!({
                "eframe": eframe,
                "style": style,
            }),
        )
        .map_err(|err| err.to_string())?;

    RustFmt::default()
        .format_str(raw)
        .map_err(|err| err.to_string())
}

handlebars_helper!(vec2: |value: JsonValue| format!("Vec2 {{ x: {}, y: {}}}", &value["x"], &value["y"]));
handlebars_helper!(stroke: |value: JsonValue| gen_stroke(&value));
handlebars_helper!(rounding: |value: JsonValue| gen_rounding(&value));
handlebars_helper!(color32: |color: JsonValue| gen_color32(&color));
handlebars_helper!(widgetvisuals: |value: JsonValue| {
    format!("WidgetVisuals {{
        bg_fill: {},
        weak_bg_fill: {},
        bg_stroke: {},
        rounding: {},
        fg_stroke: {},
        expansion: {},
    }}",
        gen_color32(&value["bg_fill"]),
        gen_color32(&value["weak_bg_fill"]),
        gen_stroke(&value["bg_stroke"]),
        gen_rounding(&value["rounding"]),
        gen_stroke(&value["fg_stroke"]),
        value["expansion"]
    )
});

fn gen_color32(value: &JsonValue) -> String {
    format!(
        "Color32::from_rgba_premultiplied({}, {}, {}, {})",
        value[0], value[1], value[2], value[3]
    )
}

fn gen_stroke(value: &JsonValue) -> String {
    format!(
        "Stroke {{
            width: {},
            color: {},
        }}",
        value["width"],
        gen_color32(&value["color"]),
    )
}

fn gen_rounding(value: &JsonValue) -> String {
    format!(
        "Rounding {{ nw: {}, ne: {}, sw: {}, se: {} }}",
        value["nw"], value["ne"], value["sw"], value["se"]
    )
}
