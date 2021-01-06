
use cgmath::{Vector3, Vector2};
use crate::world::block::{Material};
use crate::resources::Resources;
use crate::camera::PerspectiveCamera;
use crate::entity::Entity;
use crate::gl;
use crate::graphics::gl::Gl;
use crate::graphics::mesh::{Mesh, Model};
use crate::graphics::shader::ShaderProgram;
use crate::graphics::texture::{TextureAtlas, Texture};
use std::borrow::BorrowMut;
use std::ops::Deref;
use crate::graphics::buffer::{VertexBufferLayout, VertexBuffer};
use std::mem::size_of;
use crate::graphics::gl::types::GLvoid;

/// The size of each chunk
const CHUNK_SIZE:usize = 8;
/// The height of each chunk
const CHUNK_HEIGHT:usize = 8;
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

/// ChunkModel
///
/// A chunk model is built up by a chunk mesh and it is generating the
/// required buffers for an `OpenGL` render call to render the specific
/// chunk
struct ChunkModel {
    /// The underlying model
    model: Model,
}

impl Deref for ChunkModel {
    type Target = Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl ChunkModel {
    /// Creates a new model from a given chunk mesh
    ///
    /// # Arguments
    ///
    /// * `mesh` - A chunk mesh instance
    pub fn from_chunk_mesh(gl: &Gl, mesh: &ChunkMesh) -> Self {
        let mut model = Model::from_mesh(gl, &mesh.mesh);
        let vb_tile_coords = VertexBuffer::new(gl, mesh.tile_offsets.as_ptr() as *const GLvoid, mesh.tile_offsets.len() as isize * size_of::<f32>() as isize);

        let mut buffer_layout = VertexBufferLayout::new();
        buffer_layout.push_f32(2);
        model.va_mut().add_buffer(&vb_tile_coords, &buffer_layout);
        model.buffers_mut().push(vb_tile_coords);

        Self {
            model,
        }
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
    /// The tile offsets of the mesh
    tile_offsets: Vec<f32>,
    /// The current index,
    current_index: u32,
}

impl Default for ChunkMesh {
    fn default() -> Self {
        Self {
            mesh: Mesh::default(),
            tile_offsets: Vec::new(),
            current_index: 0
        }
    }
}

impl ChunkMesh {
    pub fn add_quad(&mut self,
        bottom_left: Vector3<f32>,
        top_left: Vector3<f32>,
        top_right: Vector3<f32>,
        bottom_right: Vector3<f32>,
        width: i32,
        height: i32,
        face: &VoxelFace,
        back_face: bool,
        tex_atlas: &TextureAtlas,
    ) {
        let mesh = self.mesh.borrow_mut();

        let vector_to_slice = |vector: Vector3<f32>| {
            [vector.x, vector.y, vector.z]
        };

        // Add vertex positions to mesh
        mesh.vertex_positions.reserve(12);
        mesh.vertex_positions.extend(&vector_to_slice(bottom_left));
        mesh.vertex_positions.extend(&vector_to_slice(bottom_right));
        mesh.vertex_positions.extend(&vector_to_slice(top_left));
        mesh.vertex_positions.extend(&vector_to_slice(top_right));

        // Add indices to mesh
        // Add indices to mesh
        mesh.indices.reserve(6);

        if back_face {
            mesh.indices.extend_from_slice(&[
                self.current_index + 2,
                self.current_index,
                self.current_index + 1,

                self.current_index + 1,
                self.current_index + 3,
                self.current_index + 2
            ]);
        } else {
            mesh.indices.extend_from_slice(&[
                self.current_index + 2,
                self.current_index + 3,
                self.current_index + 1,

                self.current_index + 1,
                self.current_index,
                self.current_index + 2,
            ]);
        }

        self.current_index += 4;

        // Add texture coords
        mesh.tex_coords.reserve(8);

        if face.side == Side::NORTH || face.side == Side::SOUTH {
            mesh.tex_coords.extend_from_slice(&[
                0.0,          0.0,
                0.0, width as f32,
                height as f32, 0.0,
                height as f32, width as f32,
            ]);
        } else {
            mesh.tex_coords.extend_from_slice(&[
                0.0,          0.0,
                width as f32, 0.0,
                0.0,          height as f32,
                width as f32, height as f32,
            ]);
        }


        // Add normals
        mesh.normals.reserve(12);
        let normal = face.side.normal();
        mesh.normals.extend_from_slice(&normal);
        mesh.normals.extend_from_slice(&normal);
        mesh.normals.extend_from_slice(&normal);
        mesh.normals.extend_from_slice(&normal);

        // Add tile coords
        self.tile_offsets.reserve(8);

        let push_tile_offset = |tile_offsets: &mut Vec<f32>, offset: [f32; 2]| {
            for _ in 0..4 {
                tile_offsets.extend_from_slice(&offset)
            }
        };

        match face.side {
            Side::TOP => push_tile_offset(&mut self.tile_offsets, [1.0, 15.0]),
            Side::BOTTOM => push_tile_offset(&mut self.tile_offsets, [2.0, 15.0]),
            _ => push_tile_offset(&mut self.tile_offsets, [0.0, 15.0]),
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
            let mesh = make_greedy_chunk_mesh(&chunk);

            let chunk_model = ChunkModel::from_chunk_mesh(&self.gl, &mesh);
            chunk_model.bind();

            // Create a new entity
            let ent = Entity::at_pos(Vector3::new(pos.x * CHUNK_SIZE as f32, 0.0, pos.y * CHUNK_SIZE as f32));

            // Calculate model view projection matrix
            let model = ent.model_matrix();
            let view = camera.view_matrix();
            let proj = camera.proj_matrix();
            let mvp = proj * view * model;
            shader_program.set_uniform_mat4f("u_MVP", &mvp);

            // println!("Index count: {}", chunk_model.ib().index_count());

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

/*
* These are just constants to keep track of which face we're dealing with -
* their actual values are unimportant - only that they're constant.
*/
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Side {
    SOUTH = 0,
    NORTH = 1,
    EAST = 2,
    WEST = 3,
    TOP = 4,
    BOTTOM = 5,
}

impl Side {
    /// Returns the normal of the side
    pub fn normal(&self) -> [f32; 3] {
        match *self {
            Side::SOUTH => [0.0, 0.0, -1.0],
            Side::NORTH => [0.0, 0.0, 1.0],
            Side::EAST => [-1.0, 0.0, 0.0],
            Side::WEST => [1.0, 0.0, 0.0],
            Side::TOP => [0.0, 1.0, 0.0],
            Side::BOTTOM => [0.0, -1.0, 0.0],
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct VoxelFace {
    side: Side,
    material: Material,
}

impl VoxelFace {
    fn new(chunk: &Chunk, loc: Vector3<i16>, side: Side) -> Self {
        Self {
            side,
            material: chunk.block(loc).unwrap(),
        }
    }
}

impl PartialEq for VoxelFace {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material // && self.transparent == other.transparent
    }
}

/// This function generates a chunk mesh
/// from a given chunk using `greedy meshing`
/// algorithm.
///
/// Code ported from this blog post:
/// `https://0fps.wordpress.com/2012/06/30/meshing-in-a-minecraft-game/`
///
/// # Arguments
///
/// * `chunk`- The chunk for which a mesh
/// should be generated
fn make_greedy_chunk_mesh(chunk: &Chunk) -> ChunkMesh {
    let mut mesh = ChunkMesh::default();

    /*
     * These are just working variables for the alogirthm -
     * almost all taken directly from Mikola Lysenko's javascript
     * implementation.
     */
    let (mut i, mut j, mut k, mut l, mut w, mut h, mut u, mut v, mut n, mut side);
    side = Side::SOUTH;

    let mut x = [0i16; 3];
    let mut q = [0i16; 3];
    let mut du = [0i16; 3];
    let mut dv = [0i16; 3];

    /*
     * We create a mask - this will contain the groups of matching voxels faces
     * as we proceed through the chunk in 6 directions - once for each face.
     */

    let mask_box = Box::new([None; CHUNK_SIZE * CHUNK_HEIGHT]);
    let mut mask= *mask_box;

    /*
     * These are just working variables to hold two faces during comparison.
     */

    let (mut face_op, mut face1_op);

    /*
     * We start with the lesser-spotted boolean for-loop (also known as the old
     * flippy floppy).
     *
     * The variable `back_face` will be `true` on the first iteration and `false`
     * on the second - this allows us to track which direction the indices should
     * run during creation of the quad.
     *
     * This loop runs twice, and the inner loop 3 times - totally 6 iterations - one
     * for each voxel face.
     */
    let mut back_face = true;
    let mut b = false;
    while b != back_face {

        /*
         * We sweep over the 3 dimensions - most of what follows is well described
         * by Mikola Lysenko in this post - and is ported from his Javascript
         * implementation. Where this implementation diverges, I've added commentary.
         */
        for d in 0..3 {

            u = (d + 1) % 3;
            v = (d + 2) % 3;

            x[0] = 0;
            x[1] = 0;
            x[2] = 0;

            q[0] = 0;
            q[1] = 0;
            q[2] = 0;
            q[d] = 1;

            /*
             * Here we're keeping track of the side that we're meshing.
             */
            if d == 0 {
                side = if back_face { Side::WEST } else { Side::EAST };
            } else if d == 1 {
                side = if back_face { Side::BOTTOM } else { Side::TOP };
            } else if d == 2 {
                side = if back_face { Side::SOUTH } else { Side::NORTH };
            }

            /*
             * We move through the dimensions from front to back
             */
            x[d] = -1;
            while x[d] < CHUNK_SIZE as i16 {
                /*
                 * We compute the mask
                 */
                n = 0;

                x[v] = 0;
                while x[v] < CHUNK_HEIGHT as i16 {
                    x[u] = 0;
                    while x[u] < CHUNK_SIZE as i16 {
                        /*
                         * Here we retrieve two voxel faces for comparison.
                         */
                        face_op = if x[d] >= 0 {
                            Some(VoxelFace::new(&chunk, Vector3::new(x[0], x[1], x[2]), side))
                        } else { None };
                        face1_op = if x[d] < (CHUNK_SIZE as i16 - 1) {
                            Some(VoxelFace::new(&chunk, Vector3::new(x[0] + q[0], x[1] + q[1], x[2] + q[2]), side))
                        } else { None };

                        /*
                         * Note that we're using the comparison from the `PartialEq` trait which is
                         * implemented for `VoxelFace`, which lets the faces be compared based on any
                         * number of attributes.
                         *
                         * Also, we choose the face to add to the mask depending on whether we're moving
                         * through on a backface or not.`
                         */
                        mask[n] = match (face_op, face1_op) {
                            (Some(face), Some(face1)) if face == face1 => None,
                            _ => if back_face { face1_op } else { face_op }
                        };

                        n+=1;
                        x[u] += 1;
                    }
                    x[v] += 1;
                }

                x[d]+=1;

                // println!("Mask: {:?}", mask);

                /*
                 * Now we generate the mesh for the mask
                 */
                n = 0;

                j = 0;
                while j < CHUNK_HEIGHT {
                    i = 0;
                    while i < CHUNK_SIZE {

                        if let Some(_) = mask[n] {
                            /*
                             * We compute the width
                             */
                            let compute_width = |i, w, mask: &[Option<VoxelFace>; CHUNK_SIZE * CHUNK_HEIGHT]| {
                                match mask[n + w] {
                                    Some(face) if i + w < CHUNK_SIZE && face == mask[n].unwrap() => true,
                                    _ => false,
                                }

                            };

                            w = 1;
                            while compute_width(i, w, &mask) {
                                w+=1;
                            }

                            /*
                             * Then, we compute height
                             */
                            let mut done = false;

                            h = 1;
                            while j + h < CHUNK_HEIGHT {
                                k=0;
                                while k < w {

                                    let compute_height = |h: usize, k: usize, n: usize, mask: &[Option<VoxelFace>; CHUNK_SIZE * CHUNK_HEIGHT]| {
                                        match mask[n + k + h * CHUNK_SIZE] {
                                            Some(face) if face != mask[n].unwrap() => true,
                                            _ => false,
                                        }
                                    };

                                    if compute_height(h, k, n, &mask) {
                                        done = true;
                                        break;
                                    }
                                    k+=1;
                                }

                                if done {
                                    break;
                                }
                                h+=1;
                            }

                            /*
                             * Here we check the `opaque` attribute associated with the material of
                             * the `VoxelFace` to ensure that we don't mesh aby culled faces.
                             */
                            let opaque = true; // mask[n].unwrap().opaque()
                            if opaque {
                                /*
                                 * Add quad
                                 */
                                x[u] = i as i16;
                                x[v] = j as i16;

                                du[0] = 0;
                                du[1] = 0;
                                du[2] = 0;
                                du[u] = w as i16;

                                dv[0] = 0;
                                dv[1] = 0;
                                dv[2] = 0;
                                dv[v] = h as i16;

                                /*
                                 * And here we call the quad function in order to render a merged
                                 * quad in the scene.
                                 *
                                 * We pass mask[n] to the function, which is an instance of `VoxelFace`
                                 * containing the attributes of the face - which allows for variables to
                                 * be passed to shaders - for example lighting values used to create ambient
                                 * occlusion
                                 */
                                mesh.add_quad(
                                    Vector3::new(x[0] as f32, x[1] as f32, x[2] as f32),
                                    Vector3::new((x[0] + du[0]) as f32, (x[1] + du[1]) as f32, (x[2] + du[2]) as f32),
                                    Vector3::new((x[0] + du[0] + dv[0]) as f32, (x[1] + du[1] + dv[1]) as f32, (x[2] + du[2] + dv[2]) as f32),
                                    Vector3::new((x[0] + dv[0]) as f32, (x[1] + dv[1]) as f32, (x[2] + dv[2]) as f32),
                                    w as i32,
                                    h as i32,
                                    &mask[n].unwrap(),
                                    back_face,
                                    chunk.tex_atlas,
                                );
                            }

                            /*
                             * We zero out the mask
                             */
                            l = 0;
                            while l < h {
                                k = 0;
                                while k < w {
                                    mask[n + k + l * CHUNK_SIZE] = None;
                                    k += 1;
                                }
                                l += 1;
                            }

                            /*
                             * And then finally increment the counters and continue
                             */
                            i += w;
                            n += w;

                        } else {
                            i+=1;
                            n+=1;
                        }

                    }
                    j+=1;
                }
            }
        }

        back_face = back_face && b;
        b = !b;
    }

    mesh
}