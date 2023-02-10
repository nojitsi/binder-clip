use std::process::exit;
use imgui::{Condition, InputTextCallbackHandler, InputTextMultilineCallback, StyleColor, TextCallbackData};
use textwrap::{LineEnding, Options, WordSeparator, WordSplitter, WrapAlgorithm};

use crate::support::{MAX_FONT_SIZE, MIN_FONT_SIZE, Mod};

mod support;

pub const BUTTON_FONT_SIZE: i32 = 13;

#[derive(Copy, Clone)]
struct InputTextProps {
    pub input_font_size: i32,
    pub input_width: i32,
    pub should_redraw: bool,
}

fn main() {
    let Mod { system, font_id_hash_map, mut window_props } = support::init("Binder-Clip");
    // let mut input_text = String::from("What is Lorem Ipsum?Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industrys standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.Why do we use it?It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using Content here, content here, making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for lorem ipsum will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).");
    let mut input_text = String::from("");
    let mut input_font_size: i32 = system.font_size as i32;

    let mut input_text_props = InputTextProps {
        input_font_size,
        input_width: 0,
        should_redraw: false
    };

    const WRAP_TEXT_CONTENT: fn(String, i32, i32) -> String = |content: String, display_width_px: i32, font_size_px: i32| {
        let max_chars_in_line = display_width_px / (font_size_px / 2);

        let mut pure_text = content.to_string();

        let mut last_char_is_r = false;
        pure_text.retain(|char| {
            if char == '\r' {
                last_char_is_r = true;
                return false;
            }

            if char == '\n' && last_char_is_r {
                last_char_is_r = false;
                return false;
            }

            last_char_is_r = false;

            return true;
        });

        let lines = pure_text.split("\n");
        let mut wrapped_text_lines = vec![];

        {
            for content_line in lines {
                let chars_in_line = content_line.chars().count();

                if chars_in_line <= max_chars_in_line as usize {
                    wrapped_text_lines.push(content_line.to_string());
                    continue;
                }

                let mut wrapped_line = String::from(content_line);

                wrapped_line = wrapped_line.replace(" ", "  ");
                textwrap::fill_inplace(&mut wrapped_line, max_chars_in_line as usize);
                wrapped_line = wrapped_line.replace("  ", " ");

                let wrapped_lines = wrapped_line.split("\n");

                let mut index = 0;
                let number_of_wrapped_lines = wrapped_lines.clone().count();

                let lines_with_rseperator = wrapped_lines.map(|line| {

                    let end_char;
                    if number_of_wrapped_lines == index + 1
                    { end_char = "" } else { end_char = "\r" } ;

                    index += 1;

                    return line.to_string() + end_char;
                });

                wrapped_text_lines.extend(lines_with_rseperator);
            }
        }

        return String::from(wrapped_text_lines.join("\n"));
    };

    let update_font_size = |input_text_props: &mut InputTextProps, input_text: &mut String, change: i32| {
        let size = input_text_props.input_font_size.clone() + change;
        if size < MIN_FONT_SIZE || size > MAX_FONT_SIZE {
            return;
        }
        input_text_props.input_font_size = size.clone();

        let wrapped_content = WRAP_TEXT_CONTENT(
            String::from(input_text.clone()),
            input_text_props.input_width,
            input_text_props.input_font_size,
        );

        *input_text = wrapped_content;
    };

    struct InputTextEditCallbackHandler {
        input_text_props: InputTextProps
    }
    impl InputTextCallbackHandler for InputTextEditCallbackHandler {
        fn on_always(&mut self, mut data: TextCallbackData) {
            let cursor_position = data.cursor_pos();

            if self.input_text_props.should_redraw {
                let input_content = String::from(data.str());

                let wrapped_string: &String = &WRAP_TEXT_CONTENT(
                    input_content,
                    self.input_text_props.input_width,
                    self.input_text_props.input_font_size,
                );

                data.clear();
                data.insert_chars(0, wrapped_string);
            }

            data.set_cursor_pos(cursor_position);
        }
    }

    let mut input_width = 0;
    let mut before_input_text = input_text.clone();

    system.main_loop(move |_, ui| {
        let font_id_hash_map = &font_id_hash_map;

        let new_input_width = (ui.io().display_size[0] - 21.0) as i32;

        input_text_props.input_width = new_input_width.clone();
        //wrap text on resize
        input_text_props.should_redraw = (input_width != new_input_width) || (before_input_text != input_text);
        let input_text_edit_handler = InputTextEditCallbackHandler { input_text_props };

        input_width = new_input_width;

        const HALF_BLACK_FRAME: [f32; 4] = [0.03, 0.00, 0.01, 0.5];
        const HALF_BLACK_BTN: [f32; 4] = [0.13, 0.1, 0.10, 0.5];
        const HALF_GREEN_BTN_HOVER: [f32; 4] = [0.0, 0.7, 0.0, 0.3];
        const HALF_GREEN: [f32; 4] = [0.0, 0.7, 0.0, 0.7];
        let frame_bg = ui.push_style_color(StyleColor::FrameBg, HALF_BLACK_FRAME);
        let window_bg =  ui.push_style_color(StyleColor::WindowBg, HALF_BLACK_FRAME);

        let button_bg = ui.push_style_color(StyleColor::Button, HALF_BLACK_BTN);
        let button_hover_bg = ui.push_style_color(StyleColor::ButtonHovered, HALF_GREEN_BTN_HOVER);
        let button_active_bg = ui.push_style_color(StyleColor::ButtonActive, HALF_GREEN);
        let text_selection_bg = ui.push_style_color(StyleColor::TextSelectedBg, HALF_GREEN_BTN_HOVER);

        // ui.set_scroll_y(0.1);
        // ui.set_scrol

        ui
            .window("main")
            // .draw_background(true)
            .size(ui.io().display_size, Condition::Always)
            .position([0.0, 0.0],Condition::FirstUseEver)
            .scroll_bar(false)
            // .always_auto_resize(true)
            // .always_use_window_padding(true)
            .resizable(false)
            .title_bar(false)
            .no_decoration()
            .bg_alpha(0.5)
            .build(|| {
                before_input_text = input_text.to_string();

                ui.separator();

                let font_override = ui.push_font(*font_id_hash_map.get(&input_text_props.input_font_size).unwrap());

                ui.input_text_multiline("##multiline", &mut input_text, [ui.io().display_size[0] - 16.0, ui.io().display_size[1] - 48.0])
                    .allow_tab_input(true)
                    .no_horizontal_scroll(true)
                    .callback(InputTextMultilineCallback::ALWAYS | InputTextMultilineCallback::EDIT, input_text_edit_handler)
                    .build();

                font_override.end();
                ui.separator();

                ui.text(format!("Font size: {}", input_text_props.input_font_size));

                let button_font_override = ui.push_font(*font_id_hash_map.get(&BUTTON_FONT_SIZE).unwrap());

                let button_repeat_override = ui.push_button_repeat(true);
                ui.same_line_with_spacing(135.0, 0.0);
                if ui.button_with_size("+", [25.0, 20.0]) {
                    update_font_size(&mut input_text_props, &mut input_text, 1);
                };

                ui.same_line_with_spacing(165.0,0.0);
                if ui.button_with_size("-", [25.0, 20.0]) {
                    update_font_size(&mut input_text_props, &mut input_text, -1);
                };
                button_repeat_override.end();

                let always_on_top_button_color;
                if window_props.always_on_top {
                    always_on_top_button_color = HALF_GREEN_BTN_HOVER;
                }
                else {
                    always_on_top_button_color = HALF_BLACK_FRAME;
                }

                let color_override = ui.push_style_color(StyleColor::Button, always_on_top_button_color);

                ui.same_line_with_pos(ui.io().display_size[0] - 183.0);
                if ui.button_with_size("Window always on Top", [145.0, 20.0]) {
                    window_props.always_on_top = !window_props.always_on_top;
                };

                color_override.end();

                ui.same_line_with_pos(ui.io().display_size[0] - 33.0);
                if ui.button_with_size("X", [25.0, 20.0]) {
                    exit(0x0100);
                };

                button_font_override.end();
            });

        return window_props;
    },);
}