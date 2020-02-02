
pub struct Device {
    device: wgpu::Device
}

impl Device {
    pub fn new() -> Self {
        // let device = 
        todo!()
    }
}

pub struct App {
    device: wgpu::Device
}

impl App {

    pub fn new() {
        todo!()
    }

    pub fn get_next_texture<'a>(self) -> wgpu::SwapChainOutput<'a> {
        todo!()
    }

    pub fn create_command_encoder(self) -> wgpu::CommandEncoder {
        todo!()
    }
}