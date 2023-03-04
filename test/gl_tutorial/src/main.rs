extern crate gl;
extern crate sdl2;
use std::ffi::{CStr, CString};
use std::time;

mod mat;

use mat::{Point, Axis, Matrix};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<(), String>{
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

    // let vertices: Vec<f32> = vec![
    //     -1.0, -1.0, 0.0,
    //     1.0, -1.0, 0.0,
    //     0.0, 1.0, 0.0
    // ];
    let mut vertices: Vec<Point> = vec![
        Point::new(-0.5,-0.5,-0.5),
        Point::new(-0.5,-0.5, 0.5),
        Point::new(-0.5, 0.5, 0.5),
        Point::new(0.5, 0.5,-0.5),
        Point::new(-0.5,-0.5,-0.5),
        Point::new(-0.5, 0.5,-0.5),
        Point::new(0.5,-0.5, 0.5),
        Point::new(-0.5,-0.5,-0.5),
        Point::new(0.5,-0.5,-0.5),
        Point::new(0.5, 0.5,-0.5),
        Point::new(0.5,-0.5,-0.5),
        Point::new(-0.5,-0.5,-0.5),
        Point::new(-0.5,-0.5,-0.5),
        Point::new(-0.5, 0.5, 0.5),
        Point::new(-0.5, 0.5,-0.5),
        Point::new(0.5,-0.5, 0.5),
        Point::new(-0.5,-0.5, 0.5),
        Point::new(-0.5,-0.5,-0.5),
        Point::new(-0.5, 0.5, 0.5),
        Point::new(-0.5,-0.5, 0.5),
        Point::new(0.5,-0.5, 0.5),
        Point::new(0.5, 0.5, 0.5),
        Point::new(0.5,-0.5,-0.5),
        Point::new(0.5, 0.5,-0.5),
        Point::new(0.5,-0.5,-0.5),
        Point::new(0.5, 0.5, 0.5),
        Point::new(0.5,-0.5, 0.5),
        Point::new(0.5, 0.5, 0.5),
        Point::new(0.5, 0.5,-0.5),
        Point::new(-0.5, 0.5,-0.5),
        Point::new(0.5, 0.5, 0.5),
        Point::new(-0.5, 0.5,-0.5),
        Point::new(-0.5, 0.5, 0.5),
        Point::new(0.5, 0.5, 0.5),
        Point::new(-0.5, 0.5, 0.5),
        Point::new(0.5,-0.5, 0.5)
    ];
    let color_data: Vec<f32> = vec![
        0.583,  0.771,  0.014,
        0.609,  0.115,  0.436,
        0.327,  0.483,  0.844,
        0.822,  0.569,  0.201,
        0.435,  0.602,  0.223,
        0.310,  0.747,  0.185,
        0.597,  0.770,  0.761,
        0.559,  0.436,  0.730,
        0.359,  0.583,  0.152,
        0.483,  0.596,  0.789,
        0.559,  0.861,  0.639,
        0.195,  0.548,  0.859,
        0.014,  0.184,  0.576,
        0.771,  0.328,  0.970,
        0.406,  0.615,  0.116,
        0.676,  0.977,  0.133,
        0.971,  0.572,  0.833,
        0.140,  0.616,  0.489,
        0.997,  0.513,  0.064,
        0.945,  0.719,  0.592,
        0.543,  0.021,  0.978,
        0.279,  0.317,  0.505,
        0.167,  0.620,  0.077,
        0.347,  0.857,  0.137,
        0.055,  0.953,  0.042,
        0.714,  0.505,  0.345,
        0.783,  0.290,  0.734,
        0.722,  0.645,  0.174,
        0.302,  0.455,  0.848,
        0.225,  0.587,  0.040,
        0.517,  0.713,  0.338,
        0.053,  0.959,  0.120,
        0.393,  0.621,  0.362,
        0.673,  0.211,  0.457,
        0.820,  0.883,  0.371,
        0.982,  0.099,  0.879
    ];

    // for vertex in &mut vertices {
    //     *vertex = rotate_one_axis(vertex, Axis::X, std::f32::consts::FRAC_PI_4);
    //     *vertex = rotate_one_axis(vertex, Axis::Y, std::f32::consts::FRAC_PI_4);
    //     *vertex = rotate_one_axis(vertex, Axis::Z, std::f32::consts::FRAC_PI_4);
    // }

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<Point>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
    }

    let mut color_buffer: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut color_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (color_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            color_data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
    }

    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            0,
            std::ptr::null(),
        );
    }

    unsafe {
        gl::EnableVertexAttribArray(1);
        gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            0,
            std::ptr::null(),
        );
    }

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS)
    }

    let program_id = load_shader(
        &CString::new(include_str!("triangle.vert")).map_err(|_| "error: vertex shader".to_string())?,
        &CString::new(include_str!("triangle.frag")).map_err(|_| "error: fragment shader".to_string())?
    )?;

    let translation_matrix_name = CString::new("Translation").map_err(|_| "error: GetUniformLocation".to_string())?;
    let translation_matrix_id = unsafe {gl::GetUniformLocation(program_id, translation_matrix_name.as_ptr()) };

    let rotation_matrix_name = CString::new("Rotation").map_err(|_| "error: GetUniformLocation".to_string())?;
    let rotation_matrix_id = unsafe {gl::GetUniformLocation(program_id, rotation_matrix_name.as_ptr()) };

    let projection_matrix = Matrix::make_perspective_projection_matrix(
        std::f32::consts::FRAC_PI_4,
        window_size.0 as f32 / window_size.1 as f32,
        -1.0,
        20.0
    );
    let projection_matrix_name = CString::new("Projection").map_err(|_| "error: GetUniformLocation".to_string())?;
    let projection_matrix_id = unsafe {gl::GetUniformLocation(program_id, projection_matrix_name.as_ptr()) };
    let projection_matrix_ptr = unsafe { std::mem::transmute(&projection_matrix) };

    let mut translation_x = 0.0;
    let mut translation_y = 0.0;
    let mut translation_z = 2.0;

    let mut direction_vec = Point::new(-1.0, 1.0, -1.0).normalize();
    let mut rotation_matrix = Matrix::make_rotate_matrix(&direction_vec, std::f32::consts::FRAC_PI_4).orthonormalization();
    let mut rotation_matrix_ptr = unsafe { std::mem::transmute(&rotation_matrix) };

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
                            direction_vec = Point::new(0.0, v, 0.0);
                        } else if xrel == 0 {
                            let v = if yrel > 0 {1} else {-1} as f32;
                            direction_vec = Point::new(v, 0.0, 0.0);
                        } else {
                            direction_vec = Point::new(yrel as f32, xrel as f32, 0.0).normalize();
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
            gl::UniformMatrix4fv(projection_matrix_id, 1, gl::TRUE, projection_matrix_ptr);
            gl::UseProgram(program_id);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                12*3,
                // 3,
            );
        }

        window.gl_swap_window();
    }

    Ok(())
}


fn load_shader(vertex_shader: &CStr, fragment_shader: &CStr) -> Result<gl::types::GLuint, String> {
    let vertex_shader_id = shader_from_source(vertex_shader, gl::VERTEX_SHADER)?;
    let fragment_shader_id = shader_from_source(fragment_shader, gl::FRAGMENT_SHADER)?;

    let program_id = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program_id, vertex_shader_id);
        gl::AttachShader(program_id, fragment_shader_id);
        gl::LinkProgram(program_id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(program_id, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
        return shader_error_case_return(program_id)
    }

    unsafe {
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);
    }

    Ok(program_id)
}


fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        return shader_error_case_return(id)
    }

    Ok(id)
}

fn shader_error_case_return(id: u32) -> Result<gl::types::GLuint, String> {
    let mut len: gl::types::GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }

    let error = create_whitespace_cstring_with_len(len as usize);

    unsafe {
        gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        );
    }

    return Err(error.to_string_lossy().into_owned());
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
