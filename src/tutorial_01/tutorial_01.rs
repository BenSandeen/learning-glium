use glium::Surface;

pub fn tutorial_01() {
    use glium::glutin;

    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    // Continuously loop to display the window and update it
    events_loop.run(move |ev, _, control_flow| {
        // Create a frame in the frame buffer that we can draw to and then have it render to the screen
        let mut target = display.draw();

        // Set the clear color
        target.clear_color(0.6, 0.1, 0.7, 1.0);

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