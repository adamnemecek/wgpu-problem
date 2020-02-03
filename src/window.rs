
use winit::{
    event_loop::{EventLoop},
    window::{WindowBuilder},
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