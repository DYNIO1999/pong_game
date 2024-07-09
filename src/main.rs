extern crate sdl2;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use gl::types::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;


use std::{os::raw::{c_char, c_void}, ptr::null};
use std::ffi::CStr;

extern "system" fn gl_debug_message_callback(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const c_char,
    _param: *mut c_void,
) {
    let source_str = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW_SYSTEM",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER_COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD_PARTY",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_OTHER => "OTHER",
        _ => "<SOURCE>",
    };

    let type_str = match type_ {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED_BEHAVIOR",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED_BEHAVIOR",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_OTHER => "OTHER",
        gl::DEBUG_TYPE_MARKER => "MARKER",
        _ => "<TYPE>",
    };

    let severity_str = match severity {
        gl::DEBUG_SEVERITY_HIGH => "HIGH",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
        gl::DEBUG_SEVERITY_LOW => "LOW",
        gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
        _ => "<SEVERITY>",
    };

    let message_str = unsafe { CStr::from_ptr(message).to_string_lossy().into_owned() };

    println!(
        "{}: GL {} {} ({}): {}",
        id, severity_str, type_str, source_str, message_str
    );
}

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
    
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(gl_debug_message_callback), std::ptr::null());
    
    }
    
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