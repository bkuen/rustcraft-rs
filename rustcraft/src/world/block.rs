use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};
use std::ops::{Deref};

/// Material
///
/// A `Material` represents the type of a block.
/// Internally, each material is stored as `u8`.
pub type Material = u8;

/// Materials
///
/// `Materials` represents some default 'types' of blocks.
/// Internally, each material is stored as `u8`.
///
/// # Safety
///
/// To avoid weird graphical issues, the material ids have to
/// match with those of the `blocks.lua` script.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Materials {
    Air = 0,
    Grass = 1,
    Dirt = 2,
    Stone = 3,
}

impl Into<u8> for Materials {
    fn into(self) -> u8 {
        self as u8
    }
}

/// BlockTexture
///
/// The `BlockTexture` stores the texture indices of the
/// atlas for the top, bottom and side view of a
/// certain block
#[derive(Serialize, Deserialize)]
pub struct BlockTexture {
    /// The index of the top view
    top: u32,
    /// The index of the bottom view
    bottom: u32,
    /// The index of the side view
    side: u32,
}

/// BlockData
///
/// The `BlockData` stores the nature, character and texture
/// of a certain block
#[derive(Clone)]
pub struct BlockData {
    inner: Arc<BlockDataInner>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockDataInner {
    /// The id of the block
    id: u8,
    /// The name of the block
    name: String,
    /// A block could either be `opaque` (`true`) or transparent (`false`)
    opaque: bool,
    /// A block could either be `collidable` (`true`) or even not (`false`)
    collidable: bool,
    /// The texture coordinates for the top, bottom
    /// and side view of the block.
    tex: Option<BlockTexture>,
}

impl Into<BlockData> for BlockDataInner {
    fn into(self) -> BlockData {
        BlockData {
            inner: Arc::new(self),
        }
    }
}

impl Deref for BlockData {
    type Target = BlockDataInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl BlockData {
    /// Returns the id of the block
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns the name of the block
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the texture coordinates for the top, bottom
    /// and side view of the block.
    pub fn tex_coords(&self) -> Option<&BlockTexture> {
        self.tex.as_ref()
    }

    /// Returns `true` if a block is `opaque`
    pub fn opaque(&self) -> bool {
        self.opaque
    }

    /// Returns `true` if a block is `collidable`
    pub fn collidable(&self) -> bool {
        self.collidable
    }
}

/// BlockRegistry
///
/// A block registry stores all block types which are
/// available inside the game. Typically, these block
/// types are read from a `Lua` script.
/// Each block data has its unique id which can be used
/// to access the block data from outside.
#[derive(Default, Clone)]
pub struct BlockRegistry {
    inner: Arc<BlockRegistryInner>
}

#[derive(Default)]
pub struct BlockRegistryInner {
    /// A `Vec` of block data
    blocks: RwLock<Vec<BlockData>>,
}

impl Deref for BlockRegistry {
    type Target = BlockRegistryInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl BlockRegistry {
    /// Registers a new block type
    ///
    /// # Arguments
    ///
    /// * `data` - A `BlockData` struct
    pub fn register_data(&self, data: BlockData) {
        let mut guard = self.blocks.write().unwrap();
        let blocks = &mut *guard;
        blocks.push(data);
    }

    /// Returns the `BlockData` of a given block id
    ///
    /// # Arguments
    ///
    /// `id` - The block id
    pub fn block_data<T: Into<Material> + Copy>(&self, id: T) -> Option<BlockData> {
        let guard = self.blocks.read().unwrap();
        let blocks = &*guard;
        blocks.iter().find(|&data| data.id == id.into()).map(|data| data.clone())
    }

    /// Returns all registered block types
    pub fn blocks(&self) -> Vec<BlockData> {
        let guard = self.blocks.read().unwrap();
        let blocks = &*guard;
        blocks.clone()
    }
}