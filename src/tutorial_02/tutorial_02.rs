#[macro_use]
use glium::{*, glutin, Surface};

// It looks like glium expects us to define this for ourselves in each project
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],  // Array of f32 of size 2
}

implement_vertex!(Vertex, position);  // Imported from the `Surface` module, it appears

/// Build and display basic triangle
pub fn tutorial_02() {
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    // First set up vertices
    let vertex_1 = Vertex { position: [-0.5, -0.5] };
    let vertex_2 = Vertex { position: [ 0.0,  0.5] };
    let vertex_3 = Vertex { position: [ 0.5, -0.25] };

    // Assemble the vertices into a vector
    let shape = vec![vertex_1, vertex_2, vertex_3];

    // Create vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Indices for indexing into the vertex buffer.  This is just a dummy object for right now
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vert_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let frag_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &display,
        &vert_shader_src,
        &frag_shader_src,
        None
    ).unwrap();

    events_loop.run(move |ev, _, control_flow| {
        // Create a frame in the frame buffer that we can draw to and then have it render to the screen
        let mut target = display.draw();

        // Set the clear color
        target.clear_color(0.6, 0.1, 0.7, 1.0);

        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        // Tell Glium to tell OpenGL that we've finished drawing to the frame and that it's ready to be displayed in the
        // window.  This destroys this `Frame` object
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glium::glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                    glium::glutin::event::ElementState::Pressed => match input.virtual_keycode {
                        Some(glium::glutin::event::VirtualKeyCode::Escape) => {
                            *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                            return;
                        },
                        _ => return,
                    }
                    _ => return,
                }
                _ => return,
            },
            _ => (),
        }
    });
}