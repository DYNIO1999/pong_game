extern crate sdl2;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let window = video_subsystem.window("Pong Game", 1600, 900)
        .opengl()
        .build()
        .unwrap();

    let ctx = window.gl_create_context().unwrap();
    
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut is_running: bool = true;
    
    while is_running {
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    is_running = false; 
                },
                _ => {}
            }
        }
        
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
        
    }
}