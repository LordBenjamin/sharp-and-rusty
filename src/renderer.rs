extern crate glutin;

use glutin::{ ContextBuilder, PossiblyCurrent, }; 
use glutin::platform::windows::{ RawContextExt, };
use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::os::raw::c_void;
use std::ffi::CString;

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor; // Specify a vertex attribute for color
out vec3 color;
void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
	color = aColor; // pass the color along to the fragment shader
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;
in vec3 color;
void main()
{
   // Set the fragment color to the color passed from the vertex shader
   FragColor = vec4(color, 1.0);
}
"#;

pub struct Renderer {
    #[allow(dead_code)]
    pub hwnd: *const c_void,

    context: glutin::RawContext<PossiblyCurrent>,
    shader_program: GLuint,
    vao: GLuint,
}

impl Renderer {
    pub fn new(hwnd: *mut c_void) -> Renderer {
        let raw_context = unsafe {
            ContextBuilder::new()
                .build_raw_context(hwnd)
                .unwrap()
        };
        
        let raw_context = unsafe { raw_context.make_current().unwrap() };

        // Create GL content
         gl::load_with(|symbol| raw_context.get_proc_address(symbol) as *const _);

        let (shader_program, vao) = unsafe {
            // Setup shader compilation checks
            let mut success = i32::from(gl::FALSE);
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // -1 to skip trialing null character

            // Vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            // Check for shader compilation errors
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    str::from_utf8(&info_log).unwrap()
                );
            }

            // Fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            // Check for shader compilation errors
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    str::from_utf8(&info_log).unwrap()
                );
            }

            // Link Shaders
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            // Check for linking errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    str::from_utf8(&info_log).unwrap()
                );
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            // Set up vao and vbos
            let vertices: [f32; 18] = [
                // left
                -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,

                // right
                0.5, -0.5, 0.0, 0.0, 1.0, 0.0,

                // top
                0.0,  0.5, 0.0, 0.0, 0.0, 1.0
            ];

            let (mut vbo, mut vao) = (0, 0);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * mem::size_of::<GLfloat>() as GLsizei,
                (3 * mem::size_of::<GLfloat>()) as *const c_void
            );


            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            // Wireframe
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            (shader_program, vao)
        };

        Renderer { hwnd: hwnd, context: raw_context, shader_program: shader_program, vao:vao }
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0.39, 0.58, 0.92, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        self.context.swap_buffers().unwrap();
    }

    pub fn resize(&mut self, size: glutin::dpi::PhysicalSize) {
        self.context.resize(size);
    }
}