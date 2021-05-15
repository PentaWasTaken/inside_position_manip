use std::cell::RefCell;

use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use pixels::{Pixels, SurfaceTexture};

use image::RgbaImage;
use imageproc::drawing::draw_text_mut;

use rusttype::{Font, Scale};

use crate::api::APIHandle;

pub struct Overlay {
    event_loop: EventLoop<()>,
    window: Window,
    pixels: Pixels,
    text: RefCell<String>,
}

const FONT_DATA: &[u8] = include_bytes!("./Calibri.ttf");

const XPOS_OFFSETS: &[usize] = &[0xF92610, 0x4c0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x10];

impl Overlay {
    //Creates a new overlay with a window and event loop
    pub fn new(size: (u32, u32)) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(false)
            .with_always_on_top(true)
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(size.0, size.1))
            .build(&event_loop)
            .unwrap();

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
        };

        Overlay {
            event_loop: event_loop,
            window,
            pixels,
            text: RefCell::new(String::new()),
        }
    }

    //Runs the main loop for the overlay
    pub fn run(self, api_handle: APIHandle) {
        let window = self.window;
        let mut pixels = self.pixels;

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                //Exits the program when required
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                //Updates overlay state
                Event::MainEventsCleared => {
                    //Set overlay position
                    let win_rect = api_handle.get_win_rect();
                    let win_size = window.inner_size();

                    let top = win_rect.bottom - win_size.height as i32;
                    window.set_outer_position(PhysicalPosition::new(win_rect.left, top));                    

                    window.request_redraw();
                }

                Event::RedrawRequested(_) => {
                    //Update overlay text
                    let x_pos = api_handle.read_memory_f32(XPOS_OFFSETS);
                    let mut text = String::new();
                    text = x_pos.to_string();
                    

                    //Loads the font. This might be slow as it loads it on every redraw. Fix later?
                    let font = Font::try_from_bytes(FONT_DATA).unwrap();

                    //Draws the text to the canvas. Placeholder!
                    let win_size = window.inner_size();
                    let mut canvas = RgbaImage::new(win_size.width, win_size.height);
                    draw_text_mut(
                        &mut canvas,
                        [255, 255, 255, 255].into(),
                        0,
                        0,
                        Scale::uniform(40.0),
                        &font,
                        &text
                    );

                    //Copies the canvas to the window buffer
                    let frame = pixels.get_frame();
                    frame.copy_from_slice(canvas.as_raw());

                    pixels.render().unwrap();
                }

                _ => (),
            }
        });
    }
}
