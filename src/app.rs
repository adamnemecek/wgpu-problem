
use winit::{
    dpi::PhysicalPosition,
    // event::{ElementState, Event, ModifiersState, MouseScrollDelta, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{CursorIcon, WindowBuilder},
};

use crate::{
    Device
};



pub struct Window {
    pub window: winit::window::Window,
    pub device: Device,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("ngrid")
            .build(&event_loop).expect("Failed to create window");

        let device = Device::new(&window);

        Self {
            window,
            device
        }
    }

}