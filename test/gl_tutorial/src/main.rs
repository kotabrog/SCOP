extern crate gl;
extern crate sdl2;
use std::ffi::{CStr, CString};
use std::time;
use std::env;

mod mat;
mod loader;
mod model;
mod shader;

use loader::Loader;
use mat::{Vec3d, Axis, Matrix};
use model::Model;
use shader::Program;

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

    let translation_matrix_name = CString::new("Translation").map_err(|_| "error: GetUniformLocation".to_string())?;
    let translation_matrix_id = unsafe {gl::GetUniformLocation(program.id(), translation_matrix_name.as_ptr()) };

    let rotation_matrix_name = CString::new("Rotation").map_err(|_| "error: GetUniformLocation".to_string())?;
    let rotation_matrix_id = unsafe {gl::GetUniformLocation(program.id(), rotation_matrix_name.as_ptr()) };

    let scale_matrix_name = CString::new("Scale").map_err(|_| "error: GetUniformLocation".to_string())?;
    let scale_matrix_id = unsafe {gl::GetUniformLocation(program.id(), scale_matrix_name.as_ptr()) };

    let projection_matrix = Matrix::make_perspective_projection_matrix(
        std::f32::consts::FRAC_PI_4,
        window_size.0 as f32 / window_size.1 as f32,
        -1.0,
        20.0
    );
    let projection_matrix_name = CString::new("Projection").map_err(|_| "error: GetUniformLocation".to_string())?;
    let projection_matrix_id = unsafe {gl::GetUniformLocation(program.id(), projection_matrix_name.as_ptr()) };
    let projection_matrix_ptr = unsafe { std::mem::transmute(&projection_matrix) };

    let mut translation_x = 0.0;
    let mut translation_y = 0.0;
    let mut translation_z = 2.0;

    let mut direction_vec = Vec3d::new(-1.0, 1.0, -1.0).normalize();
    let mut rotation_matrix = Matrix::make_rotate_matrix(&direction_vec, 0.0).orthonormalization();
    let mut rotation_matrix_ptr = unsafe { std::mem::transmute(&rotation_matrix) };

    let scale_matrix = Matrix::make_scale_matrix((1.0 / model.get_max_size()).min(1000.0));
    let scale_matrix_ptr = unsafe { std::mem::transmute(&scale_matrix) };

    let mut before_timestamp = time::Instant::now();

    let speed = 0.5;
    let wheel_speed = 0.5;
    let rotation_speed = 0.5;

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
                        translation_x += xrel as f32 * speed * diff_timestamp as f32 / 1000.0;
                        translation_y -= yrel as f32 * speed * diff_timestamp as f32 / 1000.0;
                    } else if mousestate.middle() {
                        if yrel == 0 {
                            let v = if xrel > 0 {1} else {-1} as f32;
                            direction_vec = Vec3d::new(0.0, v, 0.0);
                        } else if xrel == 0 {
                            let v = if yrel > 0 {1} else {-1} as f32;
                            direction_vec = Vec3d::new(v, 0.0, 0.0);
                        } else {
                            direction_vec = Vec3d::new(yrel as f32, xrel as f32, 0.0).normalize();
                        }
                        let temp_matrix = Matrix::make_rotate_matrix(
                            &direction_vec,
                            std::f32::consts::PI * rotation_speed * diff_timestamp as f32 / 1000.0
                        );
                        rotation_matrix = temp_matrix.mul(&rotation_matrix);
                        rotation_matrix = rotation_matrix.orthonormalization();
                        rotation_matrix_ptr = unsafe { std::mem::transmute(&rotation_matrix) };
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
                        translation_z += y as f32 * wheel_speed;
                    } else {
                        translation_z -= y as f32 * wheel_speed;
                    }
                }
                _ => {}
            }
        }
        before_timestamp = new_timestamp;

        let translation_matrix = Matrix::make_translation_matrix(translation_x, translation_y, translation_z);
        let translation_matrix_ptr = unsafe { std::mem::transmute(&translation_matrix) };


        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UniformMatrix4fv(translation_matrix_id, 1, gl::TRUE, translation_matrix_ptr);
            gl::UniformMatrix4fv(rotation_matrix_id, 1, gl::TRUE, rotation_matrix_ptr);
            gl::UniformMatrix4fv(scale_matrix_id, 1, gl::TRUE, scale_matrix_ptr);
            gl::UniformMatrix4fv(projection_matrix_id, 1, gl::TRUE, projection_matrix_ptr);
        }
        program.set_used();
        model.draw();
        window.gl_swap_window();
    }

    Ok(())
}
