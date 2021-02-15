use crate::world::chunk::{Chunk, ChunkRenderer, CHUNK_SIZE};
use crate::graphics::gl::Gl;
use crate::resources::Resources;
use crate::camera::PerspectiveCamera;
use crate::world::terrain_generator::{TerrainGen, SimpleTerrainGen};
use cgmath::Vector2;
use std::thread;
use std::sync::Arc;

pub mod block;
pub mod chunk;
pub mod terrain_generator;

const RENDER_DISTANCE: i32 = 6;

/// World
///
/// The world contains all chunks which
/// are currently loaded from the file
/// system.
///
/// At the moment, chunks are just stored
/// in memory, this will change in upcoming
/// releases.
pub struct World {
    /// An `OpenGL` instance
    gl: Gl,
    /// The chunks of the world which are
    /// currently loaded from the file system
    chunks: Vec<Chunk>,
    /// The chunk renderer which is used to render
    /// the given chunks to the screen
    chunk_renderer: ChunkRenderer,
    /// The terrain generator which is used to generate
    /// loading chunks
    terrain_gen: Arc<Box<dyn TerrainGen + Send + Sync>>,
}

impl World {
    /// Creates a new world
    ///
    /// # Arguments
    ///
    /// * `gl` - An `OpenGl` instance
    /// * `res` - A `Resources` instance
    pub fn new(gl: &Gl, res: &Resources) -> Self {
        Self {
            gl: gl.clone(),
            chunks: Vec::new(),
            chunk_renderer: ChunkRenderer::new(gl, res),
            terrain_gen: Arc::new(Box::new(SimpleTerrainGen::default()) as Box<dyn TerrainGen + Send + Sync>),
        }
    }

    /// Loads a chunk from the file system
    ///
    /// # Arguments
    ///
    /// * `loc` - The location of the chunk which is load from
    /// the file system
    pub fn load_chunk(&mut self, loc: &Vector2<i32>) {
        if self.chunk(loc).is_none() {
            let chunk = Chunk::new(&self.gl, loc.clone());
            self.chunks.push(chunk.clone());

            let loc = loc.clone();
            let terrain_gen = self.terrain_gen.clone();
            thread::spawn(move || {
                let height_map = terrain_gen.gen_heightmap(&loc);
                terrain_gen.gen_smooth_terrain(&chunk, &height_map);
            });
        }
    }

    /// Unloads a chunk. Stores the chunk to the
    /// file system.
    ///
    /// # Arguments
    ///
    /// * `loc` - The location of the chunk which should be unloaded
    pub fn unload_chunk(&mut self, loc: &Vector2<i32>) {
        if let Some(pos) = self.chunks.iter().position(|x| x.loc() == loc) {
            self.chunks.remove(pos);
        }
    }

    /// Clears the renderer before a render call
    pub fn clear_renderer(&self) {
        self.chunk_renderer.clear();
    }

    /// Renders the world with a given camera perspective.
    /// Internally, a "spiral like" loop will be used to render the chunks
    /// around the player.
    ///
    /// At the moment, the render distance is set within the `RENDER_DISTANCE`
    /// constant.
    ///
    /// # Arguments
    ///
    /// * `camera` - A perspective camera
    #[allow(unused_assignments)]
    pub fn render(&mut self, camera: &PerspectiveCamera) {

        self.chunk_renderer.prepare();

        let chunk_x = (camera.pos().x / CHUNK_SIZE as f32).floor();
        let chunk_y = (camera.pos().z / CHUNK_SIZE as f32).floor();

        let distance = (RENDER_DISTANCE * 2) + 3;
        let border = (distance / 2) as f32;

        let (mut x, mut y) = (0.0, 0.0);
        let (mut dx, mut dy) = (0.0, -1.0);

        let mut t = distance as f32;
        for _ in 0..distance*distance {

            if -distance as f32 / 2.0 < x && x <= distance as f32 / 2.0
                && -distance as f32 / 2.0 < y && y <= distance as f32 / 2.0
            {
                let loc = Vector2::new((chunk_x + x) as i32, (chunk_y + y) as i32);

                if x == -border || x == border || y == -border || y == border {
                    self.unload_chunk(&loc);
                    self.chunk_renderer.remove_chunk(&loc);
                } else {
                    self.load_chunk(&loc);
                    self.chunk_renderer.add_chunk(&loc);
                }

                if let Some(chunk) = self.chunk(&loc) {
                    self.chunk_renderer.render_chunk(chunk, &camera);
                }
            }

            if x == y || (x < 0.0 && x == -y) || (x > 0.0 && x == 1.0-y) {
                t = dx;
                dx = -dy;
                dy = t;
            }

            x += dx;
            y += dy;
        }
    }

    /// Returns the chunk at a given location
    ///
    /// # Arguments
    ///
    /// * `loc` - The chunk location
    ///
    /// # Safety
    ///
    /// This function returns `None` if chunk isn't
    /// loaded from the file system or haven't generated
    /// so far.
    pub fn chunk(&self, loc: &Vector2<i32>) -> Option<&Chunk> {
        self.chunks.iter().find(|&chunk| chunk.loc() == loc)
    }

    /// Returns all chunks which are currently
    /// loaded from the file system
    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }
}