extern crate gl;
extern crate sdl2;
use std::ffi::CString;
use std::time;
use std::env;

mod mat;
mod loader;
mod model;
mod shader;
mod mvp;

use loader::Loader;
use model::Model;
use shader::Program;
use mvp::MVP;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<(), String>{
    let mut model = Model::new();

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let loader = Loader::new(args[1].clone());
        loader.parse(&mut model)?;
        // model.set_colors_one(1.0);
        // model.set_colors_gradation();
        // model.set_colors_gradation_colorful();
        model.set_colors_grain();
    } else {
        model.set_cube_sample();
        // model.set_rect_sample();
    }

    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 2);

    let window_size = (900, 700);

    let window = video_subsystem
        .window("Test", window_size.0, window_size.1)
        .opengl()
        .resizable()
        .build()
        .map_err(|_| "error: window build".to_string())?;

    let _gl_context = window.gl_create_context()?;
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    model.set_buffers();
    model.config_buffer();

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS)
    }

    let program = Program::from_shaders_source(
        &CString::new(include_str!("triangle.vert")).map_err(|_| "error: vertex shader".to_string())?,
        &CString::new(include_str!("triangle.frag")).map_err(|_| "error: fragment shader".to_string())?
    )?;

    let mut mvp = MVP::new(
        program.id(),
        model.get_max_size(),
        window_size
    )?;

    let mut before_timestamp = time::Instant::now();

    let mut event_pump = sdl.event_pump()?;
    'main: loop {
        let new_timestamp = time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mousestate,
                    x: _,
                    y: _,
                    xrel,
                    yrel
                } => {
                    let diff_timestamp = new_timestamp.duration_since(before_timestamp).as_millis();
                    if mousestate.left() {
                        mvp.update_translation_mouse(
                            xrel as f32,
                            yrel as f32,
                            diff_timestamp as f32
                        );
                    } else if mousestate.middle() {
                        mvp.update_rotate_matrix(
                            xrel,
                            yrel,
                            diff_timestamp as f32
                        );
                    }
                }
                sdl2::event::Event::MouseWheel {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    x: _,
                    y,
                    direction
                } => {
                    if let sdl2::mouse::MouseWheelDirection::Flipped = direction {
                        mvp.update_translation_wheel(y as f32)
                    } else {
                        mvp.update_translation_wheel(-y as f32)
                    }
                }
                _ => {}
            }
        }
        before_timestamp = new_timestamp;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        mvp.set();
        program.set_used();
        model.draw();
        window.gl_swap_window();
    }

    Ok(())
}
