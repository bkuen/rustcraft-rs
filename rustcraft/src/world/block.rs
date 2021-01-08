use cgmath::{Vector2};

/// Material
///
/// A `Material` represents the 'type' of a block
/// as just one u8
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    name: &'static str,
    /// The texture coordinates for the top, bottom
    /// and side view of the block.
    tex_coords: BlockTextureCoords,
    /// A block could either be `opaque` (true) or transparent (false)
    opaque: bool,
}

impl BlockData {
    /// Returns the name of the block
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the texture coordinates for the top, bottom
    /// and side view of the block.
    pub fn tex_coords(&self) -> &BlockTextureCoords {
        &self.tex_coords
    }
}