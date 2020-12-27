//! Types and traits representing entities in the game

use cgmath::{Vector3, Zero, Matrix4};

/// Entity
///
/// An entity represent a game object in the game.
/// For now, it only has a position and a rotation.
pub struct Entity {
    /// The position of the entity
    pos: Vector3<f32>,
    /// the rotation of the entity
    rot: Vector3<f32>,
}

impl Entity {
    /// Creates a new entity at the given position
    /// with the given rotation
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the entity
    /// * `rot` - The rotation of the entity
    pub fn new(pos: Vector3<f32>, rot: Vector3<f32>) -> Self {
        Self {
            pos,
            rot,
        }
    }

    /// Creates a new entity at the given position
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the entity
    pub fn at_pos(pos: Vector3<f32>) -> Self {
        Self::new(pos, Vector3::zero())
    }

    /// Returns the position of the entity
    pub fn pos(&self) -> &Vector3<f32> {
        &self.pos
    }

    /// Returns the rotation of the entity
    pub fn rot(&self) -> &Vector3<f32> {
        &self.pos
    }

    /// Returns the model matrix of the entity
    pub fn model_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.pos().clone())
    }

    /// Returns the rotation matrix of the entity
    pub fn rotation_matrix(&self) -> Matrix4<f32> {
        let sx = self.rot.x.sin();
        let cx = self.rot.x.cos();
        let sy = self.rot.y.sin();
        let cy = self.rot.y.cos();
        let sz = self.rot.z.sin();
        let cz = self.rot.z.cos();

        Matrix4::new( // z
          cz, -sz, 0.0, 0.0,
          sz,  cz, 0.0, 0.0,
          0.0, 0.0, 1.0, 0.0,
          0.0, 0.0, 0.0, 1.0,
        ) * Matrix4::new( // y
          cy, 0.0,  sy, 0.0,
          0.0, 1.0, 0.0, 0.0,
          -sy, 0.0,  cy, 0.0,
          0.0, 0.0, 0.0, 1.0,
        ) * Matrix4::new( // x
          1.0, 0.0, 0.0, 0.0,
          0.0,  cx, -sx, 0.0,
          0.0,  sx,  cx, 0.0,
          0.0, 0.0, 0.0, 1.0,
        )
    }
}