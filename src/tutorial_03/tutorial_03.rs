#[macro_use]
use glium::{*, glutin, Surface};

// It looks like glium expects us to define this for ourselves in each project
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],  // Array of f32 of size 2
}

implement_vertex!(Vertex, position);  // Imported from the `Surface` module, it appears

/// Build and display basic triangle
pub fn tutorial_03() {
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let vert_shader_src = r#"
        #version 140
        in vec2 position;
        uniform float t;
        void main() {
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
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

    let mut t: f32 = -0.5;

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    events_loop.run(move |ev, _, control_flow| {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        // Create a frame in the frame buffer that we can draw to and then have it render to the screen
        let mut target = display.draw();

        // Set the clear color
        target.clear_color(0.6, 0.1, 0.7, 1.0);

        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniform! { t: t },
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