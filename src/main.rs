use std::process::exit;
use imgui::{Condition, InputTextCallbackHandler, InputTextMultilineCallback, TextCallbackData};

use crate::support::{MAX_FONT_SIZE, MIN_FONT_SIZE, Mod};

mod support;

pub const BUTTON_FONT_SIZE: i32 = 13;

#[derive(Copy, Clone)]
struct InputTextProps {
    pub input_font_size: i32,
    pub input_width: i32,
}

fn main() {
    let Mod { system,font_id_hash_map, mut window_props } = support::init("Sufler");
    let mut input_text = String::from("укрїнська мова");
    let mut input_font_size: i32 = system.font_size as i32;

    let mut input_text_props = InputTextProps {
        input_font_size,
        input_width: 0,
    };

    let update_font_size = |input_font_size: &mut i32, change: i32| {
        let size = input_font_size.clone() + change;
        if size < MIN_FONT_SIZE || size > MAX_FONT_SIZE {
            return;
        }
        *input_font_size = size.clone();
    };
    const WRAP_TEXT_CONTENT: fn(String, i32, i32) -> String = |content: String, display_width_px: i32, font_size_px: i32| {
        let max_chars_in_line = display_width_px / (font_size_px / 2);
        let pure_text = content.replace("\r\n", "");
        let lines = pure_text.split("\n");
        let mut wrapped_text_lines = vec![];

        {
            for line in lines {
                let chars_in_line = line.chars().count();

                if chars_in_line <= max_chars_in_line as usize {
                    wrapped_text_lines.push(line.to_string());
                    continue;
                }

                println!("---------------------");
                println!("Max chars in line: {}", max_chars_in_line);
                println!("Chars in line: {}", chars_in_line);
                println!("{}", line);

                let mut words_length = 0;
                let mut words_buffer = String::from("");
                let words = line.split_whitespace();

                for word in words {

                }
            }
        }

        return String::from(content);
    };

    struct InputTextEditCallbackHandler {
        input_text_props: InputTextProps
    }
    impl InputTextCallbackHandler for InputTextEditCallbackHandler {
        // let on_edit =
        fn on_edit(&mut self, mut data: TextCallbackData) {
            let mut chars = data.str().chars();
            println!("{:?}", chars);
            println!("Font: {}", self.input_text_props.input_font_size);
            println!("width: {}", self.input_text_props.input_width);


            // data.remove_chars(0, chars.count());
            let wrapped_string: &String = &WRAP_TEXT_CONTENT(
                String::from(data.str()),
                self.input_text_props.input_width,
                self.input_text_props.input_font_size,
            );

            data.clear();
            data.insert_chars(0, wrapped_string);
        }
    }

    system.main_loop(move |_, ui| {
        let font_id_hash_map = &font_id_hash_map;
        let mut input_text_edit_handler = InputTextEditCallbackHandler { input_text_props };
        input_text_edit_handler.input_text_props.input_width = (ui.io().display_size[0] - 21.0) as i32;

        ui.window("main")
            .size(ui.io().display_size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .scroll_bar(true)
            // .always_auto_resize(true)
            // .focus_on_appearing(true)
            // .ti
            .title_bar(false)
            .no_decoration()
            .bg_alpha(0.30)
            .build(|| {
                ui.separator();
                let font_override = ui.push_font(*font_id_hash_map.get(&input_text_props.input_font_size).unwrap());

                ui.input_text_multiline("##multiline", &mut input_text, [ui.io().display_size[0] - 16.0, ui.io().display_size[1] - 48.0])
                    .allow_tab_input(true)
                    .always_overwrite(true)
                    .no_horizontal_scroll(true)
                    .callback(InputTextMultilineCallback::EDIT, input_text_edit_handler)
                    .build();

                font_override.end();
                ui.separator();

                ui.text(format!("Font size: {}", input_text_props.input_font_size));

                let button_font_override = ui.push_font(*font_id_hash_map.get(&BUTTON_FONT_SIZE).unwrap());

                let button_repeat_override = ui.push_button_repeat(true);
                ui.same_line_with_spacing(135.0, 0.0);
                if ui.button_with_size("+", [25.0, 20.0]) {
                    update_font_size(&mut input_text_props.input_font_size, 1);
                };

                ui.same_line_with_spacing(165.0,0.0);
                if ui.button_with_size("-", [25.0, 20.0]) {
                    update_font_size(&mut input_text_props.input_font_size, -1);
                };
                button_repeat_override.end();

                ui.same_line_with_pos(ui.io().display_size[0] - 183.0);
                if ui.button_with_size("Window always on Top", [145.0, 20.0]) {
                    window_props.always_on_top = !window_props.always_on_top;
                };

                ui.same_line_with_pos(ui.io().display_size[0] - 33.0);
                if ui.button_with_size("X", [25.0, 20.0]) {
                    exit(0x0100);
                };

                button_font_override.end();
            });
        return window_props;
    },);
}
