
fn create_swap_chain(
    device: &wgpu::Device,
    surface: &wgpu::Surface,
    size: winit::dpi::PhysicalSize<u32>
) -> wgpu::SwapChain {
    device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Vsync,
        },
    )
}

pub struct Device {
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) swap_chain: wgpu::SwapChain,
    pub(crate) surface: wgpu::Surface,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
}

impl Device {
    pub fn new(window: &winit::window::Window) -> Self {
        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            backends: wgpu::BackendBit::PRIMARY,
        }).expect("Failed to create adapter");

        let (mut device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        });

        let size = window.inner_size();
        let swap_chain = create_swap_chain(&device, &surface, size);

        Self {
            adapter,
            device,
            swap_chain,
            surface,
            size
        }
    }
}