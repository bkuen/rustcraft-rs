use crate::graphics::mesh::{Mesh, Model};
use cgmath::{Vector3, Vector2};
use crate::world::block::face::BlockFace;
use crate::world::block::{Material, face};
use std::borrow::BorrowMut;
use std::iter::Enumerate;
use std::path::Iter;
use crate::graphics::gl::Gl;
use crate::graphics::texture::{TextureAtlas, Texture};
use crate::graphics::shader::ShaderProgram;
use crate::resources::Resources;
use crate::gl;
use crate::camera::PerspectiveCamera;
use crate::entity::Entity;

/// The size of each chunk
const CHUNK_SIZE:usize = 32;
/// The height of each chunk
const CHUNK_HEIGHT:usize = 32;
/// The area of each chunk. Usually, chunks have
/// a squared area.
const CHUNK_AREA:usize = CHUNK_SIZE * CHUNK_SIZE;
/// The volume of each chunk
const CHUNK_VOLUME:usize = CHUNK_AREA * CHUNK_HEIGHT;

/// Chunk
///
/// A chunks is a unit storing a bunch of blocks
/// in order not to render a big world at once.
/// Therefore, the whole world is split into many
/// chunks of the same size.
/// By the default configuration, each chunk is `16*16*256`
/// blocks big.
/// All the blocks are stored in a heap allocated array of
/// bytes, each byte represents a certain block material and
/// refers indirectly to its block data. Hence, only `~65 kilobytes`
/// are required to represent a whole chunk.
struct Chunk<'a> {
    /// The blocks stored in the chunk
    blocks: Box<[Material; CHUNK_VOLUME]>,
    /// The texture atlas which is used to texture the blocks
    tex_atlas: &'a TextureAtlas
}

impl<'a> Chunk<'a> {
    /// Creates a new chunk.
    /// By default, this chunk is filled with grass blocks.
    ///
    /// # Arguments
    ///
    /// * `text_atlas` - The texture atlas which should be used to
    /// texture the blocks
    pub fn new(tex_atlas: &'a TextureAtlas) -> Self {
        Self {
            tex_atlas,
            blocks: Box::new([Material::Grass; CHUNK_VOLUME]),
        }
    }

    /// Places a block to the given location
    ///
    /// # Argument
    ///
    /// * `loc` - The location the block should be placed
    /// * `material` - The material of the
    ///
    /// # Safety
    ///
    /// If the location is out of bounds, the block won't be placed
    pub fn set_block(&mut self, loc: Vector3<i16>, material: Material) {
        if let Some(index) = self.index_of(loc) {
            let mut blocks = *self.blocks;
            blocks[index] = material;
        }
    }

    /// Returns all blocks of the chunk as `Iter`
    pub fn blocks(&self) -> &[Material; CHUNK_VOLUME] {
        &*self.blocks
    }

    /// Returns the texture atlas which is used to texture the blocks
    pub fn tex_atlas(&self) -> &'a TextureAtlas {
        self.tex_atlas
    }

    /// Returns the material of a given chunk
    ///
    /// # Argument
    ///
    /// * `loc` - The location of the block in the chunk
    ///
    /// # Safety
    ///
    /// If the location is out of bounds, a `None` will be
    /// returned
    pub fn block(&self, loc: Vector3<i16>) -> Option<Material> {
        if let Some(index) = self.index_of(loc) {
            return Some(self.blocks[index]);
        }
        None
    }

    /// Returns the index of a given location
    ///
    /// # Argument
    ///
    /// * `index` - The index of the block
    ///
    /// # Safety
    ///
    /// Index needs to be between 0 (incl.) and `CHUNK_HEIGHT` (excl.). Otherwise,
    /// a `None` will be returned. Negative numbers are just allowed to calculate
    /// neighbored blocks.
    fn index_of(&self, loc: Vector3<i16>) -> Option<usize> {
        if !(
            loc.x >= 0 &&
            loc.y >= 0 &&
            loc.z >= 0 &&
            loc.x < CHUNK_SIZE as i16 &&
            loc.y < CHUNK_HEIGHT as i16 &&
            loc.z < CHUNK_SIZE as i16
        ) {
            return None
        }
        Some(CHUNK_AREA * loc.y as usize + CHUNK_SIZE * loc.z as usize + loc.x as usize)
    }
}

/// ChunkMesh
///
/// Each chunk will be rendered with a single
/// mesh. This structs offers methods to add a
/// block face to the mesh at a certain position.
struct ChunkMesh {
    /// The underlying 'normal' mesh
    mesh: Mesh,
    /// The current index,
    current_index: u32,
}

impl Default for ChunkMesh {
    fn default() -> Self {
        Self {
            mesh: Mesh::default(),
            current_index: 0
        }
    }
}

impl ChunkMesh {
    /// Add a face to the mesh
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the block
    /// * `face` - The block face which should be added to the mesh
    /// * `material` - The material of the block
    /// * `tex_atlas` - The texture atlas which is used to texture the blocks
    pub fn add_face(&mut self, pos: &Vector3<i16>, face: &BlockFace, material: Material, tex_atlas: &TextureAtlas) {
        let mesh = self.mesh.borrow_mut();

        // Add vertex positions to mesh
        mesh.vertex_positions.reserve(12);

        let mut index = 0;
        for i in 0..4 {
            mesh.vertex_positions.push(pos.x as f32 + face[index]);
            mesh.vertex_positions.push(pos.y as f32 + face[index + 1]);
            mesh.vertex_positions.push(pos.z as f32 + face[index + 2]);
            index+=3;
        }

        // Add indices to mesh
        mesh.indices.reserve(6);

        mesh.indices.push(self.current_index);
        mesh.indices.push(self.current_index + 1);
        mesh.indices.push(self.current_index + 2);

        mesh.indices.push(self.current_index + 2);
        mesh.indices.push(self.current_index + 3);
        mesh.indices.push(self.current_index);

        self.current_index += 4;

        // Create texture coords
        let tex_top = tex_atlas.sub_texture(Vector2::new(1.0, 15.0));
        let tex_side = tex_atlas.sub_texture(Vector2::new(0.0, 15.0));
        let tex_bottom = tex_atlas.sub_texture(Vector2::new(2.0, 15.0));

        // Add texture coords
        mesh.tex_coords.reserve(8);
        match *face {
            face::FRONT | face::BACK | face::RIGHT | face::LEFT => mesh.tex_coords.extend_from_slice(tex_side.coords()),
            face::TOP => mesh.tex_coords.extend_from_slice(tex_top.coords()),
            face::BOTTOM => mesh.tex_coords.extend_from_slice(tex_bottom.coords()),
            _ => unreachable!()
        }
    }
}

/// ChunkRenderer
///
/// This is a renderer which renders
/// `Minecraft-like` chunks
pub struct ChunkRenderer {
    /// An `OpenGL` instance
    gl: Gl,
    /// A texture atlas
    tex_atlas: TextureAtlas,
    /// A shader program
    shader_program: ShaderProgram,
    /// The chunk positions
    chunk_positions: Vec<Vector2<f32>>,
}

impl ChunkRenderer {

    /// Creates a new chunk renderer
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

        Self {
            shader_program,
            tex_atlas,
            chunk_positions: Vec::new(),
            gl: gl.clone()
        }
    }

    /// Adds a position to the chunk list
    ///
    /// # Arguments
    ///
    /// * `pos` - The position which should be added to the cube list
    pub fn add(&mut self, pos: Vector2<f32>) {
        self.chunk_positions.push(pos);
    }

    /// Renders the scene
    ///
    /// # Arguments
    ///
    /// * `camera` - A perspective camera
    pub fn render(&mut self, camera: &PerspectiveCamera) {
        let shader_program = self.shader_program.borrow_mut();
        shader_program.enable();
        shader_program.set_uniform_1i("u_Texture", 0);

        self.tex_atlas.bind(None);

        for pos in self.chunk_positions.iter() {
            let chunk = Chunk::new(&self.tex_atlas);
            let mesh = make_chunk_mesh(&chunk).mesh;

            let chunk_model = Model::from_mesh(&self.gl, &mesh);
            chunk_model.bind();

            // Create a new entity
            let ent = Entity::at_pos(Vector3::new(pos.x * CHUNK_SIZE as f32, 0.0, pos.y * CHUNK_SIZE as f32));

            // Calculate model view projection matrix
            let model = ent.model_matrix();
            let view = camera.view_matrix();
            let proj = camera.proj_matrix();
            let mvp = proj * view * model;
            shader_program.set_uniform_mat4f("u_MVP", &mvp);

            // `OpenGL` draw call
            unsafe {
                self.gl.DrawElements(
                    gl::TRIANGLES,
                    chunk_model.ib().index_count() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }

            chunk_model.unbind();
        }

        self.tex_atlas.unbind();
        shader_program.disable();
        self.chunk_positions.clear();
    }

    /// Clears the `OpenGL` rendered context
    pub fn clear(&self) {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.Clear(gl::DEPTH_BUFFER_BIT);
        }
    }

}

/// This function determines whether a face should be added to
/// the mesh
///
/// # Arguments
///
/// * `block_mat` - The material of the origin block
/// * `neighbored_mat_opt` - The material of the neighbored block
fn make_face(block_mat: Material, neighbored_opt: Option<Material>) -> bool {
    if let None = neighbored_opt {
        return true;
    }
    let neighbored = neighbored_opt.unwrap();

    if block_mat == Material::Air || neighbored != Material::Air {
        return false;
    }

    true
}

/// This function generates a chunk mesh
/// from a given chunk
///
/// # Arguments
///
/// * `chunk` - The chunk for which a mesh
/// should be generated
fn make_chunk_mesh(chunk: &Chunk) -> ChunkMesh {
    let mut mesh = ChunkMesh::default();

    for y in 0..CHUNK_HEIGHT {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let block_loc = Vector3::new(x as i16, y as i16, z as i16);
                let block_mat = chunk.block(block_loc).unwrap();

                let mut neighbored_mat_opt;

                // Front
                neighbored_mat_opt = chunk.block(Vector3::new(block_loc.x, block_loc.y, block_loc.z + 1));
                if make_face(block_mat, neighbored_mat_opt) {
                    mesh.add_face(&block_loc, &face::FRONT, block_mat, chunk.tex_atlas());
                }

                // Back
                neighbored_mat_opt = chunk.block(Vector3::new(block_loc.x, block_loc.y, block_loc.z - 1));
                if make_face(block_mat, neighbored_mat_opt) {
                    mesh.add_face(&block_loc, &face::BACK, block_mat, chunk.tex_atlas());
                }

                // Right
                neighbored_mat_opt = chunk.block(Vector3::new(block_loc.x + 1, block_loc.y, block_loc.z));
                if make_face(block_mat, neighbored_mat_opt) {
                    mesh.add_face(&block_loc, &face::RIGHT, block_mat, chunk.tex_atlas());
                }

                // Left
                neighbored_mat_opt = chunk.block(Vector3::new(block_loc.x - 1, block_loc.y, block_loc.z));
                if make_face(block_mat, neighbored_mat_opt) {
                    mesh.add_face(&block_loc, &face::LEFT, block_mat, chunk.tex_atlas());
                }

                // Top
                neighbored_mat_opt = chunk.block(Vector3::new(block_loc.x, block_loc.y + 1, block_loc.z));
                if make_face(block_mat, neighbored_mat_opt) {
                    mesh.add_face(&block_loc, &face::TOP, block_mat, chunk.tex_atlas());
                }

                // Bottom
                neighbored_mat_opt = chunk.block(Vector3::new(block_loc.x, block_loc.y - 1, block_loc.z));
                if make_face(block_mat, neighbored_mat_opt) {
                    mesh.add_face(&block_loc, &face::BOTTOM, block_mat, chunk.tex_atlas());
                }
            }
        }
    }

    mesh
}