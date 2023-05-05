// Generated by egui-themer (https://github.com/grantshandy/egui-themer).

use egui::{
    style::{Margin, Spacing, Interaction, Selection, Widgets, WidgetVisuals},
    epaint::Shadow,
    Vec2,
    Visuals,
    Color32,
    Rounding,
    Stroke,
    Style,
};

pub fn style() -> Style {
    let mut style = Style::default();

    // override the text styles here:
    // style.override_text_style = Option<TextStyle>;

    // override the font id here:
    // style.override_font_id = Option<FontId>;

    // set your text styles here:
    // style.text_styles = BTreeMap<TextStyle, FontId>;

    // set your drag value text style:
    // style.drag_value_text_style = TextStyle

    {{#if wrap}}
    style.wrap = Some({{wrap}});
    {{/if}}

    style.spacing = Spacing {
        item_spacing: {{vec2 spacing.item_spacing}},
        window_margin: Margin {
            left: {{spacing.window_margin.left}},
            right: {{spacing.window_margin.right}},
            top: {{spacing.window_margin.top}},
            bottom: {{spacing.window_margin.bottom}},
        },
        button_padding: {{vec2 spacing.button_padding}},
        menu_margin: Margin {
            left: {{spacing.menu_margin.left}},
            right: {{spacing.menu_margin.right}},
            top: {{spacing.menu_margin.top}},
            bottom: {{spacing.menu_margin.bottom}},
        },
        indent: {{spacing.indent}},
        interact_size: {{vec2 spacing.interact_size}},
        slider_width: {{spacing.slider_width}},
        combo_width: {{spacing.combo_width}},
        text_edit_width: {{spacing.text_edit_width}},
        icon_width: {{spacing.icon_width}},
        icon_width_inner: {{spacing.icon_width_inner}},
        icon_spacing: {{spacing.icon_spacing}},
        tooltip_width: {{spacing.tooltip_width}},
        indent_ends_with_horizontal_line: {{spacing.indent_ends_with_horizontal_line}},
        combo_height: {{spacing.combo_height}},
        scroll_bar_width: {{spacing.scroll_bar_width}},
        scroll_handle_min_length: {{spacing.scroll_handle_min_length}},
        scroll_bar_inner_margin: {{spacing.scroll_bar_inner_margin}},
        scroll_bar_outer_margin: {{spacing.scroll_bar_outer_margin}},
    };

    style.interaction = Interaction {
        resize_grab_radius_side: {{interaction.resize_grab_radius_side}},
        resize_grab_radius_corner: {{interaction.resize_grab_radius_corner}},
        show_tooltips_only_when_still: {{interaction.show_tooltips_only_when_still}},
    };

    style.visuals = Visuals {
        dark_mode: {{visuals.dark_mode}},
        {{#if style.visuals.override_text_color}}
        override_text_color: Some({{style.visuals.override_text_color}}),
        {{else}}
        override_text_color: None,
        {{/if}}
        widgets: Widgets {
            noninteractive: {{widgetvisuals visuals.widgets.noninteractive}},
            inactive: {{widgetvisuals visuals.widgets.inactive}},
            hovered: {{widgetvisuals visuals.widgets.hovered}},
            active: {{widgetvisuals visuals.widgets.active}},
            open: {{widgetvisuals visuals.widgets.open}},
        },
        selection: Selection {
            bg_fill: {{color32 visuals.selection.bg_fill}},
            stroke: {{stroke visuals.selection.stroke}},
        },
        hyperlink_color: {{color32 visuals.hyperlink_color}},
        faint_bg_color: {{color32 visuals.faint_bg_color}},
        extreme_bg_color: {{color32 visuals.extreme_bg_color}},
        code_bg_color: {{color32 visuals.code_bg_color}},
        warn_fg_color: {{color32 visuals.warn_fg_color}},
        error_fg_color: {{color32 visuals.error_fg_color}},
        window_rounding: {{rounding visuals.window_rounding}},
        window_shadow: Shadow {
            extrusion: {{visuals.window_shadow.extrusion}},
            color: {{color32 visuals.window_shadow.color}},
        },
        window_fill: {{color32 visuals.window_fill}},
        window_stroke: {{stroke visuals.window_stroke}},
        menu_rounding: {{rounding visuals.menu_rounding}},
        panel_fill: {{color32 visuals.panel_fill}},
        popup_shadow: Shadow {
            extrusion: {{visuals.popup_shadow.extrusion}},
            color: {{color32 visuals.popup_shadow.color}},
        },
        resize_corner_size: {{visuals.resize_corner_size}},
        text_cursor_width: {{visuals.text_cursor_width}},
        text_cursor_preview: {{visuals.text_cursor_preview}},
        clip_rect_margin: {{visuals.clip_rect_margin}},
        button_frame: {{visuals.button_frame}},
        collapsing_header_frame: {{visuals.collapsing_header_frame}},
        indent_has_left_vline: {{visuals.indent_has_left_vline}},
        striped: {{visuals.striped}},
        slider_trailing_fill: {{visuals.slider_trailing_fill}},
    };

    style.animation_time = {{animation_time}};
    style.explanation_tooltips = {{explanation_tooltips}};

    style
}