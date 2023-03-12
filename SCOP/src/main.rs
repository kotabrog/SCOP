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
mod img_loader;

use loader::Loader;
use model::Model;
use shader::Program;
use mvp::MVP;
use img_loader::Image;

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
        model.resolve_duplicate_indices();
        // model.set_colors_one(1.0);
        // model.set_colors_gradation();
        // model.set_colors_gradation_colorful();
        // model.set_colors_grain();
        model.set_colors_gray(6);
    } else {
        // model.set_cube_sample();
        // model.set_rect_sample();
        // model.set_rect_uv_sample();
        model.set_cube_sample_uv();
        model.resolve_duplicate_indices();
        // model.set_colors_grain();
        // model.set_colors_gradation();
        model.set_colors_gray(6);
    }
    let img = Image::new("./scop/asserts/textures/sweets_shiroi_taiyaki_white.bmp")?;

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

    model.set_uv();
    model.set_buffers();
    model.config_buffer();

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::TEXTURE_2D);
        gl::DepthFunc(gl::LESS)
    }

    let program = Program::from_shaders_source(
        &CString::new(include_str!("../asserts/shaders/triangle.vert")).map_err(|_| "error: vertex shader".to_string())?,
        &CString::new(include_str!("../asserts/shaders/triangle.frag")).map_err(|_| "error: fragment shader".to_string())?
    )?;

    let mut mvp = MVP::new(
        program.id(),
        model.get_max_size(),
        model.get_vertices(),
        window_size
    )?;

    model.set_texture(program.id())?;

    // let mut bits: [[[f32; 3]; 64]; 64] = [[[0.0; 3]; 64]; 64];
    // for i in 0..64 {
    //     let r = (i * 4) as f32 / 256.0;
    //     for j in 0..64 {
    //         bits[i][j][0] = r;
    //         bits[i][j][1] = (j * 4) as f32 / 256.0;
    //         bits[i][j][2] = r;
    //     }
    // }
    let mut bits = vec![0.0; 64 * 64 * 3];
    for i in 0..64 {
        let r = (i * 4) as f32 / 256.0;
        for j in 0..64 {
            bits[i * 64 * 3 + j * 3 + 0] = r;
            bits[i * 64 * 3 + j * 3 + 1] = (j * 4) as f32 / 256.0;
            bits[i * 64 * 3 + j * 3 + 2] = r;
        }
    }
    let mut tex_id: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut tex_id);
        gl::BindBuffer(gl::TEXTURE_2D, tex_id);
    }
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.get_width() as i32,
            img.get_height() as i32,
            0,
            gl::RGB,
            gl::FLOAT,
            img.get_ptr(),
        );
        // gl::TexImage2D(
        //     gl::TEXTURE_2D,
        //     0,
        //     gl::RGB as i32,
        //     64 as i32,
        //     64 as i32,
        //     0,
        //     gl::RGB,
        //     gl::FLOAT,
        //     // gl::FLOAT,
        //     // gl::UNSIGNED_INT,
        //     bits.as_ptr() as *const _,
        // );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    }


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
                    } else if mousestate.right() && xrel != 0 {
                        mvp.update_z_rotate_matrix(xrel, diff_timestamp as f32);
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
                sdl2::event::Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode: Some(sdl2::keyboard::Keycode::F1),
                    scancode: _,
                    keymod: sdl2::keyboard::Mod::NOMOD,
                    repeat: false
                } => {
                    model.switch_texture();
                },
                _ => {}
            }
        }
        before_timestamp = new_timestamp;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        program.set_used();
        mvp.set();
        model.draw()?;
        window.gl_swap_window();
    }

    Ok(())
}
