// https://zenn.dev/kurun/articles/73d38631e7d0baf6b068

extern crate gl_generator;

use gl_generator::{Api, Fallbacks, Profile, Registry};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    // OpenGL 3.3 bindings
    let dest = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();
    let gl_extensions = [
        "GL_APPLE_client_storage",
        "GL_APPLE_fence",
        "GL_APPLE_texture_range",
        "GL_APPLE_vertex_array_object",
        "GL_ARB_blend_func_extended",
        "GL_ARB_buffer_storage",
        "GL_ARB_copy_image",
        "GL_ARB_get_program_binary",
        "GL_ARB_invalidate_subdata",
        "GL_ARB_texture_rectangle",
        "GL_ARB_texture_storage",
        "GL_EXT_debug_marker",
        "GL_EXT_texture_filter_anisotropic",
        "GL_KHR_debug",
        "GL_KHR_blend_equation_advanced",
        "GL_KHR_blend_equation_advanced_coherent",
        "GL_KHR_blend_equation_advanced_coherent",
        "GL_ARB_shader_storage_buffer_object",
    ];
    let gl_reg = Registry::new(
        Api::Gl,
        (3, 3), // Open GL の対象バージョン
        Profile::Compatibility,
        Fallbacks::All,
        gl_extensions,
    );
    gl_reg
        .write_bindings(gl_generator::StructGenerator, &mut file_gl)
        .unwrap();
}