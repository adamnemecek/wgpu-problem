
use crate::{
    Window,
    winit::{
        dpi::PhysicalPosition,
        event::{ElementState, Event, ModifiersState, MouseScrollDelta, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
        window::{CursorIcon, WindowBuilder},
    }
};

pub struct App {
    window: Window
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        Self { window: Window::new(event_loop) }
    }

}

pub fn app() {
    let event_loop = EventLoop::new();
    let mut app = App::new(&event_loop);

    event_loop.run(move |event, _, control_flow| match event {

            Event::RedrawRequested(_) => {


                // let mut frame = app.get_next_texture();

                // app.redraw(&mut frame);
                // // {

                // let encoder = app.create_command_encoder();
                // }


            },


        });
}