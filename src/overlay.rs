use std::borrow::Borrow;

use winit::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::api::APIHandle;

pub struct Overlay {
    event_loop: EventLoop<()>,
    window: Window,
}

impl Overlay {
    //Creates a new window with
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(false)
            .with_always_on_top(true)
            .build(&event_loop)
            .unwrap();

        Overlay { event_loop, window }
    }

    //Runs the main loop for the overlay
    pub fn run(self, process_handle: APIHandle) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                //Exits the program when required
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,

                //Updates overlay state
                Event::MainEventsCleared => {}

                _ => (),
            }
        });
    }
}
