use std::collections::HashMap;
use std::path::Path;
use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};

use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontId, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::window::Icon;

mod clipboard;

pub const MIN_FONT_SIZE: i32 = 8;
pub const MAX_FONT_SIZE: i32 = 44;

const SYSTEM_FONT_SIZE: f32 = 18.0;

const DEFAULT_ALWAYS_ON_TOP: bool = true;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
    pub display: glium::Display,
}

#[derive(Copy, Clone)]
pub struct WindowProps {
    pub always_on_top: bool
}

pub struct Mod {
    pub system: System,
    pub font_id_hash_map: HashMap<i32, FontId>,
    pub window_props: WindowProps,
}

pub fn init(title: &str) -> Mod {
    let window_props = WindowProps {
        always_on_top: DEFAULT_ALWAYS_ON_TOP
    };

    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new()
        .with_vsync(false)
        .with_depth_buffer(24)
        .with_hardware_acceleration(Option::from(true))
        .with_multisampling(0)
        // .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_double_buffer(Option::from(true));

    let builder = glutin::window::WindowBuilder::new()
        .with_title(title)
        .with_movable_by_window_background(true)
        .with_titlebar_hidden(true)
        .with_always_on_top(window_props.always_on_top)
        .with_decorations(false)
        .with_min_inner_size(PhysicalSize::new(760, 500))
        .with_fullsize_content_view(true)
        .with_transparent(true)
        .with_has_shadow(false);

    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    if let Some(backend) = clipboard::init() {
        imgui.set_clipboard_backend(backend);
    } else {
        eprintln!("Failed to initialize clipboard");
    }

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();

        let dpi_mode = if let Ok(hidpi_factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match hidpi_factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), window, dpi_mode);
    }

    // Fixed font size. Note imgui_winit_support uses "logical
    // pixels", which are physical pixels scaled by the devices
    // scaling factor. Meaning, 13.0 pixels should look the same size
    // on two different screens, and thus we do not need to scale this
    // value (as the scaling is handled by winit)
    let font_size = SYSTEM_FONT_SIZE;

    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../../resources/fonts/ubuntu-mono.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                // As imgui-glium-renderer isn't gamma-correct with
                // it's font rendering, we apply an arbitrary
                // multiplier to make the font a bit "heavier". With
                // default imgui-glow-renderer this is unnecessary.
                rasterizer_multiply: 1.5,
                // Oversampling font helps improve text rendering at
                // expense of larger font atlas texture.
                oversample_h: 4,
                oversample_v: 4,
                ..FontConfig::default()
            }),
        },
    ]);

    let mut font_id_hash_map = HashMap::new();
    let font_atlas = imgui.fonts();
    for font_size in MIN_FONT_SIZE..=MAX_FONT_SIZE {
        let font_id = font_atlas.add_font(&[
            FontSource::TtfData {
                data: include_bytes!("../../resources/fonts/ubuntu-mono.ttf"),
                size_pixels: font_size as f32,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.5,
                    oversample_h: 4,
                    oversample_v: 4,
                    ..FontConfig::default()
                }),
            },
            FontSource::TtfData {
                data: include_bytes!("../../resources/fonts/ubuntu-mono.ttf"),
                size_pixels: font_size as f32,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.5,
                    oversample_h: 4,
                    oversample_v: 4,
                    glyph_ranges: FontGlyphRanges::cyrillic(),
                    ..FontConfig::default()
                }),
            },
            FontSource::TtfData {
                data: include_bytes!("../../resources/fonts/nanum2.ttf"),
                size_pixels: font_size as f32,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.5,
                    oversample_h: 4,
                    oversample_v: 4,
                    // glyph_ranges: FontGlyphRanges::chinese_simplified_common(),

                    ..FontConfig::default()
                }),
            },
            FontSource::TtfData {
                data: include_bytes!("../../resources/fonts/sc.ttf"),
                size_pixels: font_size as f32,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.5,
                    oversample_h: 4,
                    oversample_v: 4,
                    // glyph_ranges: FontGlyphRanges::chinese_simplified_common(),

                    ..FontConfig::default()
                }),
            },
            FontSource::TtfData {
                data: include_bytes!("../../resources/fonts/mplus-jp.ttf"),
                size_pixels: font_size as f32,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.5,
                    oversample_h: 1,
                    oversample_v: 1,
                    glyph_ranges: FontGlyphRanges::japanese(),

                    ..FontConfig::default()
                }),
            },

            //
            // FontSource::TtfData {
            //     data: include_bytes!("../../resources/fonts/noto-sans-korean.otf"),
            //     size_pixels: font_size as f32,
            //     config: Some(FontConfig {
            //         rasterizer_multiply: 1.5,
            //         oversample_h: 4,
            //         oversample_v: 4,
            //         glyph_ranges: FontGlyphRanges::japanese(),
            //         ..FontConfig::default()
            //     }),
            // },
            // FontSource::TtfData {
            //     data: include_bytes!("../../resources/fonts/noto-sans-korean.otf"),
            //     size_pixels: font_size as f32,
            //     config: Some(FontConfig {
            //         rasterizer_multiply: 1.5,
            //         oversample_h: 4,
            //         oversample_v: 4,
            //         glyph_ranges: FontGlyphRanges::chinese_full(),
            //         ..FontConfig::default()
            //     }),
            // },
        ]);
        font_id_hash_map.insert(font_size, font_id);
    }

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    Mod {
        system: System {
            event_loop,
            imgui,
            platform,
            renderer,
            font_size,
            display,
        },
        font_id_hash_map,
        window_props,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) -> WindowProps + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            mut imgui,
            mut platform,
            mut renderer,
            display,
            ..
        } = self;
        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();
                let gl_window = display.gl_window();
                let mut winit_window = gl_window.window();

                // if ui.is_any_item_focused() {
                //     println!("item in focus")
                // }
                //
                // if ui.is_item_activated() {
                //     // println!("item in activated")
                // }
                //
                // if ui.is_item_clicked() {
                //     // println!("item in clicked")
                // }
                //
                // if ui.is_item_active() {
                //     // println!("item in active")
                // }

                // if ui.is_any_item_hovered() {
                //     // system_window.
                //     // println!("item is activfe");
                // }

                let mut run = true;
                let window_props: WindowProps = run_ui(&mut run, ui);
                winit_window.set_always_on_top(window_props.always_on_top);

                if !run {
                    *control_flow = ControlFlow::Exit;
                }



                let mut target = display.draw();

                target.clear_color_srgb(0.0, 0.0, 0.0, 0.7);
                //
                // target.clear_depth(5.0); // target.finish();
                platform.prepare_render(ui, gl_window.window());

                let draw_data = imgui.render();

                // target.clear_all_srgb((0.0, 0.0, 0.0, 0.0), 1.0, 1);

                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");

                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}

