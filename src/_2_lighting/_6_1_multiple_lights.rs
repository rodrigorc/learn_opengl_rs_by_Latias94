use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use anyhow::Result;
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_2_6_1() {
    let init_info = WindowInitInfo::builder()
        .title("Multiple Lights".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

// set up vertex data (and buffer(s)) and configure vertex attributes
// ------------------------------------------------------------------
#[rustfmt::skip]
const VERTICES: [f32; 288] = [
    // pos             normal           tex_coord
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
    0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
    -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

    -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
    0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
    0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
    -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
    0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
    0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
    0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
    0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
];

const CUBE_POSITIONS: [glm::Vec3; 10] = [
    glm::Vec3::new(0.0, 0.0, 0.0),
    glm::Vec3::new(2.0, 5.0, -15.0),
    glm::Vec3::new(-1.5, -2.2, -2.5),
    glm::Vec3::new(-3.8, -2.0, -12.3),
    glm::Vec3::new(2.4, -0.4, -3.5),
    glm::Vec3::new(-1.7, 3.0, -7.5),
    glm::Vec3::new(1.3, -2.0, -2.5),
    glm::Vec3::new(1.5, 2.0, -2.5),
    glm::Vec3::new(1.5, 0.2, -1.5),
    glm::Vec3::new(-1.3, 1.0, -1.5),
];

const POINT_LIGHTS_POSITIONS: [glm::Vec3; 4] = [
    glm::Vec3::new(0.7, 0.2, 2.0),
    glm::Vec3::new(2.3, -3.3, -4.0),
    glm::Vec3::new(-4.0, 2.0, -12.0),
    glm::Vec3::new(0.0, 0.0, -3.0),
];

struct App {
    cube_vao: VertexArray,
    light_vao: VertexArray,
    vbo: Buffer,
    diffuse_map: Texture,
    specular_map: Texture,
    lighting_shader: MyShader,
    lighting_cube_shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();
        let lighting_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/4.1.lighting_maps.vs"),
            include_str!("./shaders/6.1.multiple_lights.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let lighting_cube_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/1.1.light_cube.vs"),
            include_str!("./shaders/1.1.light_cube.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let camera_pos = glm::vec3(0.0, 0.0, 3.0);
        let camera = Camera::new_with_position(camera_pos);

        gl.enable(DEPTH_TEST);

        // first, configure the cube's VAO (and VBO)
        let vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);

        let cube_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(cube_vao));
        // position attribute
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 8 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        // normal attribute
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            FLOAT,
            false,
            8 * size_of::<f32>() as i32,
            3 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(1);
        // texture coord attribute
        gl.vertex_attrib_pointer_f32(
            2,
            2,
            FLOAT,
            false,
            8 * size_of::<f32>() as i32,
            6 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(2);

        // second, configure the light's VAO (VBO stays the same; the vertices are the same for the light object which is also a 3D cube)
        let light_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(light_vao));
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        // note that we update the lamp's position attribute's stride to reflect the updated buffer data
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 8 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        // load textures
        let diffuse_map = load_texture_from_bytes(
            gl,
            include_bytes!("../../resources/textures/container2.png"),
        )
        .expect("Failed to load texture");

        lighting_shader.use_shader(gl);
        lighting_shader.set_int(gl, "material.diffuse", 0);

        let specular_map = load_texture_from_bytes(
            gl,
            include_bytes!("../../resources/textures/container2_specular.png"),
        )
        .expect("Failed to load texture");
        lighting_shader.set_int(gl, "material.specular", 1);

        Self {
            cube_vao,
            light_vao,
            vbo,
            lighting_shader,
            lighting_cube_shader,
            camera,
            diffuse_map,
            specular_map,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();
        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        // be sure to activate shader when setting uniforms/drawing objects
        self.lighting_shader.use_shader(gl);
        self.lighting_shader
            .set_vec3(gl, "viewPos", &self.camera.position());
        self.lighting_shader
            .set_float(gl, "material.shininess", 32.0);

        /*
           Here we set all the uniforms for the 5/6 types of lights we have. We have to set them manually and index
           the proper PointLight struct in the array to set each uniform variable. This can be done more code-friendly
           by defining light types as classes and set their values in there, or by using a more efficient uniform approach
           by using 'Uniform buffer objects', but that is something we'll discuss in the 'Advanced GLSL' tutorial.
        */
        // directional light
        self.lighting_shader
            .set_vec3(gl, "dirLight.direction", &glm::vec3(-0.2, -1.0, -0.3));
        self.lighting_shader
            .set_vec3(gl, "dirLight.ambient", &glm::vec3(0.05, 0.05, 0.05));
        self.lighting_shader
            .set_vec3(gl, "dirLight.diffuse", &glm::vec3(0.4, 0.4, 0.4));
        self.lighting_shader
            .set_vec3(gl, "dirLight.specular", &glm::vec3(0.5, 0.5, 0.5));

        // for each point light, we set the position and other attributes
        for (i, pos) in POINT_LIGHTS_POSITIONS.iter().enumerate() {
            self.lighting_shader
                .set_vec3(gl, &format!("pointLights[{}].position", i), pos);
            self.lighting_shader.set_vec3(
                gl,
                &format!("pointLights[{}].ambient", i),
                &glm::vec3(0.05, 0.05, 0.05),
            );
            self.lighting_shader.set_vec3(
                gl,
                &format!("pointLights[{}].diffuse", i),
                &glm::vec3(0.8, 0.8, 0.8),
            );
            self.lighting_shader.set_vec3(
                gl,
                &format!("pointLights[{}].specular", i),
                &glm::vec3(1.0, 1.0, 1.0),
            );
            self.lighting_shader
                .set_float(gl, &format!("pointLights[{}].constant", i), 1.0);
            self.lighting_shader
                .set_float(gl, &format!("pointLights[{}].linear", i), 0.09);
            self.lighting_shader
                .set_float(gl, &format!("pointLights[{}].quadratic", i), 0.032);
        }

        // spotLight
        self.lighting_shader
            .set_vec3(gl, "spotLight.position", &self.camera.position());
        self.lighting_shader
            .set_vec3(gl, "spotLight.direction", &self.camera.front());
        self.lighting_shader
            .set_vec3(gl, "spotLight.ambient", &glm::vec3(0.0, 0.0, 0.0));
        self.lighting_shader
            .set_vec3(gl, "spotLight.diffuse", &glm::vec3(1.0, 1.0, 1.0));
        self.lighting_shader
            .set_vec3(gl, "spotLight.specular", &glm::vec3(1.0, 1.0, 1.0));
        self.lighting_shader
            .set_float(gl, "spotLight.constant", 1.0);
        self.lighting_shader.set_float(gl, "spotLight.linear", 0.09);
        self.lighting_shader
            .set_float(gl, "spotLight.quadratic", 0.032);
        self.lighting_shader
            .set_float(gl, "spotLight.cutOff", 12.5_f32.to_radians().cos());
        self.lighting_shader
            .set_float(gl, "spotLight.outerCutOff", 15.0_f32.to_radians().cos());

        // view/projection transformations
        let projection = glm::perspective(
            ctx.width() as f32 / ctx.height() as f32,
            self.camera.zoom().to_radians(),
            0.1,
            100.0,
        );
        let view = self.camera.view_matrix();
        self.lighting_shader.set_mat4(gl, "projection", &projection);
        self.lighting_shader.set_mat4(gl, "view", &view);

        // bind diffuse map
        gl.active_texture(TEXTURE0);
        gl.bind_texture(TEXTURE_2D, Some(self.diffuse_map));
        // bind specular map
        gl.active_texture(TEXTURE1);
        gl.bind_texture(TEXTURE_2D, Some(self.specular_map));

        gl.bind_vertex_array(Some(self.cube_vao));

        for (i, pos) in CUBE_POSITIONS.iter().enumerate() {
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, pos);
            let angle = 20.0 * i as f32;
            model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1.0, 0.3, 0.5));
            self.lighting_shader.set_mat4(gl, "model", &model);

            gl.draw_arrays(TRIANGLES, 0, 36);
        }

        // also draw the lamp object
        self.lighting_cube_shader.use_shader(gl);
        self.lighting_cube_shader
            .set_mat4(gl, "projection", &projection);
        self.lighting_cube_shader.set_mat4(gl, "view", &view);

        gl.bind_vertex_array(Some(self.light_vao));
        // we now draw as many light bulbs as we have point lights.
        for pos in &POINT_LIGHTS_POSITIONS {
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, pos);
            model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2)); // a smaller cube
            self.lighting_cube_shader.set_mat4(gl, "model", &model);
            gl.draw_arrays(TRIANGLES, 0, 36);
        }
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.lighting_shader.delete(gl);
        self.lighting_cube_shader.delete(gl);

        gl.delete_vertex_array(self.cube_vao);

        gl.delete_vertex_array(self.light_vao);

        gl.delete_buffer(self.vbo);

        gl.delete_texture(self.diffuse_map);

        gl.delete_texture(self.specular_map);
    }
}

fn load_texture_from_bytes(gl: &Context, bytes: &[u8]) -> Result<Texture> {
    let img = image::load_from_memory(bytes)?.flipv().to_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();
    let texture = unsafe {
        let texture = gl.create_texture().expect("Create texture");
        gl.bind_texture(TEXTURE_2D, Some(texture));
        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            RGBA as i32,
            width as i32,
            height as i32,
            0,
            RGBA,
            UNSIGNED_BYTE,
            Some(&data),
        );
        gl.generate_mipmap(TEXTURE_2D);

        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

        texture
    };
    Ok(texture)
}
