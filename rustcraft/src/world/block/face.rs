/// BlockFace
///
/// The `BlockFace` represents one side of a cube shaped block,
/// either `Front`, `BACK`, `LEFT`, `RIGHT`, `Top` and `Button`
pub type BlockFace = [f32; 12];

/// The front face of a block
pub const FRONT: BlockFace = [
    0.0, 0.0, 1.0,
    1.0, 0.0, 1.0,
    1.0, 1.0, 1.0,
    0.0, 1.0, 1.0,
];

/// The back face of a block
pub const BACK: BlockFace = [
    1.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    1.0, 1.0, 0.0,
];

/// The right face of a block
pub const RIGHT: BlockFace = [
    1.0, 0.0, 1.0,
    1.0, 0.0, 0.0,
    1.0, 1.0, 0.0,
    1.0, 1.0, 1.0,
];

/// The left face of a block
pub const LEFT: BlockFace = [
    0.0, 0.0, 0.0,
    0.0, 0.0, 1.0,
    0.0, 1.0, 1.0,
    0.0, 1.0, 0.0,
];

/// The top face of a block
pub const TOP: BlockFace = [
    0.0, 1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
];

/// The bottom face of a block
pub const BOTTOM: BlockFace = [
    0.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    1.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
];