use crate::graphics::texture::{Texture, TextureAtlas};
use crate::graphics::shader::ShaderProgram;
use crate::graphics::mesh::{Model, Mesh};
use crate::graphics::gl::{Gl, gl};
use crate::resources::Resources;
use crate::camera::{PerspectiveCamera};
use crate::entity::Entity;

use cgmath::{Vector2, Vector3, Vector4};
use std::borrow::BorrowMut;
use std::ops::Index;

pub mod face;

/// Material
///
/// A `Material` represents the 'type' of a block
/// as just one u8
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Material {
    Air = 0,
    Grass = 1,
    Dirt = 2,
    Stone = 3,
}

/// BlockTextureCoords
///
/// The `BlockTextureCoords` stores the texture coordinates
/// for the top, bottom and side view of a certain block.
pub struct BlockTextureCoords {
    /// The coordinates of the top view
    top: Vector2<f32>,
    /// The coordinate of the bottom view
    bottom: Vector2<f32>,
    /// The coordinates of the side view
    side: Vector2<f32>,
}

/// BlockData
///
/// The `BlockData` stores the nature, character and texture
/// of a certain block
pub struct BlockData {
    /// The name of the block
    name: String,
    /// The texture coordinates for the top, bottom
    /// and side view of the block.
    tex_coords: BlockTextureCoords,
}

impl BlockData {
    /// Returns the name of the block
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the texture coordinates for the top, bottom
    /// and side view of the block.
    pub fn tex_coords(&self) -> &BlockTextureCoords {
        &self.tex_coords
    }
}

/// CubeRender
///
/// This is a renderer which renders `Minecraft-like` cubes
/// with a texture. By default, it's using the `Grass` texture.
/// Later on, this renderer will be replaced by a chunk renderer
/// generating meshes using greedy meshing strategies or similar
/// rendering techniques
pub struct CubeRenderer {
    /// An `OpenGL` instance
    gl: Gl,
    /// A texture atlas
    tex_atlas: TextureAtlas,
    /// A shader program
    shader_program: ShaderProgram,
    /// The model of the cube
    cube_model: Model,
    /// The cube positions
    cube_positions: Vec<Vector3<f32>>,
}

impl CubeRenderer {

    /// Creates a new `CubeRenderer`
    ///
    /// # Arguments
    ///
    /// * `gl` - An `OpenGL` instance
    /// * `resources` - A resource instance
    pub fn new(gl: &Gl, resources: &Resources) -> Self {
        // Create shader program
        let shader_program = ShaderProgram::from_res(gl, resources, "basic").unwrap();
        shader_program.disable();

        // Create default texture atlas
        let texture = Texture::from_resource(gl, resources, "textures/textures.png");
        let tex_atlas = TextureAtlas::from_texture(texture, Vector2::new(16.0, 16.0));
        tex_atlas.unbind();

        // Create vertex coordinates
        let mut vertex_positions = Vec::<f32>::with_capacity(72);
        vertex_positions.extend(face::BACK.iter());
        vertex_positions.extend(face::FRONT.iter());
        vertex_positions.extend(face::RIGHT.iter());
        vertex_positions.extend(face::LEFT.iter());
        vertex_positions.extend(face::TOP.iter());
        vertex_positions.extend(face::BOTTOM.iter());

        // Create indices
        let indices = vec![
            0u32, 1, 2,
            2, 3, 0,

            4, 5, 6,
            6, 7, 4,

            8, 9, 10,
            10, 11, 8,

            12, 13, 14,
            14, 15, 12,

            16, 17, 18,
            18, 19, 16,

            20, 21, 22,
            22, 23, 20,
        ];

        // Create texture coords
        let tex_top = tex_atlas.sub_texture(Vector2::new(1.0, 15.0));
        let tex_side = tex_atlas.sub_texture(Vector2::new(0.0, 15.0));
        let tex_bottom = tex_atlas.sub_texture(Vector2::new(2.0, 15.0));

        let mut tex_coords = Vec::new();
        tex_coords.extend_from_slice(tex_side.coords());
        tex_coords.extend_from_slice(tex_side.coords());
        tex_coords.extend_from_slice(tex_side.coords());
        tex_coords.extend_from_slice(tex_side.coords());
        tex_coords.extend_from_slice(tex_top.coords());
        tex_coords.extend_from_slice(tex_bottom.coords());

        // Create mesh
        let cube_mesh = Mesh {
            vertex_positions,
            tex_coords,
            indices,
        };

        // Create model
        let cube_model = Model::from_mesh(gl, &cube_mesh);

        Self {
            tex_atlas,
            shader_program,
            cube_model,
            gl: gl.clone(),
            cube_positions: Vec::new(),
        }
    }

    /// Returns the cube model of the renderer
    pub fn cube_model(&self) -> &Model {
        &self.cube_model
    }

    /// Returns the shader program of the renderer
    pub fn shader_program(&mut self) -> &mut ShaderProgram {
        &mut self.shader_program
    }

    /// Returns the texture atlas of the renderer
    pub fn tex_atlas(&self) -> &TextureAtlas {
        &self.tex_atlas
    }

    /// Adds a position to the cube list
    ///
    /// # Arguments
    ///
    /// * `pos` - The position which should be added to the cube list
    pub fn add(&mut self, pos: Vector3<f32>) {
        self.cube_positions.push(pos);
    }

    /// Clears the `OpenGL` rendered context
    pub fn clear(&self) {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.Clear(gl::DEPTH_BUFFER_BIT);
            // self.gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }

    /// Render the scene
    ///
    /// # Arguments
    ///
    /// * `camera` - A perspective camera
    pub fn render(&mut self, camera: &PerspectiveCamera) {
        let shader_program = self.shader_program.borrow_mut();
        shader_program.enable();
        shader_program.set_uniform_1i("u_Texture", 0);

        self.tex_atlas.bind(None);
        self.cube_model.bind();

        for pos in self.cube_positions.iter() {
            // Create a new entity
            let ent = Entity::at_pos(pos.clone());

            // Calculate model view projection matrix
            let model = ent.model_matrix();

            // let model = Matrix4::from_translation(Vector3::new(0.0, 0.0, 4.0));
            let view = camera.view_matrix();
            let proj = camera.proj_matrix();
            let mvp = proj * view * model;
            shader_program.set_uniform_mat4f("u_MVP", &mvp);

            // `OpenGL` draw call
            unsafe {
                self.gl.DrawElements(
                    gl::TRIANGLES,
                    self.cube_model.ib().index_count() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
        }

        self.cube_model.unbind();
        self.tex_atlas.unbind();
        shader_program.disable();
        self.cube_positions.clear();
    }
}