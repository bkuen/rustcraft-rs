use crate::world::chunk::{CHUNK_AREA, Chunk, CHUNK_SIZE, CHUNK_HEIGHT};
use cgmath::{Vector2, Vector3};
use crate::world::block::Materials;
use noise::{Perlin, NoiseFn};
use cgmath::num_traits::FromPrimitive;

/// TerrainGen
///
/// A trait which can be implemented by
/// different terrain generating algorithms.
pub trait TerrainGen {
    /// Generates a heightmap at a given chunk
    /// location
    ///
    /// # Arguments
    ///
    /// * `loc` - The location of the chunk
    fn gen_heightmap(&self, loc: &Vector2<i32>) -> [i32; CHUNK_AREA];

    /// Generates a smooth terrain using the height
    /// map
    ///
    /// # Arguments
    ///
    /// * `chunk` - A mutable instance of a chunk
    /// * `height_map` - The height map which should be applied
    /// to the generator
    fn gen_smooth_terrain(&self, chunk: &Chunk, height_map: &[i32; CHUNK_AREA]);
}

#[derive(Default)]
pub struct SimpleTerrainGen {}

impl TerrainGen for SimpleTerrainGen {
    fn gen_heightmap(&self, loc: &Vector2<i32>) -> [i32; CHUNK_AREA] {
        let cx = loc.x;
        let cy = loc.y;

        let mut height_map = [0i32; CHUNK_AREA];

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                // Get block x and y coordinate
                let block_x = x as f64 + cx as f64 * CHUNK_SIZE as f64;
                let block_y = y as f64 + cy as f64 * CHUNK_SIZE as f64;
                // Get noise value
                let mut value = Perlin::new().get([block_x / 16.0, block_y / 16.0]);

                // Make it between 0.0 and 1.0
                value = (value + 1.0) / 2.0;
                // Make it bigger
                // value *= 5.0 + 32.0;
                value *= 1.0 + 15.0;

                // Set value into height map
                height_map[y * CHUNK_SIZE + x] = i32::from_f64(value).unwrap();
            }
        }

        height_map
    }

    fn gen_smooth_terrain(&self, chunk: &Chunk, height_map: &[i32; CHUNK_AREA]) {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let height = height_map[z * CHUNK_SIZE + x];
                for y in 0..CHUNK_HEIGHT {
                    if y as i32 <= height {
                        chunk.set_block(Vector3::new(x as i16, y as i16, z as i16), Materials::Grass);
                    }
                }
            }
        }
    }
}