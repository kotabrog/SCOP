extern crate gl;
extern crate sdl2;
use std::ffi::{CStr, CString};

mod mat;

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

    let window = video_subsystem
        .window("Test", 900, 700)
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
    let vertices: Vec<f32> = vec![
        -0.5,-0.5,-0.5,
        -0.5,-0.5, 0.5,
        -0.5, 0.5, 0.5,
        0.5, 0.5,-0.5,
        -0.5,-0.5,-0.5,
        -0.5, 0.5,-0.5,
        0.5,-0.5, 0.5,
        -0.5,-0.5,-0.5,
        0.5,-0.5,-0.5,
        0.5, 0.5,-0.5,
        0.5,-0.5,-0.5,
        -0.5,-0.5,-0.5,
        -0.5,-0.5,-0.5,
        -0.5, 0.5, 0.5,
        -0.5, 0.5,-0.5,
        0.5,-0.5, 0.5,
        -0.5,-0.5, 0.5,
        -0.5,-0.5,-0.5,
        -0.5, 0.5, 0.5,
        -0.5,-0.5, 0.5,
        0.5,-0.5, 0.5,
        0.5, 0.5, 0.5,
        0.5,-0.5,-0.5,
        0.5, 0.5,-0.5,
        0.5,-0.5,-0.5,
        0.5, 0.5, 0.5,
        0.5,-0.5, 0.5,
        0.5, 0.5, 0.5,
        0.5, 0.5,-0.5,
        -0.5, 0.5,-0.5,
        0.5, 0.5, 0.5,
        -0.5, 0.5,-0.5,
        -0.5, 0.5, 0.5,
        0.5, 0.5, 0.5,
        -0.5, 0.5, 0.5,
        0.5,-0.5, 0.5
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

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
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

    let mut event_pump = sdl.event_pump()?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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
