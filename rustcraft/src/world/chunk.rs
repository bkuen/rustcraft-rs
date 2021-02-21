use cgmath::{Vector3, Vector2};
use crate::world::block::{Materials, BlockRegistry, Material};
use crate::resources::Resources;
use crate::camera::PerspectiveCamera;
use crate::entity::Entity;
use crate::gl;
use crate::graphics::gl::Gl;
use crate::graphics::mesh::{Mesh, Model};
use crate::graphics::shader::ShaderProgram;
use crate::graphics::texture::{TextureAtlas, Texture, TextureArray};
use std::borrow::{BorrowMut, Borrow};
use std::ops::{Deref};
use crate::graphics::buffer::{VertexBufferLayout, VertexBuffer};
use std::mem::size_of;
use crate::graphics::gl::types::GLvoid;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender, Receiver};

/// The size of each chunk
pub const CHUNK_SIZE:usize = 16;
/// The height of each chunk
pub const CHUNK_HEIGHT:usize = 256;
/// The area of each chunk. Usually, chunks have
/// a squared area.
pub const CHUNK_AREA:usize = CHUNK_SIZE * CHUNK_SIZE;
/// The volume of each chunk
pub const CHUNK_VOLUME:usize = CHUNK_AREA * CHUNK_HEIGHT;

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
#[derive(Clone)]
pub struct Chunk {
    inner: Arc<ChunkInner>,
}

pub struct ChunkInner {
    /// An `OpenGL` instance
    gl: Gl,
    /// The location of the chunk
    loc: Vector2<i32>,
    /// The blocks stored in the chunk
    blocks: Mutex<Box<[Material; CHUNK_VOLUME]>>,
    /// The block registry
    block_registry: BlockRegistry,
    /// The current chunk model
    model: Arc<Mutex<Option<ChunkModel>>>,
    /// A boolean determining whether the chunk model should be recalculated
    recalculate: Arc<Mutex<bool>>,
}

impl Deref for Chunk {
    type Target = ChunkInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for Chunk {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc
    }
}

impl Chunk {
    /// Creates a new chunk.
    /// By default, this chunk is filled with grass blocks.
    ///
    /// # Arguments
    ///
    /// * `gl` - An `OpenGl` instance
    /// * `loc` - The location of the chunk
    pub fn new(gl: &Gl, loc: Vector2<i32>, block_registry: &BlockRegistry) -> Self {
        Self {
            inner: Arc::new(ChunkInner {
                loc,
                gl: gl.clone(),
                blocks: Mutex::new(Box::new([Materials::Air as u8; CHUNK_VOLUME])),
                block_registry: block_registry.clone(),
                model: Arc::new(Mutex::new(None)),
                recalculate: Arc::new(Mutex::new(true)),
            }),
        }
    }

    /// Places a block to the given location
    ///
    /// # Argument
    ///
    /// * `loc` - The location the block should be placed
    /// * `material` - The material of the block
    ///
    /// # Safety
    ///
    /// If the location is out of bounds, the block won't be placed
    pub fn set_block<T: Into<Material>>(&self, loc: Vector3<i16>, material: T) {
        if let Some(index) = self.index_of(loc) {
            {
                let mut guard = self.blocks.lock().unwrap();
                (*guard)[index] = material.into();
            }
            {
                let mut guard = self.recalculate.lock().unwrap();
                *guard = true;
            }
        }
    }

    /// Returns the model of the chunk
    pub fn model(&self) -> Arc<Mutex<Option<ChunkModel>>> {
        self.model.clone()
    }

    /// Returns the location of the chunk
    pub fn loc(&self) -> &Vector2<i32> {
        &self.loc
    }

    /// Returns the block registry
    pub fn block_registry(&self) -> &BlockRegistry {
        &self.block_registry
    }

    // /// Returns all blocks of the chunk as `Iter`
    // pub fn blocks(&self) -> &[Material; CHUNK_VOLUME] {
    //     &*self.blocks
    // }

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
        // println!("X: {}, Y: {}, Z: {}", loc.x, loc.y, loc.z);
        if let Some(index) = self.index_of(loc) {
            let guard = self.blocks.lock().unwrap();
            let blocks = &*guard;
            // println!("Index: {}, Material: {:?}", index, blocks[index]);
            return Some(blocks[index]);
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
        // println!("{:?}", loc);

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
pub struct ChunkModel {
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
pub struct ChunkMesh {
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
        mesh.tex_coords.extend_from_slice(&[
            0.0,          0.0,
            width as f32, 0.0,
            0.0,          height as f32,
            width as f32, height as f32,
        ]);

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
    /// An array of textures
    textures: TextureArray,
    /// A shader program
    shader_program: ShaderProgram,
    /// A map which internally stores the chunk models
    chunk_map: HashMap<Vector2<i32>, Option<ChunkModel>>,
    /// A channel to send/receive chunk mesh updates
    chunk_update_channel: (Sender<(Vector2<i32>, ChunkMesh)>, Receiver<(Vector2<i32>, ChunkMesh)>)
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


        let textures = TextureArray::from_resource(gl, resources, "textures/textures.png", (16, 16), 6);

        Self {
            shader_program,
            tex_atlas,
            textures,
            gl: gl.clone(),
            chunk_map: HashMap::new(),
            chunk_update_channel: channel(),
        }
    }

    /// Add a chunk
    pub fn add_chunk(&mut self, loc: &Vector2<i32>) {
        if !self.chunk_map.contains_key(loc) {
            self.chunk_map.insert(loc.clone(), None);
        }
    }

    /// Remove a chunk
    pub fn remove_chunk(&mut self, loc: &Vector2<i32>) {
        self.chunk_map.remove(loc);
    }

    /// Recalculates a chunk
    ///
    /// # Arguments
    ///
    /// * `chunk` - The chunk which should be recalculated
    pub fn recalculate_chunk(&self, chunk: &Chunk) {
        {
            let mut guard = chunk.recalculate.lock().unwrap();
            *guard = false;
        }
        let chunk = chunk.clone();
        let (tx, _) = &self.chunk_update_channel;
        let sender = tx.clone();
        thread::spawn(move || {
            let mesh = make_greedy_chunk_mesh(&chunk);
            sender.send((chunk.loc.clone(), mesh)).unwrap_or_else(drop);
        });

    }

    /// Prepares the rendering process by reading in some mesh updates
    /// and inserting them into the chunk map
    pub fn prepare(&mut self) {
        let (_, rx) = &self.chunk_update_channel;
        for (loc, mesh) in rx.try_iter() {
            let model = ChunkModel::from_chunk_mesh(&self.gl, &mesh);
            self.chunk_map.insert(loc, Some(model));
        }
    }

    /// Returns the model at a given location or `None`
    /// if the chunk is not loaded
    ///
    /// # Arguments
    ///
    /// * `loc` - The location of the chunk (model)
    fn model(&self, loc: &Vector2<i32>) -> Option<&ChunkModel> {
        if let Some(model) = self.chunk_map.get(loc) {
            model.as_ref()
        } else {
            None
        }
    }

    // /// Renders the scene
    // ///
    // /// # Arguments
    // ///
    // /// * `camera` - A perspective camera
    // pub fn render(&mut self, camera: &PerspectiveCamera) {
    //     let shader_program = self.shader_program.borrow_mut();
    //     shader_program.enable();
    //     shader_program.set_uniform_1i("u_Texture", 0);
    //
    //     self.tex_atlas.bind(None);
    //
    //     for pos in self.chunk_positions.iter() {
    //         let chunk = Chunk::new(&self.gl, Vector2::new(0, 0));
    //         let mesh = make_greedy_chunk_mesh(&chunk);
    //
    //         let chunk_model = ChunkModel::from_chunk_mesh(&self.gl, &mesh);
    //         chunk_model.bind();
    //
    //         // Create a new entity
    //         let ent = Entity::at_pos(Vector3::new(pos.x * CHUNK_SIZE as f32, 0.0, pos.y * CHUNK_SIZE as f32));
    //
    //         // Calculate model view projection matrix
    //         let model = ent.model_matrix();
    //         let view = camera.view_matrix();
    //         let proj = camera.proj_matrix();
    //         let mvp = proj * view * model;
    //         shader_program.set_uniform_mat4f("u_MVP", &mvp);
    //
    //         // `OpenGL` draw call
    //         unsafe {
    //             self.gl.DrawElements(
    //                 gl::TRIANGLES,
    //                 chunk_model.ib().index_count() as i32,
    //                 gl::UNSIGNED_INT,
    //                 std::ptr::null(),
    //             );
    //         }
    //
    //         chunk_model.unbind();
    //     }
    //
    //     self.tex_atlas.unbind();
    //     shader_program.disable();
    //     self.chunk_positions.clear();
    // }

    /// Renders a given chunk
    ///
    /// # Arguments
    ///
    /// * `chunk` - The chunk which should be rendered to the screen
    pub fn render_chunk(&self, chunk: &Chunk, camera: &PerspectiveCamera) {
        let recalculate;
        {
            let guard = chunk.recalculate.lock().unwrap();
            recalculate = *guard;
        }

        if recalculate {
            self.recalculate_chunk(&chunk);
            // chunk.recalculate_model();
        }

        // if let Some(chunk_model) = chunk.model.lock().unwrap().as_ref() {
        if let Some(chunk_model) = self.model(chunk.loc()) {
            let shader_program = self.shader_program.borrow();
            shader_program.enable();

            let texture_unit = 2;


            // shader_program.set_uniform_1i("u_Texture", 0);
            // self.tex_atlas.bind(None);
            shader_program.set_uniform_1i("u_Texture", texture_unit as i32);
            self.textures.bind(Some(texture_unit));
            chunk_model.bind();

            // Create a new entity
            let ent = Entity::at_pos(Vector3::new(
                chunk.loc().x as f32 * CHUNK_SIZE as f32,
                0.0,
                chunk.loc().y as f32 * CHUNK_SIZE as f32
            ));

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
            // self.tex_atlas.unbind();
            self.textures.unbind();
            shader_program.disable();
        }
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
pub struct VoxelFace {
    side: Side,
    material: Material,
}

impl VoxelFace {
    fn new(chunk: &Chunk, loc: Vector3<i16>, side: Side) -> Self {
        Self {
            side,
            material: chunk.block(loc).unwrap_or(Materials::Air as u8),
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
                            let vface = VoxelFace::new(&chunk, Vector3::new(x[0], x[1], x[2]), side);
                            Some(vface)
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
                                if n + w >= mask.len() {
                                    return false;
                                }

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
                                            Some(face) => face != mask[n].unwrap(),
                                            _ => true,
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
                            let block_data = chunk.block_registry().block_data(mask[n].unwrap().material).unwrap();

                            if block_data.opaque() {
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