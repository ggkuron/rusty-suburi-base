extern crate glutin;
extern crate parti_game as game;

pub fn main() {

    let width = 1024;
    let height = 768;

    let mut events_loop = glutin::EventsLoop::new();

    let window = {
        let wb = glutin::WindowBuilder::new()
            .with_title("PARTI")
            .with_dimensions(width, height);
        let gl_builder = glutin::ContextBuilder::new().with_vsync(true);

        glutin::GlWindow::new(wb, gl_builder, &events_loop).expect("new fa")
    };

    let mut app = game::App::new(
        window, width, height
    );

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Closed | 
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            state: glutin::ElementState::Pressed,
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        }, ..
                    } => running = false,
                    _ => app.handle_input(event) 
                }
            }
        });
        app.render();
    }
}

