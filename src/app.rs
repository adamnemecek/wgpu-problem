
use {
    crate::{
        Window,
    },
    winit::{
        dpi::PhysicalPosition,
        event::{ElementState, Event, ModifiersState, MouseScrollDelta, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    }
};

pub struct App {
    window: Window
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        Self { window: Window::new(event_loop) }
    }


    pub fn get_next_texture<'a>(&'a mut self) -> wgpu::SwapChainOutput<'a> {
        self.window.device.swap_chain.get_next_texture()
    }

    pub fn create_command_encoder(&mut self) -> wgpu::CommandEncoder {
        self.window.device.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 })
        // todo!()
    }

    // pub fn redraw<'a>(&'a mut self, frame: &'a mut wgpu::SwapChainOutput) {
    //     let frame = self.get_next_texture();
    //     let encoder = self.create_command_encoder();

    // }

}

pub fn app() {
    let event_loop = EventLoop::new();
    let mut app = App::new(&event_loop);

    event_loop.run(move |event, _, control_flow| match event {
        _ => ()
            // Event::RedrawRequested(_) => {


            //     // let mut frame = app.get_next_texture();

            //     // app.redraw(&mut frame);
            //     // // {

            //     // let encoder = app.create_command_encoder();
            //     // }


            // },


        });
}