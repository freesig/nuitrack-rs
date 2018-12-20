extern crate nannou;
extern crate nuitrack_rs as nuitrack;

use nannou::prelude::*;
use nannou::vulkano;
use nannou::vulkano::buffer::{BufferUsage, ImmutableBuffer};
use nannou::vulkano::command_buffer::DynamicState;
use nannou::vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use nannou::vulkano::device::DeviceOwned;
use nannou::vulkano::format::Format;
use nannou::vulkano::framebuffer::{RenderPassAbstract, Subpass};
use nannou::vulkano::image::{Dimensions, ImmutableImage};
use nannou::vulkano::pipeline::viewport::Viewport;
use nannou::vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use nannou::vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use nannou::window::SwapchainFramebuffers;
use nuitrack::{Color3, Joint};
use std::cell::RefCell;
use std::sync::{mpsc, Arc};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    rx: Receivers,
    current: Current,
    graphics: Graphics,
}

struct Graphics {
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    framebuffers: RefCell<SwapchainFramebuffers>,
    sampler: Arc<Sampler>,
}

#[derive(Debug, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    mode: u32,
}

nannou::vulkano::impl_vertex!(Vertex, mode, position, tex_coords);

impl Vertex {
    const MODE_RGBA: u32 = 0;
    const MODE_DEPTH: u32 = 1;
    const MODE_SKELETON: u32 = 2;
}

// The most recently received skeletons, depth and color.
struct Current {
    skeletons: Option<Vec<Skeleton>>,
    depth: Option<DepthFrame>,
    color: Option<ColorFrame>,
    // The current color image on the GPU.
    rgba_image: Arc<ImmutableImage<Format>>,
    depth_image: Arc<ImmutableImage<Format>>,
}

struct Receivers {
    skeletons: mpsc::Receiver<Vec<Skeleton>>,
    depth: mpsc::Receiver<DepthFrame>,
    color: mpsc::Receiver<ColorFrame>,
}

struct Skeleton {
    joints: Vec<Joint>,
}

struct DepthFrame {
    rows: u32,
    cols: u32,
    data: Vec<u16>,
}

struct ColorFrame {
    rows: u32,
    cols: u32,
    data: Vec<Color3>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_dimensions(640 * 2, 480)
        .view(view)
        .build()
        .unwrap();

    /////////////////////
    // Nuitrack Thread //
    /////////////////////

    let recording_path = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .expect("must specify a file. e.g. `cargo run --example nannou_playback -- foo.snap`");

    let (skeletons_tx, skeletons) = mpsc::channel();
    let (depth_tx, depth) = mpsc::channel();
    let (color_tx, color) = mpsc::channel();

    std::thread::spawn(move || {
        let mut nui = nuitrack_rs::playback(recording_path, true)
            .expect("Couldn't create player");

        nui.skeleton_data(move |data| {
            let skeletons = data.skeletons()
                .iter()
                .map(|skeleton| Skeleton {
                    joints: skeleton.joints().to_vec()
                })
                .collect();
            skeletons_tx.send(skeletons).ok();
        }).expect("Failed to add callback");

        nui.depth_data(move |data| {
            let depth = DepthFrame {
                rows: data.rows as _,
                cols: data.cols as _,
                data: data.frame().to_vec(),
            };
            depth_tx.send(depth).ok();
        }).expect("Failed to add callback");

        nui.color_data(move |data| {
            let color = ColorFrame {
                rows: data.rows as _,
                cols: data.cols as _,
                data: data.frame().to_vec(),
            };
            color_tx.send(color).ok();
        }).expect("Failed to add callback");

        // Run at ~30fps forever.
        loop {
            nui.update().expect("failed to update nui player");
            std::thread::sleep(std::time::Duration::from_millis(32));
        }
    });
    let rx = Receivers { skeletons, depth, color };

    /////////////////////////////
    // Graphics Initialisation //
    /////////////////////////////

    let device = app.main_window().swapchain().device().clone();

    let vertex_shader = vs::Shader::load(device.clone()).unwrap();
    let fragment_shader = fs::Shader::load(device.clone()).unwrap();

    let render_pass = Arc::new(
        nannou::vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: app.main_window().swapchain().format(),
                    samples: 1,
                    initial_layout: ImageLayout::PresentSrc,
                    final_layout: ImageLayout::PresentSrc,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

    // Use dummy images for RGBA and depth for now until we get some data.
    let (width, height) = (640, 480);
    let (rgba_image, _future) = ImmutableImage::from_iter(
        (0..width * height).map(|_| [0u8, 0, 0, 1]),
        Dimensions::Dim2d { width, height },
        Format::R8G8B8A8Srgb,
        app.main_window().swapchain_queue().clone(),
    ).unwrap();
    let (depth_image, _future) = ImmutableImage::from_iter(
        (0..width * height).map(|_| std::u16::MAX),
        Dimensions::Dim2d { width, height },
        Format::R16Unorm,
        app.main_window().swapchain_queue().clone(),
    ).unwrap();

    // The sampler that will be used to sample the RGBA and depth textures.
    let sampler = Sampler::new(
        device.clone(),
        Filter::Linear,
        Filter::Linear,
        MipmapMode::Nearest,
        SamplerAddressMode::ClampToEdge,
        SamplerAddressMode::ClampToEdge,
        SamplerAddressMode::ClampToEdge,
        0.0,
        1.0,
        0.0,
        0.0,
    )
    .unwrap();

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vertex_shader.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fragment_shader.main_entry_point(), ())
            .blend_alpha_blending()
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    let framebuffers = RefCell::new(SwapchainFramebuffers::default());

    let graphics = Graphics {
        render_pass,
        pipeline,
        framebuffers,
        sampler,
    };

    let current = Current {
        skeletons: None,
        depth: None,
        color: None,
        rgba_image,
        depth_image,
    };

    Model {
        rx,
        current,
        graphics,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Update the skeleton.
    if let Some(skeletons) = model.rx.skeletons.try_iter().last() {
        model.current.skeletons = Some(skeletons);
    }

    // Update the current depth.
    if let Some(depth) = model.rx.depth.try_iter().last() {
        let (width, height) = (depth.cols as _, depth.rows as _);
        let (depth_image, _future) = ImmutableImage::from_iter(
            depth.data.iter().cloned(),
            Dimensions::Dim2d { width, height },
            Format::R16Unorm,
            app.main_window().swapchain_queue().clone(),
        ).unwrap();
        model.current.depth_image = depth_image;
        model.current.depth = Some(depth);
    }

    // Update the current rgb.
    if let Some(color) = model.rx.color.try_iter().last() {
        let (width, height) = (color.cols as _, color.rows as _);
        let (rgba_image, _future) = ImmutableImage::from_iter(
            color.data.iter().map(|c| [c.red, c.green, c.blue, 255]),
            Dimensions::Dim2d { width, height },
            Format::R8G8B8A8Srgb,
            app.main_window().swapchain_queue().clone(),
        ).unwrap();
        model.current.rgba_image = rgba_image;
        model.current.color = Some(color);
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let [img_w, img_h] = frame.swapchain_image().dimensions();
    let graphics = &model.graphics;

    // Dynamic state.
    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [img_w as _, img_h as _],
        depth_range: 0.0..1.0,
    };
    let dynamic_state = DynamicState {
        line_width: None,
        viewports: Some(vec![viewport]),
        scissors: None,
    };

    // The descriptor set for the rgb texture.
    let descriptor_set = Arc::new(
        PersistentDescriptorSet::start(graphics.pipeline.clone(), 0)
            .add_sampled_image(model.current.rgba_image.clone(), graphics.sampler.clone())
            .unwrap()
            .add_sampled_image(model.current.depth_image.clone(), graphics.sampler.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    // Update framebuffers so that count matches swapchain image count and dimensions match.
    graphics.framebuffers.borrow_mut()
        .update(&frame, graphics.render_pass.clone(), |builder, image| builder.add(image))
        .unwrap();

    // Left, right, top, bottom and middle values for vertex and texture coordinate systems.
    let (vl, vr, vt, vb, vm) = (-1.0, 1.0, -1.0, 1.0, 0.0);
    let (tl, tr, tt, tb) = (0.0, 1.0, 0.0, 1.0);

    // Rgba texture quad on the left.
    let rgba_vertices = [
        Vertex { position: [vl, vt], tex_coords: [tl, tt], mode: Vertex::MODE_RGBA },
        Vertex { position: [vl, vb], tex_coords: [tl, tb], mode: Vertex::MODE_RGBA },
        Vertex { position: [vm, vt], tex_coords: [tr, tt], mode: Vertex::MODE_RGBA },
        Vertex { position: [vl, vb], tex_coords: [tl, tb], mode: Vertex::MODE_RGBA },
        Vertex { position: [vm, vb], tex_coords: [tr, tb], mode: Vertex::MODE_RGBA },
        Vertex { position: [vm, vt], tex_coords: [tr, tt], mode: Vertex::MODE_RGBA },
    ];

    // Depth texture quad on the right.
    let depth_vertices = [
        Vertex { position: [vm, vt], tex_coords: [tl, tt], mode: Vertex::MODE_DEPTH },
        Vertex { position: [vm, vb], tex_coords: [tl, tb], mode: Vertex::MODE_DEPTH },
        Vertex { position: [vr, vt], tex_coords: [tr, tt], mode: Vertex::MODE_DEPTH },
        Vertex { position: [vm, vb], tex_coords: [tl, tb], mode: Vertex::MODE_DEPTH },
        Vertex { position: [vr, vb], tex_coords: [tr, tb], mode: Vertex::MODE_DEPTH },
        Vertex { position: [vr, vt], tex_coords: [tr, tt], mode: Vertex::MODE_DEPTH },
    ];

    // Skeleton vertices as squares overlaid on the Rgba texture half of the window.
    let mut skeleton_vertices = vec![];
    if let Some(ref skeletons) = model.current.skeletons {
        for skeleton in skeletons {
            for joint in &skeleton.joints {
                let w = 8.0 / img_w as f32;
                let h = 8.0 / img_h as f32;
                let vs = Rect::from_w_h(w, h)
                    .shift([joint.proj.x - 1.0, joint.proj.y * 2.0 - 1.0].into())
                    //.shift_x(-1.0)
                    .triangles_iter()
                    .flat_map(|tri| tri.vertices())
                    .map(|v| Vertex {
                        position: [v.x, v.y],
                        tex_coords: [0.0, 0.0],
                        mode: Vertex::MODE_SKELETON,
                    });
                skeleton_vertices.extend(vs);
            }
        }
    }

    // Chain all the vertices together ready for vertex buffer creation.
    let vertices = rgba_vertices.iter().cloned()
        .chain(depth_vertices.iter().cloned())
        .chain(skeleton_vertices)
        .collect::<Vec<_>>();

    // The vertex buffer that will be submitted to the GPU.
    let (vertex_buffer, _future) = ImmutableBuffer::from_iter(
        vertices.into_iter(),
        BufferUsage::all(),
        app.main_window().swapchain_queue().clone(),
    ).unwrap();

    let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];

    frame
        .add_commands()
        .begin_render_pass(
            graphics.framebuffers.borrow()[frame.swapchain_image_index()].clone(),
            false,
            clear_values,
        )
        .unwrap()
        .draw(
            graphics.pipeline.clone(),
            &dynamic_state,
            vec![vertex_buffer.clone()],
            descriptor_set,
            (),
        )
        .unwrap()
        .end_render_pass()
        .expect("failed to add `end_render_pass` command");

    frame
}

//////////////////
// GLSL Shaders //
//////////////////

mod vs {
    nannou::vulkano_shaders::shader! {
    ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 tex_coords;
layout(location = 2) in uint mode;

layout(location = 0) out vec2 v_tex_coords;
layout(location = 1) flat out uint v_mode;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    //v_tex_coords = position * 0.5 + 0.5;
    v_tex_coords = tex_coords;
    v_mode = mode;
}"
    }
}

mod fs {
    nannou::vulkano_shaders::shader! {
    ty: "fragment",
        src: "
#version 450

layout(location = 0) in vec2 tex_coords;
layout(location = 1) flat in uint mode;

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform sampler2D rgba_sampler;
layout(set = 0, binding = 1) uniform sampler2D depth_sampler;

void main() {
    // RGBA.
    if (mode == uint(0)) {
        f_color = texture(rgba_sampler, tex_coords);
    // Depth.
    } else if (mode == uint(1)) {
        float d = texture(depth_sampler, tex_coords).x;
        f_color = vec4(d, d, d, 1.0);
    // Skeleton.
    } else {
        f_color = vec4(1.0, 0.2, 0.6, 1.0);
    }
}"
    }
}
