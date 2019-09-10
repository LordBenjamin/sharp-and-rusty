extern crate raw_window_handle;
extern crate wgpu;

pub struct Renderer {
    device : wgpu::Device,
    bind_group : wgpu::BindGroup,
    render_pipeline : wgpu::RenderPipeline,
    swap_chain: wgpu::SwapChain,
    surface: wgpu::Surface,
}

impl Renderer {
    pub fn new(window: raw_window_handle::RawWindowHandle, size: glutin::dpi::PhysicalSize) -> Renderer {
        match window {
            raw_window_handle::RawWindowHandle::Windows(h) => println!("create {:?}", h.hwnd),
            _ => {}
        }

        let instance = wgpu::Instance::new();
        let surface = instance.create_surface(window);

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
        });

        let device = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let vs = include_bytes!("shaders/shader.vert.spv");
        let vs_module =
            device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap());

        let fs = include_bytes!("shaders/shader.frag.spv");
        let fs_module =
            device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap());

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { bindings: &[] });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let swap_chain = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: size.width.round() as u32,
                height: size.height.round() as u32,
                present_mode: wgpu::PresentMode::Vsync,
            },
        );

        Renderer {
            device : device,
            bind_group : bind_group,
            render_pipeline : render_pipeline,
            swap_chain: swap_chain,
            surface,
        }
    }

    pub fn draw(&mut self) {
        let frame = self.swap_chain.get_next_texture();
                let mut encoder =
                    self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color::GREEN,
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&self.render_pipeline);
                    rpass.set_bind_group(0, &self.bind_group, &[]);
                    rpass.draw(0 .. 3, 0 .. 1);
                }

                self.device.get_queue().submit(&[encoder.finish()]);
    }

    pub fn resize(&mut self, size: glutin::dpi::PhysicalSize) {
        println!("resize {:?}", size);
        self.swap_chain = self.device.create_swap_chain(
            &self.surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: size.width.round() as u32,
                height: size.height.round() as u32,
                present_mode: wgpu::PresentMode::Vsync,
            },
        );
    }
}
