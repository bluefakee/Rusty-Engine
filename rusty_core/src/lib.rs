use std::error::Error;

use glium::{Display, glutin::{window::WindowBuilder, ContextBuilder, event_loop::EventLoop, event::{WindowEvent, Event}}, Surface};

pub mod debug;


struct Program {
    display: Display
}


pub fn start() -> Result<(), Box<dyn Error>> {
    
    if let Err(err) = debug::try_initialize_logger() {
        panic!("Loggerinitialization failed ({}:?)", err);
    }

    debug::info!("Logger initialized");

    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new();
    let eventloop = EventLoop::new();
    let display = Display::new(wb, cb, &eventloop)?;
    
    let program = Program { display };

    debug::info!("Display created. Starting game_loop");

    eventloop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: windowevent, ..} => {
                match windowevent {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                        return;
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        let mut frame = program.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        if let Err(e) = frame.finish() {
            debug::error!("Error happened trying to draw frame: {:?}", e);
            control_flow.set_exit_with_code(1);
            return;
        }
    });
}
